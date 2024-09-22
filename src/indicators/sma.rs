use std::error::Error;

#[inline]
pub fn calculate_sma(data: &[f64], period: usize) -> Result<Vec<f64>, Box<dyn Error>> {
    if period == 0 || period > data.len() {
        return Err("Invalid period specified for SMA calculation.".into());
    }
    
    let output_len = data.len().saturating_sub(period) + 1;
    let mut sma_values = Vec::with_capacity(output_len);

    let period_f64 = period as f64;
    let inv_period = 1.0 / period_f64;

    let mut sum: f64 = data.iter().take(period).sum();
    sma_values.push(sum * inv_period);

    for &value in &data[period..] {
        sum += value - data[sma_values.len() - 1];
        sma_values.push(sum * inv_period);
    }

    Ok(sma_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::TEST_CANDLES;
    use crate::indicators::data_loader::select_candle_field;

    #[test]
    fn test_sma_accuracy() {
        let candles = TEST_CANDLES.lock().unwrap();
        let close_prices = select_candle_field(&candles, "close").expect("Failed to extract close prices");
        let period = 9;
        let sma_result = calculate_sma(&close_prices, period).expect("Failed to calculate SMA");
        let expected_last_five_sma = vec![
            59180.8,
            59175.0,
            59129.4,
            59085.4,
            59133.7,
        ];
        assert!(
            sma_result.len() >= 5,
            "Not enough SMA values for the test"
        );
        let start_index = sma_result.len().saturating_sub(5);
        let result_last_five_sma = &sma_result[start_index..];

        for (i, &value) in result_last_five_sma.iter().enumerate() {
            assert!(
                (value - expected_last_five_sma[i]).abs() < 1e-1,
                "SMA value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five_sma[i],
                value
            );
        }
    }
}