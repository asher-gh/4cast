// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

pub mod chart;

use app::mad_mape;
use crate::chart::ChartData;
use csv::Reader;
use serde::{Deserialize, Serialize};
use simple_moving_average::{SumTreeSMA, SMA};
use std::sync::Mutex;
use tauri::State;

const SMA_WINDOW: usize = 2;

#[derive(Serialize)]
struct CustomResp {
    dates: Vec<String>,
    beds_actual: Vec<u32>,
    beds_forecast: Vec<u32>,
    mad: f32,
    mape: f32,
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

#[derive(Debug)]
pub struct AppState(Mutex<ChartData>);

#[tauri::command]
fn log(state: tauri::State<AppState>) {
    dbg!(state);
}

#[tauri::command]
fn fetch_data(state: State<AppState>, shift: usize) -> CustomResp {
    let data = state.0.lock().unwrap();
    // data.add_data();
    let l = data.dates.len();
    let (dates, beds_actual, beds_forecast ) = match shift {
        _ if shift > 0 && l > shift => (
            (*data.dates)[l - shift..].to_vec(),
            (*data.beds_actual)[l - shift..].to_vec(),
            (*data.beds_forecast)[l - shift..].to_vec(),
        ),
        _ => (
            (*data.dates).to_vec(),
            (*data.beds_actual).to_vec(),
            (*data.beds_forecast).to_vec(),
        ),
    };

    let (mad, mape) = mad_mape(&beds_actual, &beds_forecast);

    CustomResp {
        dates,
        beds_actual,
        beds_forecast,
        mad,
        mape,
    }
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
    let mut dates = vec![];
    let mut beds_actual = vec![];
    let mut beds_sma = SumTreeSMA::<_, f64, SMA_WINDOW>::new();
    let mut beds_forecast = vec![];
    for res in rdr.deserialize() {
        let record: Record = res?;
        dates.push(record.date);
        beds_actual.push(record.bed_count);
        beds_sma.add_sample(record.bed_count as f64);
        let forecasted = beds_sma.get_average().ceil() as u32;
        beds_forecast.push(forecasted);
    }


    let mut data = state.0.lock().unwrap();
    *data = ChartData {
        dates,
        beds_actual,
        beds_forecast,
    };
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![log, read_csv, fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
