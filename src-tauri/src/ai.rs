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
    prompt: String,
    stream: bool
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
    response: String,
    model: String,
    done: bool,
    done_reason: String,
}


pub async fn send_message(content: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("发送请求中 {}", &content);

    // let messages = vec![
    //     Message {
    //         role: "system".to_string(),
    //         content: "You are an AI assistant who knows everything.".to_string(),
    //     },
    //     Message {
    //         role: "user".to_string(),
    //         content
    //     },
    // ];

    let request_body = RequestBody {
        model: "qwen:7b".to_string(),
        prompt: content,
        stream: false
    };

    let client = reqwest::Client::new();

    // let mut headers = HeaderMap::new();
    // headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", "sk-0J6lwoWGDs9koD4MJegiFwKq5dgdePeJmgupjP7eMTIggNhD"))?);

    // let response = client
    //     .post("https://api.chatanywhere.tech/v1/chat/completions")
    //     .headers(headers)
    //     .json(&request_body)
    //     .send()
    //     .await?;

    let response = client
        .post("http://127.0.0.1:11434/api/generate")
        .json(&request_body)
        .send()
        .await?;

    let mut primary_command = "".to_string();

    if response.status().is_success() {
        let response_body: ResponseBody = response.json().await?;
        println!("{:?}", response_body);
        let mes = &response_body.response;
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