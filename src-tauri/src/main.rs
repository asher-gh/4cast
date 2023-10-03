// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
use chrono::{Datelike, Days, NaiveDate};
use rand::prelude::{thread_rng, Rng};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Clone)]
struct ChartData {
    x: Vec<String>,
    y: Vec<f64>,
    now: NaiveDate,
}

#[derive(Serialize)]
struct CustomResp {
    x: Vec<String>,
    y: Vec<f64>,
}

impl ChartData {
    fn new() -> Self {
        let mut rng = thread_rng();
        let now = NaiveDate::from_ymd_opt(2014, 9, 3).unwrap();
        let y = vec![rng.gen::<f64>() * 150.];
        let x: Vec<String> = vec![format!(
            "{:02}/{:02}/{}",
            now.day(),
            now.month(),
            now.year()
        )];

        ChartData { x, y, now }
    }

    fn add_data(&mut self) {
        let mut rng = thread_rng();
        self.now = self.now.checked_add_days(Days::new(1)).unwrap();
        self.x.push(format!(
            "{:02}/{:02}/{}",
            self.now.day(),
            self.now.month(),
            self.now.year()
        ));
        self.y
            .push(rng.gen_range(-0.4..0.6) + self.y.last().unwrap());
    }
}

#[derive(Debug)]
pub struct AppState(Mutex<ChartData>);

#[tauri::command]
fn log(state: tauri::State<AppState>) {
    dbg!(state);
}

#[tauri::command]
fn add_data(state: State<AppState>, shift: usize) -> CustomResp {
    let mut data = state.0.lock().unwrap();
    data.add_data();
    let l = data.x.len();
    let (x, y) = match shift {
        _ if shift > 0 && l > shift => ((*data.x)[l-shift..].to_vec(), (*data.y)[l-shift..].to_vec()),
        _ => ((*data.x).to_vec(), (*data.y).to_vec()),
    };
    CustomResp { x, y }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(ChartData::new())))
        .invoke_handler(tauri::generate_handler![add_data, log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
