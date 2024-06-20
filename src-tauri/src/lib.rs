mod invoke_handler;
mod ai;
mod config;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,
            invoke_handler::get_pwd,
            invoke_handler::ssh,
            invoke_handler::execute_ssh_command,
            invoke_handler::ai_mod
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
