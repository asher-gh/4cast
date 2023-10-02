// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
use std::sync::Mutex;

#[derive(Default, Debug)]
struct ChartData<'a> {
    x: Vec<&'a str>,
    y: Vec<&'a str>,
}

#[derive(Debug)]
pub struct AppState<'a>(Mutex<ChartData<'a>>);

#[tauri::command]
fn log(state: tauri::State<AppState>) {
    dbg!(state);
}

#[tauri::command]
fn add_random(state: tauri::State<AppState>) {
    let mut data = state.0.lock().unwrap();
    data.x.push("hello");
    data.y.push("world");
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![add_random, log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
