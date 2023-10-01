use clap::Parser;
use dirs;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use rustix::process;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::time::Duration;
use std::{
    env,
    fs::{self},
    io::Read,
};
use sys_info::boottime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    // Load the config from the environment
    let config = Config::from_env().expect("Failed to get API config");

    // get the prompt from the user
    let prompt = args.prompt.join(" ");

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

    // get the messages from the chatlog. limit the total number of tokens
    let messages = get_messages_from_chatlog(&chatlog_text, 4096)?;

    // send the POST request to OpenAI
    let client = Client::new();
    let data = OpenAIRequest {
        model: config.model.to_string(),
        messages,
    };
    let response = send_request_to_openai(&client, data, &config)?;

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

    let mut chatlog = vec![];

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

// get version from Cargo.toml
#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Juan Gonzalez <jrg2156@gmail.com>")]
struct CliArgs {
    /// The prompt to send to ChatGPT
    #[clap(name = "prompt")]
    prompt: Vec<String>,

    /// The ChatGPT model to use (default: gpt-3.5-turbo)
    #[clap(short, long)]
    model: Option<String>,
}

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

struct Config {
    api_key: String,
    model: String,
    timeout: u64,
}

impl Config {
    fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            api_key: env::var("OPENAI_API_KEY")?,
            model: env::var("CHATGPT_CLI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
            timeout: env::var("CHATGPT_CLI_REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(120),
        })
    }
}

fn get_messages_from_chatlog(chatlog_text: &str, max_tokens: i64) -> Result<Vec<Message>, serde_json::Error> {
    let mut total_tokens = 0;
    let mut messages = vec![];
    if !chatlog_text.is_empty() {
        let chatlog: Vec<Log> = serde_json::from_str(chatlog_text)?;
        for log in chatlog.iter().rev() {
            if total_tokens + log.tokens > max_tokens {
                continue;
            }
            total_tokens += log.tokens;
            messages.push(Message {
                role: log.role.clone(),
                content: log.content.clone(),
            });
        }
    }
    Ok(messages)
}


fn send_request_to_openai(client: &Client, data: OpenAIRequest, config: &Config) -> Result<serde_json::Value, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", config.api_key).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let json_data = serde_json::to_string(&data).unwrap();
    client.post("https://api.openai.com/v1/chat/completions")
        .timeout(Duration::from_secs(config.timeout))
        .headers(headers)
        .body(json_data)
        .send()?
        .json()
}
