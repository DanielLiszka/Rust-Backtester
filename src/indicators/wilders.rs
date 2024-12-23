use std::error::Error;

#[derive(Debug, Clone)]
pub struct WildersParams {
    pub period: Option<usize>,
}

impl Default for WildersParams {
    fn default() -> Self {
        WildersParams {
            period: Some(5),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WildersInput<'a> {
    pub data: &'a [f64],
    pub params: WildersParams,
}

impl<'a> WildersInput<'a> {
    pub fn new(data: &'a [f64], params: WildersParams) -> Self {
        WildersInput { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        WildersInput {
            data,
            params: WildersParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| WildersParams::default().period.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct WildersOutput {
    pub values: Vec<f64>,
}

#[inline]
pub fn calculate_wilders(input: &WildersInput) -> Result<WildersOutput, Box<dyn Error>> {
    let data = input.data;
    let period = input.get_period();

    let n = data.len();
    if period == 0 || period > n {
        return Err("Invalid period specified for Wilder's Moving Average.".into());
    }

    let out_size = n - (period - 1);
    let mut out = Vec::with_capacity(out_size);

    let per = 1.0 / (period as f64);

    let mut sum = 0.0;
    for i in 0..period {
        sum += data[i];
    }
    let mut val = sum / period as f64;
    out.push(val);

    for i in period..n {
        val = (data[i] - val) * per + val;
        out.push(val);
    }

    Ok(WildersOutput { values: out })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_wilders_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = WildersParams { period: Some(5) };
        let input = WildersInput::new(&close_prices, params);

        let wilder_result = calculate_wilders(&input).expect("Wilder's calculation failed");
        let out_vals = &wilder_result.values;

        let expected_last_five = [
            59302.18156619092,
            59277.94525295273,
            59230.15620236219,
            59215.12496188975,
            59103.09996951180,
        ];

        assert!(
            out_vals.len() >= 5,
            "Not enough Wilder's output to compare final 5 values."
        );

        let start_idx = out_vals.len() - 5;
        let actual_last_five = &out_vals[start_idx..];

        for (i, (&actual, &expected)) in actual_last_five.iter().zip(&expected_last_five).enumerate()
        {
            let diff = (actual - expected).abs();
            assert!(
                diff < 1e-10,
                "Mismatch at index {}: expected {}, got {}, diff {}",
                i,
                expected,
                actual,
                diff
            );
        }

        let default_input = WildersInput::with_default_params(&close_prices);
        let default_output =
            calculate_wilders(&default_input).expect("Wilder's default calculation failed");
        assert!(
            !default_output.values.is_empty(),
            "Should produce some output with default period"
        );
    }
}
