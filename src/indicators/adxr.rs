use crate::indicators::data_loader::Candles;
use std::error::Error;

#[inline(always)]
pub fn calculate_adxr(candles: &Candles, period: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    let high = candles.select_candle_field("high")?;
    let low = candles.select_candle_field("low")?;
    let close = candles.select_candle_field("close")?;

    let len = close.len();
    if period == 0 || period > len {
        return Err("Invalid period specified for adxr calculation.".into());
    }

    if len < period + 1 {
        return Err("Not enough data points to calculate adxr.".into());
    }

    let period_f64 = period as f64;
    let rp = 1.0 / period_f64;

    // Initial sums for TR, plusDM, minusDM over the first `period` bars
    let mut tr_sum = 0.0;
    let mut plus_dm_sum = 0.0;
    let mut minus_dm_sum = 0.0;

    for i in 1..=period {
        let prev_close = close[i - 1];
        let current_high = high[i];
        let current_low = low[i];

        let tr = (current_high - current_low)
            .max((current_high - prev_close).abs())
            .max((current_low - prev_close).abs());

        let up_move = current_high - high[i - 1];
        let down_move = low[i - 1] - current_low;

        if up_move > down_move && up_move > 0.0 {
            plus_dm_sum += up_move;
        }
        if down_move > up_move && down_move > 0.0 {
            minus_dm_sum += down_move;
        }

        tr_sum += tr;
    }

    let mut atr = tr_sum;
    let mut plus_dm_smooth = plus_dm_sum;
    let mut minus_dm_smooth = minus_dm_sum;

    let mut plus_di_prev = if atr != 0.0 { (plus_dm_smooth / atr) * 100.0 } else { 0.0 };
    let mut minus_di_prev = if atr != 0.0 { (minus_dm_smooth / atr) * 100.0 } else { 0.0 };

    let initial_dx = if plus_di_prev + minus_di_prev != 0.0 {
        ((plus_di_prev - minus_di_prev).abs() / (plus_di_prev + minus_di_prev)) * 100.0
    } else {
        0.0
    };

    // We'll store all ADX values after we have them
    // First we must accumulate dx values until we have `period` of them.
    let mut dx_sum = initial_dx;
    let mut dx_count = 1;
    let mut adx = Vec::with_capacity(len - period);

    // Compute DX and then ADX
    for i in (period + 1)..len {
        let prev_close = close[i - 1];
        let current_high = high[i];
        let current_low = low[i];

        let tr = (current_high - current_low)
            .max((current_high - prev_close).abs())
            .max((current_low - prev_close).abs());

        let up_move = current_high - high[i - 1];
        let down_move = low[i - 1] - current_low;

        let plus_dm = if up_move > down_move && up_move > 0.0 { up_move } else { 0.0 };
        let minus_dm = if down_move > up_move && down_move > 0.0 { down_move } else { 0.0 };

        atr = atr - (atr * rp) + tr;
        plus_dm_smooth = plus_dm_smooth - (plus_dm_smooth * rp) + plus_dm;
        minus_dm_smooth = minus_dm_smooth - (minus_dm_smooth * rp) + minus_dm;

        let plus_di_current = if atr != 0.0 { (plus_dm_smooth / atr) * 100.0 } else { 0.0 };
        let minus_di_current = if atr != 0.0 { (minus_dm_smooth / atr) * 100.0 } else { 0.0 };

        let dx = if plus_di_current + minus_di_current != 0.0 {
            ((plus_di_current - minus_di_current).abs() / (plus_di_current + minus_di_current)) * 100.0
        } else {
            0.0
        };

        if dx_count < period {
            dx_sum += dx;
            dx_count += 1;

            if dx_count == period {
                // Compute first ADX
                let first_adx = dx_sum * rp;
                adx.push(first_adx);
            }
        } else {
            // Subsequent ADX values are smoothed:
            let previous_adx = *adx.last().unwrap();
            let adx_current = ((previous_adx * (period_f64 - 1.0)) + dx) * rp;
            adx.push(adx_current);
        }
    }

    // Now compute ADXR:
    // ADXR[i] = (ADX[i] + ADX[i - period]) / 2
    // We have `adx` length = (len - period), so ADXR length = (len - 2*period)
    let mut adxr = Vec::with_capacity(adx.len() - period);
    for i in period..adx.len() {
        adxr.push((adx[i] + adx[i - period]) / 2.0);
    }

    Ok(adxr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::load_test_candles;

    #[test]
    fn test_adxr_accuracy() {
        let candles = load_test_candles().expect("Failed to load test candles");
        let period: usize = 14;
        let ad_result: Vec<f64> = calculate_adxr(&candles,period).expect("Failed to calculate adxr");

        let expected_last_five_ad = vec![
            37.10,
            37.3,
            37.0,
            36.2,
            36.3,
        ];

        assert!(
            ad_result.len() >= 5,
            "Not enough adxr values for the test"
        );

        let start_index = ad_result.len() - 5;
        let result_last_five_ad = &ad_result[start_index..];

        for (i, &value) in result_last_five_ad.iter().enumerate() {
            let expected_value = expected_last_five_ad[i];
            assert!(
                (value - expected_value).abs() < 1e-1,
                "adxr value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }
    }
}
