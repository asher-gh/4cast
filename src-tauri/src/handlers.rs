use csv::Reader;
use serde::Deserialize;
use simple_moving_average::{SumTreeSMA, SMA};
use tauri::State;

use crate::{mad_mape, AppState, CustomResp, Error};

const SMA_WINDOW: usize = 2;

#[tauri::command]
pub fn log(state: tauri::State<AppState>) {
    dbg!(state);
}

#[tauri::command]
pub fn fetch_data(state: State<AppState>, shift: usize) -> CustomResp {
    let data = state.0.lock().unwrap();
    // data.add_data();
    let l = data.dates.len();
    let (dates, beds_actual, beds_forecast) = match shift {
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
pub async fn read_csv(state: State<'_, AppState>, csv_path: String) -> Result<(), Error> {
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
    *data = crate::ChartData {
        dates,
        beds_actual,
        beds_forecast,
    };
    Ok(())
}
