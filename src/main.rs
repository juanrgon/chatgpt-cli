use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::process::exit;

#[derive(Serialize)]
struct OpenAIRequest<'a> {
    model: &'a str,
    messages: Vec<OpenAIMessage<'a>>,
}

#[derive(Serialize)]
struct OpenAIMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
}

#[derive(Deserialize)]
struct OpenAIResponseMessage {
    content: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = args[1..].join(" ");

    if prompt.is_empty() {
        eprintln!("Error: missing prompt for OpenAI API.");
        exit(1);
    }

    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY environment variable is not set.");
            exit(1);
        }
    };

    let request = OpenAIRequest {
        model: "gpt-3.5-turbo",
        messages: vec![OpenAIMessage {
            role: "user",
            content: prompt.as_str(),
        }],
    };

    let client = Client::new();

    // Send the request to the OpenAI API
    // and deserialize the response into a struct.
    // If the request fails, the program will panic and print the response to the console.
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .unwrap();

    let response_body = response.text().unwrap();

    // Deserialize the response into a struct. If the deserialization fails, print the response to the console.
    let openai_response: OpenAIResponse = match serde_json::from_str(&response_body) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("{}", response_body);
            exit(1);
        }
    };

    println!("{}", openai_response.choices[0].message.content);
}
