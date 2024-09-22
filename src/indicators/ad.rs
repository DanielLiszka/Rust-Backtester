use crate::indicators::data_loader::Candles;
use std::error::Error;

pub fn calculate_ad(candles: &Candles) -> Result<Vec<f64>, Box<dyn Error>> {
    let high: &[f64] = candles.select_candle_field("high")?;
    let low: &[f64] = candles.select_candle_field("low")?;
    let close: &[f64] = candles.select_candle_field("close")?;
    let volume: &[f64] = candles.select_candle_field("volume")?;

    let size: usize = high.len();
    let mut output: Vec<f64> = Vec::with_capacity(size);
    let mut sum: f64 = 0.0;

    for ((&high, &low), (&close, &volume)) in high
        .iter()
        .zip(low.iter())
        .zip(close.iter().zip(volume.iter()))
    {
        let hl = high - low;

        if hl != 0.0 {
            let mfm: f64 = ((close - low) - (high - close)) / hl;
            let mfv: f64 = mfm * volume;
            sum += mfv;
        }
        output.push(sum);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::TEST_CANDLES;

    #[test]
    fn test_ad_accuracy() {
        let candles = TEST_CANDLES.lock().unwrap();
        let ad_result = calculate_ad(&candles).expect("Failed to calculate AD");

        let expected_last_five_ad = vec![
            1645918.16,
            1645876.11,
            1645824.27,
            1645828.87,
            1645728.78,
        ];

        assert!(
            ad_result.len() >= 5,
            "Not enough AD values for the test"
        );

        let start_index = ad_result.len() - 5;
        let result_last_five_ad = &ad_result[start_index..];

        for (i, &value) in result_last_five_ad.iter().enumerate() {
            let expected_value = expected_last_five_ad[i];
            assert!(
                (value - expected_value).abs() < 1e-1,
                "AD value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }
    }
}