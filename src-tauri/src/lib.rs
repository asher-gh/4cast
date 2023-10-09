use serde::{Deserialize, Serialize};
use std::sync::Mutex;
pub mod forecast;
pub use forecast::{mad_mape, FCData};
pub mod handlers;
pub use handlers::{fetch_data, log, read_csv};

const SMA_WINDOW: usize = 2;

#[derive(Serialize)]
pub struct CustomResp {
    pub dates: Vec<String>,
    pub beds_actual: Vec<u32>,
    pub beds_forecast: Vec<u32>,
    pub mad: f32,
    pub mape: f32,
}

#[derive(Debug, Deserialize, Clone)]
struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Beds(England)")]
    bed_count: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    CSV(#[from] csv::Error),
    #[error(transparent)]
    FheRuntime(#[from] sunscreen::RuntimeError),
    #[error(transparent)]
    Sunscreen(#[from] sunscreen::Error),
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
pub struct AppState(pub Mutex<FCData>);

pub fn vec_to_arr<T>(v: Vec<T>) -> [T; SMA_WINDOW] {
    // TODO: handle edge cases
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!(
            "Expected a Vec of length {} but it was {}",
            SMA_WINDOW,
            v.len()
        )
    })
}
