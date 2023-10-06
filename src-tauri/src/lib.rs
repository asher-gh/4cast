pub fn mad_mape(actual: &[u32], forecast: &[u32]) -> (f32, f32) {
    let (mut mad, mut mape, l) = (0_u32, 0_u32, actual.len() as f32);

    for (act, fore) in actual.iter().zip(forecast) {
        let diff = act.abs_diff(*fore);
        mad += diff;
        mape += ((diff as f32 / *act as f32) * 10000.0) as u32 ;
    }

    (mad as f32 / l, mape as f32/ (l*100.))
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
