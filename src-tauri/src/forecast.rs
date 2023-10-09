use crate::{vec_to_arr, Error, Record, SMA_WINDOW};
use csv::Reader;
use simple_moving_average::{SumTreeSMA, SMA};
use std::io::Read;
use sunscreen::{
    fhe_program,
    types::{bfv::Fractional, Cipher},
    Compiler, FheProgramInput, FheRuntime,
};
// TODO: refactor: encapsulate encryption setup and key and program storage

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

pub fn sma<R: Read>(mut rdr: Reader<R>) -> Result<FCData, Error> {
    let mut data = FCData::default();
    let mut beds_sma = SumTreeSMA::<f64, f64, SMA_WINDOW>::new();

    for res in rdr.deserialize() {
        let record: Record = res?;
        data.dates.push(record.date);
        data.beds_actual.push(record.bed_count);
        let forecasted = beds_sma.get_average().ceil() as u32;
        data.beds_forecast.push(forecasted);
        beds_sma.add_sample(record.bed_count as f64);
    }
    Ok(data)
}

pub fn naive_sma<R: Read>(mut rdr: Reader<R>) -> Result<FCData, Error> {
    let mut data = FCData::default();
    for res in rdr.deserialize() {
        let rec: Record = res?;
        // dbg!(&rec);
        data.dates.push(rec.date);
        data.beds_actual.push(rec.bed_count);
        let l = data.beds_actual.len();
        data.beds_forecast.push(if l > SMA_WINDOW {
            let x = &data.beds_actual[l - SMA_WINDOW - 1..l - 1];

            let res =
                x.to_vec().into_iter().reduce(|a, c| a + c).unwrap() as f64 / SMA_WINDOW as f64;

            res.ceil() as u32
        } else {
            if let Some(x) = data.beds_actual.last() {
                (*x).clone()
            } else {
                0
            }
        });
    }

    Ok(data)
}

pub fn enc_sma<R: Read>(mut rdr: Reader<R>) -> Result<FCData, Error> {
    // OPTIMIZE: cache sum
    let mut data = FCData::default();
    let app = Compiler::new().fhe_program(fhe_mean).compile()?;
    let runtime = FheRuntime::new(app.params())?;
    let (pub_key, priv_key) = runtime.generate_keys()?;

    for res in rdr.deserialize() {
        let rec: Record = res?;
        // dbg!(&rec);
        data.dates.push(rec.date);
        data.beds_actual.push(rec.bed_count);
        let l = data.beds_actual.len();
        data.beds_forecast.push(if l > SMA_WINDOW {
            let x = &data.beds_actual[l - SMA_WINDOW - 1..l - 1];
            let input = vec_to_arr(
                x.to_vec()
                    .into_iter()
                    .map(|x| Fractional::<64>::from(x as f64))
                    .collect(),
            );

            let args: Vec<FheProgramInput> = vec![runtime.encrypt(input, &pub_key)?.into()];
            let res_enc = runtime.run(app.get_fhe_program(fhe_mean).unwrap(), args, &pub_key)?;

            let res: Fractional<64> = runtime.decrypt(&res_enc[0], &priv_key)?;

            res.ceil() as u32
        } else {
            if let Some(x) = data.beds_actual.last() {
                (*x).clone()
            } else {
                0
            }
        });
    }

    Ok(data)
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

    #[test]
    fn test_enc_sma() {
        let bed_data = "Date,Beds(England)
1-Aug-20,879
2-Aug-20,847
3-Aug-20,842
4-Aug-20,807
5-Aug-20,805
6-Aug-20,756
7-Aug-20,711
8-Aug-20,650
9-Aug-20,611
10-Aug-20,714
11-Aug-20,672
12-Aug-20,642
13-Aug-20,652
14-Aug-20,642
15-Aug-20,630
16-Aug-20,634
17-Aug-20,626
18-Aug-20,597";

        let bed_count = [
            879u32, 847, 842, 807, 805, 756, 711, 650, 611, 714, 672, 642, 652, 642, 630, 634, 626,
            597,
        ];

        let bed_forecast: Vec<u32> = bed_count
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                if i >= SMA_WINDOW {
                    let x = bed_count[(i - SMA_WINDOW)..i]
                        .to_vec()
                        .into_iter()
                        .reduce(|a, x| a + x)
                        .unwrap() as f64
                        / SMA_WINDOW as f64;

                    x.ceil() as u32
                } else {
                    x
                }
            })
            .collect();

        let res = enc_sma(csv::Reader::from_reader(bed_data.as_bytes())).unwrap();
        dbg!(&bed_forecast, &res.beds_forecast);

        for (i, x) in bed_forecast.iter().enumerate() {
            assert_eq!(*x, res.beds_forecast[i]);
        }
    }
}
