use crate::indicators::data_loader::Candles;
use std::error::Error;

#[inline]
pub fn calculate_adx(candles: &Candles, period: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    let high = candles.select_candle_field("high")?;
    let low = candles.select_candle_field("low")?;
    let close = candles.select_candle_field("close")?;

    let len = close.len();
    if period == 0 || period > len {
        return Err("Invalid period specified for ADX calculation.".into());
    }

    if len < period + 1 {
        return Err("Not enough data points to calculate ADX.".into());
    }

    let period_f64 = period as f64;
    let reciprocal_period = 1.0 / period_f64;
    let one_minus_rp = 1.0 - reciprocal_period;
    let period_minus_one = period_f64 - 1.0;

    let mut tr_sum = 0.0;
    let mut plus_dm_sum = 0.0;
    let mut minus_dm_sum = 0.0;

    // Initial sums
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

    // With real data, ATR should never be zero after initialization. We skip the check.
    let plus_di_prev = (plus_dm_smooth / atr) * 100.0;
    let minus_di_prev = (minus_dm_smooth / atr) * 100.0;

    let sum_di = plus_di_prev + minus_di_prev;
    let initial_dx = if sum_di != 0.0 {
        ((plus_di_prev - minus_di_prev).abs() / sum_di) * 100.0
    } else {
        0.0
    };

    let mut dx_sum = initial_dx;
    let mut dx_count = 1;
    let mut adx = Vec::with_capacity(len - period);

    // We'll track the last ADX value in a variable to avoid repeated indexing
    let mut last_adx = 0.0;
    let mut have_adx = false;

    for i in (period + 1)..len {
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

        let plus_dm = if up_move > down_move && up_move > 0.0 { up_move } else { 0.0 };
        let minus_dm = if down_move > up_move && down_move > 0.0 { down_move } else { 0.0 };

        // Smooth values
        atr = atr * one_minus_rp + tr;
        plus_dm_smooth = plus_dm_smooth * one_minus_rp + plus_dm;
        minus_dm_smooth = minus_dm_smooth * one_minus_rp + minus_dm;

        let plus_di_current = (plus_dm_smooth / atr) * 100.0;
        let minus_di_current = (minus_dm_smooth / atr) * 100.0;

        let sum_di_current = plus_di_current + minus_di_current;
        let dx = if sum_di_current != 0.0 {
            let diff = (plus_di_current - minus_di_current).abs();
            (diff / sum_di_current) * 100.0
        } else {
            0.0
        };

        if dx_count < period {
            dx_sum += dx;
            dx_count += 1;
            if dx_count == period {
                last_adx = dx_sum * reciprocal_period;
                adx.push(last_adx);
                have_adx = true;
            }
        } else {
            // Smooth ADX calculation
            // ADX[i] = ((ADX[i-1]*(period-1)) + DX) / period
            // Use last_adx from previous iteration
            let adx_current = ((last_adx * period_minus_one) + dx) * reciprocal_period;
            adx.push(adx_current);
            last_adx = adx_current;
        }
    }

    Ok(adx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::load_test_candles;

    #[test]
    fn test_ad_accuracy() {
        let candles = load_test_candles().expect("Failed to load test candles");
        let period: usize = 14;
        let ad_result: Vec<f64> = calculate_adx(&candles, period).expect("Failed to calculate adx");

        let expected_last_five_adx = vec![36.14, 36.52, 37.01, 37.46, 38.47];

        assert!(ad_result.len() >= 5, "Not enough adx values for the test");

        let start_index = ad_result.len() - 5;
        let result_last_five_ad = &ad_result[start_index..];

        for (i, &value) in result_last_five_ad.iter().enumerate() {
            let expected_value = expected_last_five_adx[i];
            assert!(
                (value - expected_value).abs() < 1e-1,
                "adx value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }
    }
}