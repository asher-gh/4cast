// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
use chrono::{Datelike, Days, NaiveDate};
use csv::Reader;
use rand::prelude::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{sync::Mutex, thread, time::Duration};
use tauri::State;

#[derive(Debug, Clone)]
struct ChartData {
    x: Vec<String>,
    y: Vec<f64>,
    now: NaiveDate,
    bed_actual: Vec<Record>,
}

#[derive(Serialize)]
struct CustomResp {
    x: Vec<String>,
    y: Vec<f64>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    CSV(#[from] csv::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
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

        ChartData {
            x,
            y,
            now,
            bed_actual: vec![],
        }
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
        _ if shift > 0 && l > shift => (
            (*data.x)[l - shift..].to_vec(),
            (*data.y)[l - shift..].to_vec(),
        ),
        _ => ((*data.x).to_vec(), (*data.y).to_vec()),
    };
    CustomResp { x, y }
}

#[derive(Debug, Deserialize, Clone)]
struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Beds(England)")]
    bed_count: u32,
}

#[tauri::command]
async fn read_csv(state: State<'_, AppState>, csv_path: String) -> Result<(), Error> {
    // TODO: progress indicator
    let mut rdr = Reader::from_path(csv_path)?;
    thread::sleep(Duration::from_secs(3));
    let mut buffer = vec![];
    for res in rdr.deserialize() {
        let record: Record = res?;
        println!("{:?}", record);
        buffer.push(record);
    }
    let mut data = state.0.lock().unwrap();
    data.bed_actual = buffer;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(ChartData::new())))
        .invoke_handler(tauri::generate_handler![add_data, log, read_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
