extern crate ndarray;

use std::error::Error;
use ndarray::Array2;
use crate::indicators::data_loader::select_candle_field;

#[derive(Debug)]
pub struct AC {
    pub osc: Vec<f64>,
    pub change: Vec<f64>,
}
#[inline]
pub fn calculate_acosc(candles: &Array2<f64>) -> Result<AC, Box<dyn Error>> {
    let high_prices = select_candle_field(candles, "high")?;
    let low_prices = select_candle_field(candles, "low")?;

    let len = low_prices.len();
    if len < 39 {
        return Err("Not enough data points to calculate AC oscillator".into());
    }

    let mut osc = Vec::with_capacity(len - 38);
    let mut change = Vec::with_capacity(len - 38);

    let mut queue_sma5 = [0.0; 5];
    let mut queue_sma34 = [0.0; 34];
    let mut queue_sma5_ao = [0.0; 5];

    let mut sum_sma5 = 0.0;
    let mut sum_sma34 = 0.0;
    let mut sum_sma5_ao = 0.0;

    for i in 0..34 {
        let medprice = (high_prices[i] + low_prices[i]) * 0.5;
        sum_sma34 += medprice;
        queue_sma34[i] = medprice;
        if i < 5 {
            sum_sma5 += medprice;
            queue_sma5[i] = medprice;
        }
    }

    let mut prev_res = 0.0;
    let mut is_first = true;

    for i in 34..len {
        let medprice = (high_prices[i] + low_prices[i]) * 0.5;

        sum_sma5 += medprice - queue_sma5[i % 5];
        queue_sma5[i % 5] = medprice;
        let sma5 = sum_sma5 * 0.2;

        sum_sma34 += medprice - queue_sma34[i % 34];
        queue_sma34[i % 34] = medprice;
        let sma34 = sum_sma34 / 34.0;

        let ao = sma5 - sma34;

        if i < 39 {
            sum_sma5_ao += ao;
            queue_sma5_ao[i - 34] = ao;
            continue;
        }

        sum_sma5_ao += ao - queue_sma5_ao[i % 5];
        queue_sma5_ao[i % 5] = ao;
        let sma5_ao = sum_sma5_ao * 0.2;

        let res = ao - sma5_ao;

        let mom = if is_first {
            is_first = false;
            0.0
        } else {
            res - prev_res
        };

        prev_res = res;

        osc.push(res);
        change.push(mom);
    }

    Ok(AC { osc, change })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::TEST_CANDLES;

    #[test]
    fn test_acosc_accuracy() {
        let candles = TEST_CANDLES.lock().unwrap();
        let acosc_result: AC = calculate_acosc(&candles).expect("Failed to calculate acosc");
        let expected_last_five_acosc_osc = vec![
            273.30,
            383.72,
            357.7,
            291.25,
            176.84,
        ];
        let expected_last_five_acosc_change = vec![
            49.6,
            110.4,
            -26.0,
            -66.5,
            -114.4,
        ];

        assert!(
            acosc_result.osc.len() >= 5,
            "Not enough acosc osc values for the test"
        );
        assert!(
            acosc_result.change.len() >= 5,
            "Not enough acosc change values for the test"
        );

        let start_index_osc = acosc_result.osc.len().saturating_sub(5);
        let result_last_five_acosc_osc = &acosc_result.osc[start_index_osc..];

        let start_index_change = acosc_result.change.len().saturating_sub(5);
        let result_last_five_acosc_change = &acosc_result.change[start_index_change..];

        for (i, &value) in result_last_five_acosc_osc.iter().enumerate() {
            assert!(
                (value - expected_last_five_acosc_osc[i]).abs() < 1e-1,
                "acosc osc value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five_acosc_osc[i],
                value
            );
        }

        for (i, &value) in result_last_five_acosc_change.iter().enumerate() {
            assert!(
                (value - expected_last_five_acosc_change[i]).abs() < 1e-1,
                "acosc change value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five_acosc_change[i],
                value
            );
        }
    }
}