#[inline]
pub fn calculate_sma(data: &[f64], period: usize) -> Vec<f64> {
    let capacity: usize = data.len() - period + 1;
    let mut sma_values: Vec<f64> = Vec::with_capacity(capacity);
    let mut sum = 0.0;

    for i in 0..data.len() {
        sum += data[i];
        if i >= period - 1 {
            sma_values.push(sum / period as f64);
            sum -= data[i + 1 - period];
        }
    }
    sma_values
}

#[cfg(test)]
    #[test]
    fn test_sma_accuracy() {
        use super::data_loader::TEST_CLOSE_PRICES;  // Access CLOSE_PRICES from the local data_loader module
        let close_prices = TEST_CLOSE_PRICES.lock().unwrap();  // Access the global data

        let period = 9;
        let result_sma = calculate_sma(&close_prices, period);

        let expected_last_five_sma = vec![
            59180.8,
            59175.0,
            59129.4,
            59085.4,
        ];

        let start_index = result_sma.len() - 5;
        let result_last_five_sma = &result_sma[start_index..result_sma.len() - 1];

        for (i, &value) in result_last_five_sma.iter().enumerate() {
            assert!((value - expected_last_five_sma[i]).abs() < 1e-1,
                "SMA value mismatch at index {}: expected {}, got {}", i, expected_last_five_sma[i], value);
        }
    }