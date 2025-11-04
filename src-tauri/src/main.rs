// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod instances;
mod utils;
use crate::instances::{instance_get, instance_start, instance_stop, instances_list};

fn main() {
    tauri::Builder::default()
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
