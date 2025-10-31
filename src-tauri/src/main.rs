// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{command};
use std::process::{Command};

#[command]
fn gcloud_init() -> Result<String, String> {
    let gcloud_path = "C:/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin/gcloud.cmd";

    // Spawn gcloud init interactively
    let output = Command::new(gcloud_path)
        .args(["auth", "application-default", "login"])
        .env("CLOUDSDK_CONFIG", "cfg/gcloud")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Authorized".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![gcloud_init])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
