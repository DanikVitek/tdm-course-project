#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use course_project::command;
use tauri_plugin_log::{LogTarget, LoggerBuilder};

fn main() {
    tauri::Builder::default()
        .plugin(
            LoggerBuilder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![command::compute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
