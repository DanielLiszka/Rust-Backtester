use std::error::Error;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct HighPassParams {
    pub period: Option<usize>,
}

impl Default for HighPassParams {
    fn default() -> Self {
        HighPassParams { period: Some(48) }
    }
}

#[derive(Debug, Clone)]
pub struct HighPassInput<'a> {
    pub data: &'a [f64],
    pub params: HighPassParams,
}

impl<'a> HighPassInput<'a> {
    pub fn new(data: &'a [f64], params: HighPassParams) -> Self {
        HighPassInput { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        HighPassInput {
            data,
            params: HighPassParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params.period.unwrap_or(48)
    }
}

#[derive(Debug, Clone)]
pub struct HighPassOutput {
    pub values: Vec<f64>,
}

pub fn calculate_highpass(input: &HighPassInput) -> Result<HighPassOutput, Box<dyn Error>> {
    let data = input.data;
    let period = input.get_period();

    let len = data.len();
    if len == 0 {
        return Err("No data available for highpass calculation.".into());
    }

    if len < 2 {
        // If we have less than 2 data points, just return the available data
        return Ok(HighPassOutput {
            values: data.to_vec(),
        });
    }

    // Compute alpha
    let k = 1.0;
    let two_pi_k_div = 2.0 * PI * k / (period as f64);
    let sin_val = two_pi_k_div.sin();
    let cos_val = two_pi_k_div.cos();
    let alpha = 1.0 + (sin_val - 1.0) / cos_val;

    let one_minus_half_alpha = 1.0 - alpha / 2.0;
    let one_minus_alpha = 1.0 - alpha;

    // Initialize newseries with exact length and set the first element
    let mut newseries = vec![0.0; len];
    newseries[0] = data[0];

    for i in 1..len {
        // newseries[i] = (1 - alpha/2)*data[i] - (1 - alpha/2)*data[i-1] + (1 - alpha)*newseries[i-1]
        let val = one_minus_half_alpha * data[i] - one_minus_half_alpha * data[i - 1]
            + one_minus_alpha * newseries[i - 1];
        newseries[i] = val;
    }

    Ok(HighPassOutput { values: newseries })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_highpass_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        // We'll use close prices as input data
        let data = &candles.close;

        let input = HighPassInput::with_default_params(data);
        let result = calculate_highpass(&input).expect("Failed to calculate highpass");

        // Provided test values for the last 5:
        // -265.1027020005024, -330.0916060058495, -422.7478979710918, -261.87532144673423, -698.9026088956363
        let expected_last_five = [
            -265.1027020005024,
            -330.0916060058495,
            -422.7478979710918,
            -261.87532144673423,
            -698.9026088956363,
        ];

        assert!(result.values.len() >= 5, "Not enough highpass values");
        let start_index = result.values.len().saturating_sub(5);
        let last_five = &result.values[start_index..];

        for (i, &value) in last_five.iter().enumerate() {
            assert!(
                (value - expected_last_five[i]).abs() < 1e-6,
                "Highpass value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five[i],
                value
            );
        }

        // Check that values are finite
        for val in &result.values {
            assert!(val.is_finite(), "Highpass output should be finite");
        }
    }
}
