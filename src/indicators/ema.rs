pub fn calculate_ema(data: &[f64], period: usize) -> Vec<f64> {
    let mut ema_values = Vec::with_capacity(data.len());
    let alpha = 2.0 / (period as f64 + 1.0);

    // Start by calculating the initial EMA using the SMA of the first `period` values
    let mut sum = 0.0;
    for i in 0..period {
        sum += data[i];
    }
    let mut ema = sum / period as f64;
    ema_values.push(ema);

    // Calculate EMA for the rest of the data
    for &price in &data[period..] {
        ema = alpha * price + (1.0 - alpha) * ema;
        ema_values.push(ema);
    }

    ema_values
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ema_accuracy() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        // Expected EMA values manually calculated (for demonstration purposes)
        let expected_ema = vec![2.0, 2.5, 3.0, 3.5, 4.0];
        let result = calculate_ema(&data, 3);
        assert_eq!(result, expected_ema);
    }
}