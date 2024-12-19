use std::error::Error;
use crate::indicators::data_loader::Candles;

#[derive(Debug, Clone)]
pub struct AroonParams {
    pub length: Option<usize>,
}

impl Default for AroonParams {
    fn default() -> Self {
        AroonParams {
            length: Some(14),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AroonInput<'a> {
    pub candles: &'a Candles,
    pub params: AroonParams,
}

impl<'a> AroonInput<'a> {
    pub fn new(candles: &'a Candles, params: AroonParams) -> Self {
        AroonInput { candles, params }
    }

    pub fn with_default_params(candles: &'a Candles) -> Self {
        AroonInput {
            candles,
            params: AroonParams::default(),
        }
    }

    fn get_length(&self) -> usize {
        self.params.length.unwrap_or(14)
    }
}

#[derive(Debug, Clone)]
pub struct AroonOutput {
    pub aroon_up: Vec<f64>,
    pub aroon_down: Vec<f64>,
}

pub fn calculate_aroon(input: &AroonInput) -> Result<AroonOutput, Box<dyn Error>> {
    let candles = input.candles;
    let length = input.get_length();

    if length == 0 {
        return Err("Invalid length specified for Aroon calculation.".into());
    }

    let len = candles.close.len();
    if len == 0 {
        return Err("No candles available.".into());
    }

    let high = candles.select_candle_field("high")?;
    let low = candles.select_candle_field("low")?;

    let mut aroon_up = vec![f64::NAN; len];
    let mut aroon_down = vec![f64::NAN; len];

    // We look back length+1 bars including the current one
    let window = length + 1;

    for i in 0..len {
        if i+1 < window {
            // Not enough data yet
            continue;
        }

        let start = i + 1 - window; 
        // Find highest and lowest in the last length+1 bars
        let mut highest = high[start];
        let mut highest_idx = start;
        let mut lowest = low[start];
        let mut lowest_idx = start;

        for j in (start+1)..=i {
            let h_val = high[j];
            if h_val > highest {
                highest = h_val;
                highest_idx = j;
            }
            let l_val = low[j];
            if l_val < lowest {
                lowest = l_val;
                lowest_idx = j;
            }
        }

        let offset_highest = i - highest_idx; // bars since highest
        let offset_lowest = i - lowest_idx;   // bars since lowest

        // Aroon Up = ((length - offset_highest)/length)*100
        // Aroon Down = ((length - offset_lowest)/length)*100
        let up_val = ((length - offset_highest) as f64 / length as f64) * 100.0;
        let down_val = ((length - offset_lowest) as f64 / length as f64) * 100.0;

        aroon_up[i] = up_val;
        aroon_down[i] = down_val;
    }

    Ok(AroonOutput {
        aroon_up,
        aroon_down,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_aroon_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AroonInput::with_default_params(&candles);
        let result = calculate_aroon(&input).expect("Failed to calculate Aroon");

        // Given test values for the last 5:
        // Aroon Up: 21.43%,14.29%, 7.14%, 0.00%, 0.00%
        // Aroon Down: 71.43%, 64.29%, 57.14%, 50.00%, 42.86%
        let expected_up_last_five = vec![21.43, 14.29, 7.14, 0.0, 0.0];
        let expected_down_last_five = vec![71.43,64.29,57.14,50.0,42.86];

        assert!(result.aroon_up.len() >= 5 && result.aroon_down.len() >= 5, "Not enough Aroon values");
        let start_index = result.aroon_up.len().saturating_sub(5);

        let up_last_five = &result.aroon_up[start_index..];
        let down_last_five = &result.aroon_down[start_index..];

        for (i, &value) in up_last_five.iter().enumerate() {
            assert!(
                (value - expected_up_last_five[i]).abs() < 1e-2,
                "Aroon Up mismatch at index {}: expected {}, got {}",
                i,
                expected_up_last_five[i],
                value
            );
        }

        for (i, &value) in down_last_five.iter().enumerate() {
            assert!(
                (value - expected_down_last_five[i]).abs() < 1e-2,
                "Aroon Down mismatch at index {}: expected {}, got {}",
                i,
                expected_down_last_five[i],
                value
            );
        }

        // Check finite after enough data
        let length = input.get_length();
        for val in result.aroon_up.iter().skip(length) {
            if !val.is_nan() {
                assert!(val.is_finite(), "Aroon Up should be finite after enough data");
            }
        }
        for val in result.aroon_down.iter().skip(length) {
            if !val.is_nan() {
                assert!(val.is_finite(), "Aroon Down should be finite after enough data");
            }
        }
    }
}
