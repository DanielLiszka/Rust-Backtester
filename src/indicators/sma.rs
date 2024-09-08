pub fn calculate_sma(data: &[f64], period: usize) -> Vec<f64> {
    let mut sma_values = Vec::with_capacity(data.len() - period + 1);
    let mut sum = 0.0;

    // Process the loop in a vectorizable manner
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
mod tests {
    use super::*;
    #[test]
    fn test_sma_accuracy() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_sma = vec![2.0, 3.0, 4.0, 5.0];
        let result = calculate_sma(&data, 3);
        assert_eq!(result, expected_sma);
    }
}