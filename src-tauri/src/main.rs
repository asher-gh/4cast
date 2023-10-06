// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod chart;
mod handlers;
use crate::handlers::{fetch_data, log, read_csv};
use app::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![log, read_csv, fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
