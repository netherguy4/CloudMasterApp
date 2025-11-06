// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_window_state::Builder as WindowStateBuilder;

mod instances;
mod utils;
use crate::instances::{instance_get, instance_start, instance_stop, instances_list};

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .plugin(WindowStateBuilder::default().build())
        .invoke_handler(tauri::generate_handler![
            instance_get,
            instance_start,
            instance_stop,
            instances_list
        ])
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
