use std::error::Error;

pub fn calculate_ema(data: &[f64], period: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    
    if period == 0 || period > data.len() {
        return Err("Invalid period specified for EMA calculation.".into());
    }
    
    let len = data.len();
    let alpha = 2.0 / (period as f64 + 1.0);
    let mut ema_values = Vec::with_capacity(len);
    
    let mut last_ema = data[0];
    ema_values.push(last_ema);
    
    for i in 1..len {
        last_ema = alpha * data[i] + (1.0 - alpha) * last_ema;
        ema_values.push(last_ema);
    }
    
    Ok(ema_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::TEST_CANDLES;
    use crate::indicators::data_loader::select_candle_field;

    #[test]
    fn test_ema_accuracy() {
        let candles = TEST_CANDLES.lock().unwrap();
        let close_prices = select_candle_field(&candles, "close").expect("Failed to extract close prices");
        let period = 9;

        let ema_result = calculate_ema(&close_prices, period).expect("Failed to calculate EMA");

        let expected_last_five_ema = vec![
            59302.2,
            59277.9,
            59230.2,
            59215.1,
            59103.1,
        ];

        assert!(
            ema_result.len() >= 5,
            "Not enough EMA values for the test"
        );

        let start_index = ema_result.len().saturating_sub(5);
        let result_last_five_ema = &ema_result[start_index..];

        for (i, &value) in result_last_five_ema.iter().enumerate() {
            assert!(
                (value - expected_last_five_ema[i]).abs() < 1e-1,
                "EMA value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five_ema[i],
                value
            );
        }

    }
}