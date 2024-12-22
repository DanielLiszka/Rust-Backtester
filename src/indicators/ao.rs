use crate::indicators::data_loader::Candles;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AoParams {
    pub short_period: Option<usize>,
    pub long_period: Option<usize>,
}

impl Default for AoParams {
    fn default() -> Self {
        AoParams {
            short_period: Some(5),
            long_period: Some(34),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AoInput<'a> {
    pub candles: &'a Candles,
    pub params: AoParams,
}

impl<'a> AoInput<'a> {
    pub fn new(candles: &'a Candles, params: AoParams) -> Self {
        AoInput { candles, params }
    }

    pub fn with_default_params(candles: &'a Candles) -> Self {
        AoInput {
            candles,
            params: AoParams::default(),
        }
    }

    fn get_short_period(&self) -> usize {
        self.params.short_period.unwrap_or(5)
    }

    fn get_long_period(&self) -> usize {
        self.params.long_period.unwrap_or(34)
    }
}

#[derive(Debug, Clone)]
pub struct AoOutput {
    pub values: Vec<f64>,
}

pub fn calculate_ao(input: &AoInput) -> Result<AoOutput, Box<dyn Error>> {
    let candles = input.candles;
    let short = input.get_short_period();
    let long = input.get_long_period();

    if short == 0 || long == 0 || short >= long {
        return Err("Invalid periods specified for AO calculation.".into());
    }

    let len = candles.close.len();
    if len == 0 {
        return Err("No candles available.".into());
    }

    let high = candles.select_candle_field("high")?;
    let low = candles.select_candle_field("low")?;

    let mut hl2_values = Vec::with_capacity(len);
    for i in 0..len {
        hl2_values.push((high[i] + low[i]) * 0.5);
    }

    let mut ao_values = vec![f64::NAN; len];
    let mut short_sum = 0.0;
    let mut long_sum = 0.0;

    for i in 0..len {
        let val = hl2_values[i];
        short_sum += val;
        long_sum += val;

        // If we've accumulated more than `short` candles in short_sum, remove the oldest one
        if i >= short {
            short_sum -= hl2_values[i - short];
        }

        // Similarly for long_sum
        if i >= long {
            long_sum -= hl2_values[i - long];
        }

        // We have a valid short SMA after i >= short-1
        // We have a valid long SMA after i >= long-1
        if i >= long - 1 {
            let short_sma = short_sum / short as f64;
            let long_sma = long_sum / long as f64;
            ao_values[i] = short_sma - long_sma;
        }
    }

    Ok(AoOutput { values: ao_values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_ao_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AoInput::with_default_params(&candles);
        let result = calculate_ao(&input).expect("Failed to calculate AO");

        let expected_last_five = [-1671.3, -1401.6706, -1262.3559, -1178.4941, -1157.4118];

        assert!(
            result.values.len() >= 5,
            "Not enough AO values for the test"
        );

        let start_index = result.values.len().saturating_sub(5);
        let result_last_five = &result.values[start_index..];

        for (i, &value) in result_last_five.iter().enumerate() {
            assert!(
                (value - expected_last_five[i]).abs() < 1e-1,
                "AO value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five[i],
                value
            );
        }

        // Check that values are finite for valid indices
        for val in result.values.iter().skip(input.get_long_period() - 1) {
            assert!(val.is_finite(), "AO output should be finite");
        }
    }
}
