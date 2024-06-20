use clap::{Error, Result};
use clap::error::ContextValue::None;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use tokio;

use crate::config;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    choices: Vec<Choice>,
}


pub async fn send_message(content: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("发送请求中 {}", &content);

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are an AI assistant who knows everything.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content
        },
    ];

    let request_body = RequestBody {
        model: "gpt-3.5-turbo".to_string(),
        messages,
    };

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", "sk-0J6lwoWGDs9koD4MJegiFwKq5dgdePeJmgupjP7eMTIggNhD"))?);

    let response = client
        .post("https://api.chatanywhere.tech/v1/chat/completions")
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;

    let mut primary_command = "".to_string();

    if response.status().is_success() {
        let response_body: ResponseBody = response.json().await?;
        let mes = &response_body.choices[0].message.content;
        primary_command = handle_response(mes)?;

        // let mut ctx: ClipboardContext = clipboard::ClipboardProvider::new().unwrap();
        //
        // ctx.set_contents(primary_command.to_owned()).unwrap();

        println!("Assistant: {}", primary_command);
    } else {
        eprintln!("Error: {}", response.text().await?);
    }

    Ok(primary_command)
}

fn handle_response(message: &String) -> Result<String, &'static str> {

    Ok(message.to_string())
}