use std::error::Error;

#[inline]
pub fn calculate_sma(data: &[f64], period: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    if period == 0 || period > data.len() {
        return Err("Invalid period specified for SMA calculation.".into());
    }

    let output_len = data.len() - period + 1;
    let mut sma_values = Vec::with_capacity(output_len);

    let inv_period = 1.0 / period as f64;

    let mut sum: f64 = data[..period].iter().sum();
    sma_values.push(sum * inv_period);

    for i in period..data.len() {
        sum += data[i] - data[i - period];
        sma_values.push(sum * inv_period);
    }

    Ok(sma_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::{load_test_candles, Candles};

    #[test]
    fn test_sma_accuracy() {
        // Lock the TEST_CANDLES mutex to safely access the data
        let candles = load_test_candles().expect("Failed to load test candles");

        // Use the select_candle_field method from the Candles struct
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let period = 9;
        let sma_result = calculate_sma(&close_prices, period).expect("Failed to calculate SMA");

        // Expected SMA values (these should be updated to match your actual data)
        let expected_last_five_sma = vec![59180.8, 59175.0, 59129.4, 59085.4, 59133.7];

        assert!(
            sma_result.len() >= 5,
            "Not enough SMA values for the test"
        );

        let start_index = sma_result.len() - 5;
        let result_last_five_sma = &sma_result[start_index..];

        for (i, &value) in result_last_five_sma.iter().enumerate() {
            let expected_value = expected_last_five_sma[i];
            assert!(
                (value - expected_value).abs() < 1e-1,
                "SMA value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }
    }
}