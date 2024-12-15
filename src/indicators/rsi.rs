use std::error::Error;

#[inline]
pub fn calculate_rsi(data: &[f64], period: usize) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    if period == 0 || period > data.len() {
        return Err("Invalid period specified for RSI calculation.".into());
    }

    let len = data.len();
    let mut rsi = Vec::with_capacity(len);

    rsi.extend(std::iter::repeat(f64::NAN).take(period));

    let inv_period = 1.0 / period as f64;
    let beta = 1.0 - inv_period;

    let mut avg_gain = 0.0;
    let mut avg_loss = 0.0;

    for i in 1..=period {
        let delta = data[i] - data[i - 1];
        if delta > 0.0 {
            avg_gain += delta;
        } else {
            avg_loss += -delta;
        }
    }

    avg_gain *= inv_period;
    avg_loss *= inv_period;

    let initial_rsi = if avg_gain + avg_loss == 0.0 {
        50.0
    } else {
        100.0 * avg_gain / (avg_gain + avg_loss)
    };
    rsi.push(initial_rsi);

    for i in (period + 1)..len {
        let delta = data[i] - data[i - 1];
        let gain = if delta > 0.0 { delta } else { 0.0 };
        let loss = if delta < 0.0 { -delta } else { 0.0 };

        avg_gain = inv_period * gain + beta * avg_gain;
        avg_loss = inv_period * loss + beta * avg_loss;

        let current_rsi = if avg_gain + avg_loss == 0.0 {
            50.0
        } else {
            100.0 * avg_gain / (avg_gain + avg_loss)
        };

        rsi.push(current_rsi);
    }

    Ok(rsi)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::{load_test_candles, Candles};

    #[test]
    fn test_rsi_accuracy() {
        let candles = load_test_candles().expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");
        let period = 14;

        let rsi_result = calculate_rsi(&close_prices, period).expect("Failed to calculate RSI");

        let expected_last_five_rsi = vec![43.42, 42.68, 41.62, 42.86, 39.01];

        assert!(rsi_result.len() >= 5, "Not enough RSI values for the test");

        let start_index = rsi_result.len().saturating_sub(5);
        let result_last_five_rsi = &rsi_result[start_index..];

        for (i, &value) in result_last_five_rsi.iter().enumerate() {
            assert!(
                (value - expected_last_five_rsi[i]).abs() < 1e-2,
                "RSI value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five_rsi[i],
                value
            );
        }
    }
}
