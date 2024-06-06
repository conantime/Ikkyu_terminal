use clap::{Error, Result};
use clap::error::ContextValue::None;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use tokio;

use crate::ai_ask::config;
use config::get_config;

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


pub async fn send_message(content: String) -> Result<(), Box<dyn std::error::Error>> {
    // let config = get_config();
    //
    // println!("{:?}", config);
    println!("发送请求中 {}", &content);

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are an AI assistant who knows everything.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "简短用中文列出相关windows命令(只返回最相关命令):".to_string() + &content + "/n并以数组格式提供",
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

    if response.status().is_success() {
        let response_body: ResponseBody = response.json().await?;
        let message = &response_body.choices[0].message.content;
        let primaryCommand = handle_response(message);
        println!("Assistant: {}", message);
    } else {
        eprintln!("Error: {}", response.text().await?);
    }

    Ok(())
}

fn handle_response(message: String) -> Result<(), String> {
    let m = &message;
}