use std::error::Error;

#[derive(Debug, Clone)]
pub struct WmaParams {
    pub period: Option<usize>,
}

impl Default for WmaParams {
    fn default() -> Self {
        WmaParams {
            period: Some(30),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WmaInput<'a> {
    pub data: &'a [f64],
    pub params: WmaParams,
}

impl<'a> WmaInput<'a> {
    pub fn new(data: &'a [f64], params: WmaParams) -> Self {
        WmaInput { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        WmaInput {
            data,
            params: WmaParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params.period.unwrap_or(30)
    }
}

#[derive(Debug, Clone)]
pub struct WmaOutput {
    pub values: Vec<f64>,
}

pub fn calculate_wma(input: &WmaInput) -> Result<WmaOutput, Box<dyn Error>> {
    let data = input.data;
    let len = data.len();
    let period = input.get_period();
    let mut values = vec![f64::NAN; len];
    if period == 0 {
        return Err("Period cannot be zero for WMA calculation.".into());
    }
    if period > len {
        return Ok(WmaOutput { values });
    }
    if period == 1 {
        values.copy_from_slice(data);
        return Ok(WmaOutput { values });
    }

    let lookback_total = period - 1;
    let start_idx = lookback_total;
    let sum_of_weights = (period * (period + 1)) >> 1;
    let divider = sum_of_weights as f64;
    let period_f = period as f64;

    let mut period_sub = 0.0;
    let mut period_sum = 0.0;
    let mut in_idx = 0;
    let mut i = 1;

    while in_idx < start_idx {
        let val = data[in_idx];
        period_sub += val;
        period_sum += val * (i as f64);
        in_idx += 1;
        i += 1;
    }

    let mut trailing_idx = 0;
    {
        let val = data[in_idx];
        in_idx += 1;
        period_sub += val;
        period_sum += val * period_f;
        let trailing_val = data[trailing_idx];
        trailing_idx += 1;
        values[start_idx] = period_sum / divider;
        period_sum -= period_sub;
        let mut trailing_value = trailing_val;

        while in_idx < len {
            let new_val = data[in_idx];
            in_idx += 1;

            period_sub += new_val;
            period_sub -= trailing_value;

            period_sum += new_val * period_f;

            trailing_value = data[trailing_idx];
            trailing_idx += 1;

            values[in_idx - 1] = period_sum / divider;
            period_sum -= period_sub;
        }
    }

    Ok(WmaOutput { values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_wma_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let data = &candles.close;

        let input = WmaInput::with_default_params(data);
        let result = calculate_wma(&input).expect("Failed to calculate WMA");

        let expected_last_five = vec![
            59638.52903225806,
            59563.7376344086,
            59489.4064516129,
            59432.02580645162,
            59350.58279569892,
        ];

        assert!(result.values.len() >= 5, "Not enough WMA values");
        let start_index = result.values.len().saturating_sub(5);
        let last_five = &result.values[start_index..];

        for (i, &value) in last_five.iter().enumerate() {
            assert!(
                (value - expected_last_five[i]).abs() < 1e-6,
                "WMA value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five[i],
                value
            );
        }

        let period = input.get_period();
        for val in result.values.iter().skip(period - 1) {
            if !val.is_nan() {
                assert!(val.is_finite(), "WMA output should be finite");
            }
        }
    }
}
