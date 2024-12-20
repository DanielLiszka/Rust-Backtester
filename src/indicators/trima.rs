use std::error::Error;

#[derive(Debug, Clone)]
pub struct TrimaParams {
    pub period: Option<usize>,
}

impl Default for TrimaParams {
    fn default() -> Self {
        // Default to a 14-period TRIMA if not specified
        TrimaParams {
            period: Some(14)
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrimaInput<'a> {
    pub data: &'a [f64],
    pub params: TrimaParams,
}

impl<'a> TrimaInput<'a> {
    pub fn new(data: &'a [f64], params: TrimaParams) -> Self {
        TrimaInput { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        TrimaInput {
            data,
            params: TrimaParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| TrimaParams::default().period.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct TrimaOutput {
    pub values: Vec<f64>,
}

#[inline]
pub fn calculate_trima(input: &TrimaInput) -> Result<TrimaOutput, Box<dyn Error>> {
    let data = input.data;
    let p = input.get_period();

    if p == 0 || p > data.len() {
        return Err("Invalid period specified for TRIMA calculation.".into());
    }

    let (m1, m2) = if p % 2 == 0 {
        (p / 2, (p / 2) + 1)
    } else {
        let half = (p + 1) / 2;
        (half, half)
    };

    let len_first_sma = data.len().saturating_sub(m1) + 1;
    if len_first_sma == 0 {
        return Err("Not enough data for first SMA".into());
    }

    let len_final = len_first_sma.saturating_sub(m2) + 1;
    if len_final == 0 {
        return Err("Not enough data for second SMA".into());
    }

    // Compute the first SMA inline
    let mut first_sma = Vec::with_capacity(len_first_sma);
    let inv_m1 = 1.0 / (m1 as f64);

    // Initial sum for first SMA
    let mut sum = 0.0;
    for &val in &data[..m1] {
        sum += val;
    }
    first_sma.push(sum * inv_m1);

    // Rolling calculation of first SMA
    for i in m1..data.len() {
        sum += data[i] - data[i - m1];
        first_sma.push(sum * inv_m1);
    }

    // Compute second SMA inline from first_sma
    let inv_m2 = 1.0 / (m2 as f64);
    let mut second_sum = 0.0;
    for &val in &first_sma[..m2] {
        second_sum += val;
    }

    let mut trima_values = Vec::with_capacity(len_final);
    trima_values.push(second_sum * inv_m2);

    for i in m2..first_sma.len() {
        second_sum += first_sma[i] - first_sma[i - m2];
        trima_values.push(second_sum * inv_m2);
    }

    Ok(TrimaOutput {
        values: trima_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::{read_candles_from_csv};

    #[test]
    fn test_trima_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = TrimaParams { period: Some(30) };
        let input = TrimaInput::new(&close_prices, params);
        let trima_result = calculate_trima(&input).expect("Failed to calculate TRIMA");

        // Corrected test values
        let expected_last_five_trima = vec![
            59957.916666666664,
            59846.770833333336,
            59750.620833333334,
            59665.2125,
            59581.612499999996,
        ];

        assert!(
            trima_result.values.len() >= 5,
            "Not enough TRIMA values for the test"
        );
        let start_index = trima_result.values.len() - 5;
        let result_last_five_trima = &trima_result.values[start_index..];

        for (i, &value) in result_last_five_trima.iter().enumerate() {
            let expected_value = expected_last_five_trima[i];
            assert!(
                (value - expected_value).abs() < 1e-6,
                "TRIMA value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }

        // Test default params (14)
        let default_input = TrimaInput::with_default_params(&close_prices);
        let default_trima_result = calculate_trima(&default_input)
            .expect("Failed to calculate TRIMA with defaults");
        assert!(
            !default_trima_result.values.is_empty(),
            "Should produce some TRIMA values with default params"
        );
    }
}
