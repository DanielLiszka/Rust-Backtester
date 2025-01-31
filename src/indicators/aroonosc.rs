/// # Aroon Oscillator
///
/// The Aroon Oscillator measures the relative time since the most recent highest
/// high and lowest low within a specified `length`. It oscillates between -100
/// and +100, providing insights into the strength and direction of a price trend.
/// Higher positive values indicate a stronger uptrend, while negative values
/// signify a more dominant downtrend.
///
/// ## Parameters
/// - **length**: The number of recent bars to look back when identifying the highest
///   high and lowest low (defaults to 14).
///
/// ## Errors
/// - **InvalidLength**: aroon_osc: The specified `length` is zero.
/// - **NoCandles**: aroon_osc: No candle data available.
/// - **EmptySlices**: aroon_osc: One or both high/low slices are empty.
/// - **SlicesLengthMismatch**: aroon_osc: High/low slices have different lengths.
/// - **NotEnoughData**: aroon_osc: Not enough data points to compute the Aroon Oscillator.
///
/// ## Returns
/// - **`Ok(AroonOscOutput)`** on success, containing a `Vec<f64>` of the oscillator values.
/// - **`Err(AroonOscError)`** otherwise.
use crate::utilities::data_loader::Candles;

#[derive(Debug, Clone)]
pub enum AroonOscData<'a> {
    Candles { candles: &'a Candles },
    SlicesHL { high: &'a [f64], low: &'a [f64] },
}

#[derive(Debug, Clone)]
pub struct AroonOscParams {
    pub length: Option<usize>,
}

impl Default for AroonOscParams {
    fn default() -> Self {
        Self { length: Some(14) }
    }
}

#[derive(Debug, Clone)]
pub struct AroonOscInput<'a> {
    pub data: AroonOscData<'a>,
    pub params: AroonOscParams,
}

impl<'a> AroonOscInput<'a> {
    pub fn from_candles(candles: &'a Candles, params: AroonOscParams) -> Self {
        Self {
            data: AroonOscData::Candles { candles },
            params,
        }
    }

