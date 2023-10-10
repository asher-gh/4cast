use crate::{
    forecast::{enc_sma, EncSMAInput},
    mad_mape, AppState, CustomResp, Error,
};
use csv::Reader;
use std::time::Instant;
use tauri::State;
#[tauri::command]
pub fn log(state: tauri::State<AppState>) {
    dbg!(state);
}

#[tauri::command]
pub fn fetch_data(state: State<AppState>, shift: usize) -> CustomResp {
    let data = state.0.lock().unwrap();
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

#[tauri::command]
pub async fn read_csv(state: State<'_, AppState>, csv_path: String) -> Result<(), Error> {
    // TODO: progress indicator
    // TODO: Validate data when reading CSV
    let rdr = Reader::from_path(csv_path)?;
    // let rdr = Reader::from_reader(bed_data.as_bytes());
    let mut data = state.0.lock().unwrap();
    let start = Instant::now();
    let mut input = EncSMAInput::new(rdr);
    *data = enc_sma(&mut input)?;
    dbg!(start.elapsed().as_millis());
    Ok(())
}
