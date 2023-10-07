use std::fs::File;

use csv::Reader;
use simple_moving_average::{SumTreeSMA, SMA};

use crate::{Error, Record, SMA_WINDOW};

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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_mad_mape() {
        let actual = Rc::new([90, 105, 110, 80]);
        let forecast = Rc::new([95, 100, 120, 85]);

        const E: f32 = 0.000001;
        let (mad, mape) = mad_mape(&*actual, &*forecast);

        assert!((mad - 6.25).abs() < E);
        assert!((mape - 6.4125).abs() < E);
    }
}
