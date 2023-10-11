use crate::{forecast::enc_sma, mad_mape, AppState, CustomResp, Error};
use csv::Reader;
use std::time::Instant;
use tauri::State;

// FIX: better state logging
//
// #[tauri::command]
// pub fn log(state: tauri::State<AppState>) {
// dbg!(state);
// }

#[tauri::command]
pub fn fetch_data(state: State<AppState>, shift: usize) -> CustomResp {
    let data = state.fc_data.lock().unwrap();
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
    // FIX: Invalid CSV breaks the program
    let start = Instant::now(); // start timer
    let mut input = state.fhe_runtime.lock().unwrap();
    input.rdr = Some(Reader::from_path(csv_path)?);
    let mut data = state.fc_data.lock().unwrap();
    *data = enc_sma(&mut input)?;
    dbg!(start.elapsed().as_millis()); // end timer
    Ok(())
}
