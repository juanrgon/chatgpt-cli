use dirs;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use std::time::Duration;
use rustix::process;
use serde::{Deserialize, Serialize};
use sys_info::boottime;
use std::fs::OpenOptions;
use std::{
    env,
    fs::{self},
    io::{Error, Read},
};


#[derive(Serialize, Deserialize, Debug)]
struct Log {
    role: String,
    content: String,
    tokens: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct OpenAIRequest {
    #[serde(rename = "model")]
    model: String,
    #[serde(rename = "messages")]
    messages: Vec<Message>,
}

fn main() -> Result<(), Error> {
    // get OPENAI_API_KEY from environment variable
    let key = "OPENAI_API_KEY";
    let openai_api_key = env::var(key).expect(&format!("{} not set", key));

    // get the prompt from the user
    let args: Vec<String> = env::args().skip(1).collect();
    let prompt = args.join(" ");

    // Get the boottime of the system
    let boot_time = boottime().expect("Unable to get boot time");
    let boot_time_since_unix_epoch = boot_time.tv_sec;

    // load the chatlog for this terminal window
    let chatlog_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".chatgpt")
        .join(boot_time_since_unix_epoch.to_string())
        .join(
            process::getppid()
                .expect("Failed to get parent process id")
                .as_raw_nonzero()
                .to_string(),
        )
        .join("chatlog.json");

    fs::create_dir_all(chatlog_path.parent().unwrap())?;

    let mut file = OpenOptions::new()
        .create(true) // create the file if it doesn't exist
        .append(true) // don't overwrite the contents
        .read(true)
        .open(&chatlog_path)
        .unwrap();

    let mut chatlog_text = String::new();
    file.read_to_string(&mut chatlog_text)?;

    // get the messages from the chatlog. limit the total number of tokens to 3000
    const MAX_TOKENS: i64 = 2000;
    let mut total_tokens: i64 = 0;
    let mut messages: Vec<Message> = vec![];
    let mut chatlog: Vec<Log> = vec![];

    if !chatlog_text.is_empty() {
        chatlog = serde_json::from_str(&chatlog_text)?;
        for log in chatlog.iter().rev() {
            if total_tokens + log.tokens > MAX_TOKENS {
                continue;
            }

            total_tokens += log.tokens;
            messages.push(Message {
                role: log.role.clone(),
                content: log.content.clone(),
            });
        }
    }

    messages = messages.into_iter().rev().collect();

    messages.push(Message {
        role: "user".to_string(),
        content: prompt.clone(),
    });

    // send the POST request to OpenAI
    let client = Client::new();
    let model = env::var("CHATGPT_CLI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
    let data = OpenAIRequest {
        model: model.to_string(),
        messages,
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", openai_api_key).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let json_data = serde_json::to_string(&data)?;
    let timeout_secs = env::var("CHATGPT_REQUEST_TIMEOUT_SECS")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(10); // default value of 10 seconds
    let response = client
        .post("https://api.openai.com/v1/chat/completions".to_string())
        .timeout(Duration::from_secs(timeout_secs))
        .headers(headers)
        .body(json_data)
        .send()
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();

    // if the response is an error, print it and exit
    match response["error"].as_object() {
        None => response["error"].clone(),
        Some(_) => {
            println!(
                "Received an error from OpenAI: {}",
                response["error"]["message"].as_str().unwrap()
            );
            return Ok(());
        }
    };

    let prompt_tokens = response["usage"]["prompt_tokens"].as_i64().unwrap();
    let answer_tokens = response["usage"]["completion_tokens"].as_i64().unwrap();
    let answer = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();

    // Show the response from OpenAI
    println!("{}", answer);

    // save the new messages to the chatlog
    chatlog.push(Log {
        role: "user".to_string(),
        content: prompt,
        tokens: prompt_tokens,
    });
    chatlog.push(Log {
        role: "assistant".to_string(),
        content: answer.to_string(),
        tokens: answer_tokens,
    });

    // write the chatlog to disk
    let chatlog_text = serde_json::to_string(&chatlog)?;
    fs::write(&chatlog_path, chatlog_text)?;

    Ok(())
}
