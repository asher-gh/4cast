use crate::{Error, Record, SMA_WINDOW};
use csv::Reader;
use simple_moving_average::{SumTreeSMA, SMA};
use std::fs::File;
use sunscreen::{
    fhe_program,
    types::{bfv::Fractional, Cipher},
};

#[derive(Debug, Clone, Default)]
pub struct FCData {
    pub dates: Vec<String>,
    pub beds_actual: Vec<u32>,
    pub beds_forecast: Vec<u32>,
}

pub fn mad_mape(actual: &[u32], forecast: &[u32]) -> (f32, f32) {
    let (mut mad, mut mape, l) = (0_u32, 0_u32, actual.len() as f32);
    for (act, fore) in actual.iter().zip(forecast) {
        let diff = act.abs_diff(*fore);
        mad += diff;
        mape += ((diff as f32 / *act as f32) * 10000.0) as u32;
    }
    (mad as f32 / l, mape as f32 / (l * 100.))
}

pub fn sma(mut rdr: Reader<File>) -> Result<FCData, Error> {
    let mut dates = vec![];
    let mut beds_actual = vec![];
    let mut beds_forecast = vec![];
    let mut beds_sma = SumTreeSMA::<f64, f64, SMA_WINDOW>::new();

    for (_i, res) in rdr.deserialize().into_iter().enumerate() {
        let record: Record = res?;
        dates.push(record.date);
        beds_actual.push(record.bed_count);
        let forecasted = beds_sma.get_average().ceil() as u32;
        beds_forecast.push(forecasted);
        beds_sma.add_sample(record.bed_count as f64);
    }
    Ok(FCData {
        dates,
        beds_actual,
        beds_forecast,
    })
}

#[fhe_program(scheme = "bfv")]
fn fhe_mean(nums: [Cipher<Fractional<64>>; SMA_WINDOW]) -> Cipher<Fractional<64>> {
    let sum = nums.into_iter().reduce(move |a, e| a + e).unwrap();
    sum / (SMA_WINDOW as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_to_arr;
    use std::rc::Rc;
    use sunscreen::{Compiler, FheProgramInput, FheRuntime};

    #[test]
    fn test_mad_mape() {
        let actual = Rc::new([90, 105, 110, 80]);
        let forecast = Rc::new([95, 100, 120, 85]);

        const E: f32 = 0.000001;
        let (mad, mape) = mad_mape(&*actual, &*forecast);

        assert!((mad - 6.25).abs() < E);
        assert!((mape - 6.4125).abs() < E);
    }

    #[test]
    fn test_fhe_mean() {
        // fake data
        let data = vec![100., 150., 200., 250.];
        // setup
        let app = Compiler::new().fhe_program(fhe_mean).compile().unwrap();
        let runtime = FheRuntime::new(app.params()).unwrap();
        let (pub_key, prv_key) = runtime.generate_keys().unwrap();
        // input
        let input = vec_to_arr(
            data[..SMA_WINDOW]
                .into_iter()
                .map(|x| Fractional::<64>::from(*x))
                .collect(),
        );
        // encrypted input
        let args: Vec<FheProgramInput> = vec![runtime.encrypt(input, &pub_key).unwrap().into()];
        // calling encrypted program with encrypted args
        let res_enc = runtime
            .run(app.get_fhe_program(fhe_mean).unwrap(), args, &pub_key)
            .unwrap();
        // decrypting the result
        let res_dec: Fractional<64> = runtime.decrypt(&res_enc[0], &prv_key).unwrap();
        // (100.0 + 150.0) / 2 = 125.0
        let diff: f64 = (res_dec - 125.0).abs();
        let e = 1e-10;
        assert!(diff < e);
    }
}
