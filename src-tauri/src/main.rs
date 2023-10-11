// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

use std::sync::Mutex;

use app::{
    fetch_data,
    forecast::FheProgramState,
    handlers::{__cmd__fetch_data, __cmd__read_csv},
    read_csv, AppState,
};

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            fc_data: Default::default(),
            fhe_runtime: Mutex::new(FheProgramState::new(None)),
        })
        .invoke_handler(tauri::generate_handler![read_csv, fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
