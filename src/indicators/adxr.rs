use crate::indicators::data_loader::Candles;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AdxrParams {
    pub period: Option<usize>,
}

impl Default for AdxrParams {
    fn default() -> Self {
        // Common default for ADXR period is often 14
        AdxrParams { period: Some(14) }
    }
}

#[derive(Debug, Clone)]
pub struct AdxrInput<'a> {
    pub candles: &'a Candles,
    pub params: AdxrParams,
}

impl<'a> AdxrInput<'a> {
    pub fn new(candles: &'a Candles, params: AdxrParams) -> Self {
        AdxrInput { candles, params }
    }

    pub fn with_default_params(candles: &'a Candles) -> Self {
        AdxrInput {
            candles,
            params: AdxrParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| AdxrParams::default().period.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct AdxrOutput {
    pub values: Vec<f64>,
}

#[inline(always)]
pub fn calculate_adxr(input: &AdxrInput) -> Result<AdxrOutput, Box<dyn Error>> {
    let candles = input.candles;
    let period = input.get_period();

    let high = candles.select_candle_field("high")?;
    let low = candles.select_candle_field("low")?;
    let close = candles.select_candle_field("close")?;

    let len = close.len();
    if period == 0 || period > len {
        return Err("Invalid period specified for ADXR calculation.".into());
    }

    if len < period + 1 {
        return Err("Not enough data points to calculate ADXR.".into());
    }

    let period_f64 = period as f64;
    let rp = 1.0 / period_f64;

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

    let plus_di_prev = if atr != 0.0 {
        (plus_dm_smooth / atr) * 100.0
    } else {
        0.0
    };
    let minus_di_prev = if atr != 0.0 {
        (minus_dm_smooth / atr) * 100.0
    } else {
        0.0
    };

    let initial_dx = if plus_di_prev + minus_di_prev != 0.0 {
        ((plus_di_prev - minus_di_prev).abs() / (plus_di_prev + minus_di_prev)) * 100.0
    } else {
        0.0
    };

    let mut dx_sum = initial_dx;
    let mut dx_count = 1;
    let mut adx = Vec::with_capacity(len - period);

    for i in (period + 1)..len {
        let prev_close = close[i - 1];
        let current_high = high[i];
        let current_low = low[i];

        let tr = (current_high - current_low)
            .max((current_high - prev_close).abs())
            .max((current_low - prev_close).abs());

        let up_move = current_high - high[i - 1];
        let down_move = low[i - 1] - current_low;

        let plus_dm = if up_move > down_move && up_move > 0.0 {
            up_move
        } else {
            0.0
        };
        let minus_dm = if down_move > up_move && down_move > 0.0 {
            down_move
        } else {
            0.0
        };

        atr = atr - (atr * rp) + tr;
        plus_dm_smooth = plus_dm_smooth - (plus_dm_smooth * rp) + plus_dm;
        minus_dm_smooth = minus_dm_smooth - (minus_dm_smooth * rp) + minus_dm;

        let plus_di_current = if atr != 0.0 {
            (plus_dm_smooth / atr) * 100.0
        } else {
            0.0
        };
        let minus_di_current = if atr != 0.0 {
            (minus_dm_smooth / atr) * 100.0
        } else {
            0.0
        };

        let sum_di = plus_di_current + minus_di_current;
        let dx = if sum_di != 0.0 {
            ((plus_di_current - minus_di_current).abs() / sum_di) * 100.0
        } else {
            0.0
        };

        if dx_count < period {
            dx_sum += dx;
            dx_count += 1;

            if dx_count == period {
                let first_adx = dx_sum * rp;
                adx.push(first_adx);
            }
        } else {
            let previous_adx = *adx.last().unwrap();
            let adx_current = ((previous_adx * (period_f64 - 1.0)) + dx) * rp;
            adx.push(adx_current);
        }
    }

    let mut adxr = Vec::with_capacity(adx.len() - period);
    for i in period..adx.len() {
        adxr.push((adx[i] + adx[i - period]) / 2.0);
    }

    Ok(AdxrOutput { values: adxr })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_adxr_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        // Using specified parameters
        let params = AdxrParams { period: Some(14) };
        let input = AdxrInput::new(&candles, params);
        let ad_result = calculate_adxr(&input).expect("Failed to calculate adxr");

        let expected_last_five_ad = [37.10, 37.3, 37.0, 36.2, 36.3];

        assert!(
            ad_result.values.len() >= 5,
            "Not enough adxr values for the test"
        );

        let start_index = ad_result.values.len() - 5;
        let result_last_five_ad = &ad_result.values[start_index..];

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

        // Test with default parameters (no period specified)
        let default_input = AdxrInput::with_default_params(&candles);
        let default_adxr_result =
            calculate_adxr(&default_input).expect("Failed to calculate ADXR with defaults");
        assert!(
            !default_adxr_result.values.is_empty(),
            "Should produce ADXR values with default params"
        );
    }
}
