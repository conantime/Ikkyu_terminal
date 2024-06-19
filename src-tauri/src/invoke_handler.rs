#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_assignments)] // 禁用未使用赋值的警告

use std::env::current_dir;
use std::fmt::format;
use ssh2::{Channel, Session};
use std::path::Path;
use std::io::{Read, Write};
use std::ptr::null;
use tauri::Manager;
use std::sync::Mutex;

static SSH_SESSION: Mutex<Option<Session>> = Mutex::new(None);


#[tauri::command]
pub fn get_pwd() -> String {
    println!("{:?}", current_dir());
    format!("{:?}", current_dir().unwrap())
}

#[tauri::command]
pub fn ssh(ip_with_port: &str, username: &str, password: &str, local_file_path: &str, target_file_path: &str, command: &str) -> String {
    publish(ip_with_port, username, password, local_file_path, target_file_path, command)
}

// #[tauri::command]
// fn event_emit(name: &str, info: String) -> String {
//     let msg = format!("Hello, {}! You've been greeted from Rust!", &name);
//     Manager::app_handle.emit_all(name, &msg).unwrap();
//     println!("{}", msg);
//     format!("{}", msg)
// }

fn publish(ip_with_port: &str, username: &str, password: &str, local_file_path: &str, target_file_path: &str, command: &str) -> String {
    // 连接远程服务器
    let tcp = std::net::TcpStream::connect(ip_with_port); // 连接到远程服务器
    let mut response_txt = "".to_string();
    match tcp {
        Ok(value) => {
            let mut sess = Session::new().unwrap(); // 创建一个新的会话
            sess.set_tcp_stream(value); // 设置会话的 TCP 流
            sess.handshake().unwrap(); // 进行握手
            sess.userauth_password(username, password).unwrap(); // 使用用户名和密码进行身份验证

            // 执行远程命令
            let mut channel = sess.channel_session().unwrap(); // 创建一个新的会话通道
            channel.exec(command).unwrap(); // 执行命令
            let mut output = Vec::new(); // 创建一个空的字节向量
            channel.read_to_end(&mut output).unwrap(); // 读取命令输出
            println!("{}", String::from_utf8_lossy(&output)); // 打印命令输出
            response_txt = String::from_utf8_lossy(&output).to_string();
            let mut ssh_session = SSH_SESSION.lock().unwrap();
            *ssh_session = Some(sess);

        },
        Err(e) => {
            println!("{:?}", e.to_string());
            response_txt = "".to_string();
        }
    }
    response_txt
}

#[tauri::command]
pub fn execute_ssh_command(command: &str) -> String {
    let mut ssh_session = SSH_SESSION.lock().unwrap();
    if let Some(ref mut session) = *ssh_session {
        let mut channel = session.channel_session().unwrap(); // 创建一个新的会话通道
        channel.exec(command).unwrap(); // 执行命令
        let mut output = Vec::new(); // 创建一个空的字节向量
        channel.read_to_end(&mut output).unwrap(); // 读取命令输出
        println!("{}", String::from_utf8_lossy(&output)); // 打印命令输出
        String::from_utf8_lossy(&output).to_string()
    } else {
        String::from("No active SSH channel.")
    }
}


// 获取文件大小
fn get_file_size(file_path: &str) -> u64 {
    std::fs::metadata(file_path) // 获取文件元数据
        .map(|metadata| metadata.len()) // 获取文件大小
        .unwrap_or(0) // 如果获取失败，则返回 0
}