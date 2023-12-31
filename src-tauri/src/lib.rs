use forecast::FheProgramState;
use serde::{Deserialize, Serialize};
use std::{fs::File, sync::Mutex};
pub mod forecast;
pub use forecast::{mad_mape, FCData};
pub mod handlers;
pub use handlers::{fetch_data, read_csv};

pub const SMA_WINDOW: usize = 2;

#[derive(Serialize)]
pub struct CustomResp {
    pub dates: Vec<String>,
    pub beds_actual: Vec<u32>,
    pub beds_forecast: Vec<u32>,
    pub mad: f32,
    pub mape: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Beds(England)")]
    pub bed_count: u32,
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

pub struct AppState {
    pub fc_data: Mutex<FCData>,
    pub fhe_runtime: Mutex<FheProgramState<File>>,
}

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

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    #[test]
    fn rayon_mean() {
        let arr: Vec<u32> = (10..20).into_iter().collect();
        let exp: Vec<u32> = arr.iter().map(|x| x * 2).collect();

        // use rayon to parallel map the arr and check for sequence
        let t_arr: Vec<u32> = arr.par_iter().map(|x| x * 2).collect();

        t_arr.into_iter().zip(exp.into_iter()).for_each(|(t, e)| {
            assert_eq!(t, e);
        });
    }
}
