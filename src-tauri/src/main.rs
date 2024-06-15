// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{alloc, fs};
use std::io;
use serde_json::Value::String;

fn list_cur_dir() -> Result<Vec<alloc::string::String>, ()> {
    let current_dir = std::env::current_dir()?;
    println!("当前目录: {:?}", current_dir);

    // 读取当前目录下的内容
    let entries = fs::read_dir(current_dir)?;

    let mut file_list = vec![];
    // 遍历并打印每个条目
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().ok_or(io::Error::new(io::ErrorKind::Other, "Failed to get file name"))?;
        let file_name_str = file_name.to_string_lossy(); // 转换成字符串
        file_list.push(file_name_str.to_string())
    }

   Ok(file_list)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> Result<Vec<alloc::string::String>, ()> {
    println!("ok");
    let mut r = vec![];
    if name == "ls" {
        r = list_cur_dir()?;
    }
    Ok(r)
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
