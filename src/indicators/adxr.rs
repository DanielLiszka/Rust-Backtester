use crate::indicators::data_loader::Candles;
use std::error::Error;
use std::collections::VecDeque;

#[inline]
pub fn calculate_adxr(candles: &Candles, period: usize) -> Result<Vec<f64>, Box<dyn Error>> {

    let high: &[f64] = candles.select_candle_field("high")?;
    let low: &[f64] = candles.select_candle_field("low")?;
    let close: &[f64] = candles.select_candle_field("close")?;

    let close_len: usize = close.len();
    if period == 0 || period > close_len {
        return Err("Invalid period specified for adxr calculation.".into());
    }

    let len = close_len;
    if len < period + 1 {
        return Err("Not enough data points to calculate adxr.".into());
    }

    let period_f64 = period as f64;
    let reciprocal_period = 1.0 / period_f64;

    let mut tr_sum: f64 = 0.0;
    let mut plus_dm_sum: f64 = 0.0;
    let mut minus_dm_sum: f64 = 0.0;

    for i in 1..=period {
        let current_high = high[i];
        let current_low = low[i];
        let prev_close = close[i - 1];
        let prev_high = high[i - 1];
        let prev_low = low[i - 1];

        let tr = (current_high - current_low)
            .max((current_high - prev_close).abs())
            .max((current_low - prev_close).abs());

        let up_move = current_high - prev_high;
        let down_move = prev_low - current_low;

        plus_dm_sum += if up_move > down_move && up_move > 0.0 { up_move } else { 0.0 };
        minus_dm_sum += if down_move > up_move && down_move > 0.0 { down_move } else { 0.0 };

        tr_sum += tr;
    }

    let mut atr = tr_sum;
    let mut plus_dm_smooth = plus_dm_sum;
    let mut minus_dm_smooth = minus_dm_sum;

    let mut plus_di_prev = if atr != 0.0 { (plus_dm_smooth / atr) * 100.0 } else { 0.0 };
    let mut minus_di_prev = if atr != 0.0 { (minus_dm_smooth / atr) * 100.0 } else { 0.0 };

    let initial_dx: f64 = if (plus_di_prev + minus_di_prev) != 0.0 {
        ((plus_di_prev - minus_di_prev).abs() / (plus_di_prev + minus_di_prev)) * 100.0
    } else {
        0.0
    };
    let mut dx_sum = initial_dx;
    let mut dx_count = 1;

    let mut adx = Vec::with_capacity(len - period);
    let mut adxr = Vec::with_capacity(len - period - period);
    let mut adx_buffer: VecDeque<f64> = VecDeque::with_capacity(period);

    for i in (period + 1)..len {
        let current_high: f64 = high[i];
        let current_low: f64 = low[i];
        let prev_close: f64 = close[i - 1];
        let prev_high: f64 = high[i - 1];
        let prev_low: f64 = low[i - 1];

        let tr: f64 = (current_high - current_low)
            .max((current_high - prev_close).abs())
            .max((current_low - prev_close).abs());

        let up_move = current_high - prev_high;
        let down_move = prev_low - current_low;

        let plus_dm = if up_move > down_move && up_move > 0.0 { up_move } else { 0.0 };
        let minus_dm = if down_move > up_move && down_move > 0.0 { down_move } else { 0.0 };

        atr = atr - atr * reciprocal_period + tr;
        plus_dm_smooth = plus_dm_smooth - plus_dm_smooth * reciprocal_period + plus_dm;
        minus_dm_smooth = minus_dm_smooth - minus_dm_smooth * reciprocal_period + minus_dm;

        let plus_di_current = if atr != 0.0 { (plus_dm_smooth / atr) * 100.0 } else { 0.0 };
        let minus_di_current = if atr != 0.0 { (minus_dm_smooth / atr) * 100.0 } else { 0.0 };

        let dx = if (plus_di_current + minus_di_current) != 0.0 {
            ((plus_di_current - minus_di_current).abs() / (plus_di_current + minus_di_current)) * 100.0
        } else {
            0.0
        };

        if dx_count < period {
            dx_sum += dx;
            dx_count += 1;

            if dx_count == period {
                let first_adx = dx_sum * reciprocal_period;
                adx.push(first_adx);
                adx_buffer.push_back(first_adx);
                // ADXR cannot be calculated yet as we need adxr_period ADX values
            }
        } else {
            let adx_current = (adx[adx.len() - 1] * (period as f64 - 1.0) + dx) * reciprocal_period;
            adx.push(adx_current);

            // Compute ADXR if enough ADX values are available
            if adx_buffer.len() == period {
                let adxr_value = (adx_current + adx_buffer.pop_front().unwrap()) / 2.0;
                adxr.push(adxr_value);
            }

            adx_buffer.push_back(adx_current);
        }
    }

    Ok(adxr)
}

mod tests {
    use super::*;
    use crate::indicators::data_loader::TEST_CANDLES;

    #[test]
    fn test_adxr_accuracy() {
        let candles: std::sync::MutexGuard<'_, Candles> = TEST_CANDLES.lock().unwrap();
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