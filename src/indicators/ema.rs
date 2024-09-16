#[inline]
pub fn calculate_ema(data: &[f64], period: usize) -> Vec<f64> {
    let len = data.len();
    let alpha = 2.0 / (period as f64 + 1.0);
    let mut ema_values = Vec::with_capacity(len);

    let mut last_ema = data[0];
    ema_values.push(last_ema);

    for i in 1..data.len() {
        last_ema = alpha * data[i] + (1.0 - alpha) * last_ema;
        ema_values.push(last_ema);
    }

    ema_values
}

#[test]
fn test_ema_accuracy() {
    use super::data_loader::TEST_CLOSE_PRICES;  // Access CLOSE_PRICES from the local data_loader module

    let close_prices = TEST_CLOSE_PRICES.lock().unwrap();  // Access the global data

    let period = 9;
    let result_ema = calculate_ema(&close_prices, period);

    let expected_last_four_ema = vec![
        59302.2,
        59277.9,
        59230.2,
        59215.1
    ];

    let start_index = result_ema.len() - 5;
    let result_last_five_ema = &result_ema[start_index..result_ema.len() - 1];

    for (i, &value) in result_last_five_ema.iter().enumerate() {
        assert!((value - expected_last_four_ema[i]).abs() < 1e-1,
            "ema value miematch at index {}: expected {}, got {}", i, expected_last_four_ema[i], value);
    }
}