    pub fn from_slices_hl(high: &'a [f64], low: &'a [f64], params: AroonOscParams) -> Self {
        Self {
            data: AroonOscData::SlicesHL { high, low },
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles) -> Self {
        Self {
            data: AroonOscData::Candles { candles },
            params: AroonOscParams::default(),
        }
    }

    pub fn get_length(&self) -> usize {
        self.params
            .length
            .unwrap_or_else(|| AroonOscParams::default().length.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct AroonOscOutput {
    pub values: Vec<f64>,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AroonOscError {
    #[error(transparent)]
    CandleFieldError(#[from] Box<dyn std::error::Error>),

    #[error("aroonosc: Invalid length specified for Aroon Osc calculation. length={length}")]
    InvalidLength { length: usize },

    #[error("aroonosc: No candles available.")]
    NoCandles,

    #[error("aroonosc: One or both of the slices for AroonOsc are empty.")]
    EmptySlices,

    #[error("aroonosc: Mismatch in high/low slice length. high_len={high_len}, low_len={low_len}")]
    SlicesLengthMismatch { high_len: usize, low_len: usize },

    #[error("aroonosc: Not enough data points for Aroon Osc: required={required}, found={found}")]
    NotEnoughData { required: usize, found: usize },
}

#[inline]
pub fn aroon_osc(input: &AroonOscInput) -> Result<AroonOscOutput, AroonOscError> {
    let length = input.get_length();
    if length == 0 {
        return Err(AroonOscError::InvalidLength { length });
    }

    let (high, low) = match &input.data {
        AroonOscData::Candles { candles } => {
            if candles.close.is_empty() {
                return Err(AroonOscError::NoCandles);
            }
            let high = candles.select_candle_field("high")?;
            let low = candles.select_candle_field("low")?;
            (high, low)
        }
        AroonOscData::SlicesHL { high, low } => {
            if high.is_empty() || low.is_empty() {
                return Err(AroonOscError::EmptySlices);
            }
            if high.len() != low.len() {
                return Err(AroonOscError::SlicesLengthMismatch {
                    high_len: high.len(),
                    low_len: low.len(),
                });
            }
            (*high, *low)
        }
    };

    let len = low.len();
    if len < length {
        return Err(AroonOscError::NotEnoughData {
            required: length,
            found: len,
        });
    }

    let mut values = vec![f64::NAN; len];
    let window = length + 1;
    let inv_length = 1.0 / length as f64;

    for i in (window - 1)..len {
        let start = i + 1 - window;
        let mut highest_val = high[start];
        let mut lowest_val = low[start];
        let mut highest_idx = start;
        let mut lowest_idx = start;

        for j in (start + 1)..=i {
            let h_val = high[j];
            if h_val > highest_val {
                highest_val = h_val;
                highest_idx = j;
            }
            let l_val = low[j];
            if l_val < lowest_val {
                lowest_val = l_val;
                lowest_idx = j;
            }
        }

        let offset_highest = i - highest_idx;
        let offset_lowest = i - lowest_idx;

        let up = (length as f64 - offset_highest as f64) * inv_length * 100.0;
        let down = (length as f64 - offset_lowest as f64) * inv_length * 100.0;

        values[i] = up - down;
    }

    Ok(AroonOscOutput { values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_aroon_osc_partial_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let partial_params = AroonOscParams { length: Some(20) };
        let input = AroonOscInput::from_candles(&candles, partial_params);
        let result = aroon_osc(&input).expect("Failed to calculate Aroon Osc with partial params");
        assert_eq!(result.values.len(), candles.close.len());
    }

    #[test]
    fn test_aroon_osc_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AroonOscInput::with_default_candles(&candles);
        let result = aroon_osc(&input).expect("Failed to calculate Aroon Osc");

        let expected_last_five = [-50.0, -50.0, -50.0, -50.0, -42.8571];

        assert!(result.values.len() >= 5, "Not enough Aroon Osc values");
        assert_eq!(
            result.values.len(),
            candles.close.len(),
            "Aroon Osc output length does not match input length!"
        );
        let start_index = result.values.len().saturating_sub(5);
        let last_five = &result.values[start_index..];

        for (i, &value) in last_five.iter().enumerate() {
            assert!(
                (value - expected_last_five[i]).abs() < 1e-2,
                "Aroon Osc mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five[i],
                value
            );
        }

        let length = 14;
        for val in result.values.iter().skip(length) {
            if !val.is_nan() {
                assert!(
                    val.is_finite(),
                    "Aroon Osc should be finite after enough data"
                );
            }
        }
    }

    #[test]
    fn test_aroon_osc_params_with_default_params() {
        let default_params = AroonOscParams::default();
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AroonOscInput::from_candles(&candles, default_params);
        let result = aroon_osc(&input).expect("Failed to calculate Aroon Osc");
        assert_eq!(result.values.len(), candles.close.len());
    }

    #[test]
    fn test_aroon_osc_input_with_default_candles() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AroonOscInput::with_default_candles(&candles);
        match input.data {
            AroonOscData::Candles { .. } => {}
            _ => panic!("Expected AroonOscData::Candles variant"),
        }
        assert!(input.params.length.is_some());
    }

    #[test]
    fn test_aroon_osc_with_slices_data_reinput() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let first_params = AroonOscParams { length: Some(10) };
        let first_input = AroonOscInput::from_candles(&candles, first_params);
        let first_result = aroon_osc(&first_input).expect("Failed to calculate first Aroon Osc");
        let second_params = AroonOscParams { length: Some(5) };
        let second_input = AroonOscInput::from_slices_hl(
            &first_result.values,
            &first_result.values,
            second_params,
        );
        let second_result = aroon_osc(&second_input).expect("Failed to calculate second Aroon Osc");
        assert_eq!(second_result.values.len(), first_result.values.len());
        for i in 240..second_result.values.len() {
            assert!(!second_result.values[i].is_nan());
        }
    }

    #[test]
    fn test_aroon_osc_accuracy_nan_check() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AroonOscInput::with_default_candles(&candles);
        let result = aroon_osc(&input).expect("Failed to calculate Aroon Osc");
        if result.values.len() > 50 {
            for i in 50..result.values.len() {
                assert!(
                    !result.values[i].is_nan(),
                    "Expected no NaN after index {}, but found NaN",
                    i
                );
            }
        }
    }
}
