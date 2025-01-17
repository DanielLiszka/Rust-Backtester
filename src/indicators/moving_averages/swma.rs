/// # Symmetric Weighted Moving Average (SWMA)
///
/// A moving average that applies triangular (symmetric) weighting across its
/// window. This places more emphasis on the center of the period than on its
/// extremes, offering a balance between smoothing and responsiveness. If the
/// input data is empty, this implementation returns an empty `Vec<f64>` without
/// error.
///
/// ## Parameters
/// - **period**: Window size (number of data points). Defaults to 5.
///
/// ## Errors
/// - **AllValuesNaN**: swma: All input data values are `NaN`.
/// - **InvalidPeriod**: swma: `period` is zero.
/// - **PeriodExceedsLength**: swma: `period` is larger than the input data length.
///
/// ## Returns
/// - **`Ok(SwmaOutput)`** on success, containing a `Vec<f64>` of length matching the input.
/// - **`Err(SwmaError)`** otherwise.
use crate::utilities::data_loader::{source_type, Candles};

#[derive(Debug, Clone)]
pub enum SwmaData<'a> {
    Candles {
        candles: &'a Candles,
        source: &'a str,
    },
    Slice(&'a [f64]),
}

#[derive(Debug, Clone)]
pub struct SwmaOutput {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct SwmaParams {
    pub period: Option<usize>,
}

impl Default for SwmaParams {
    fn default() -> Self {
        Self { period: Some(5) }
    }
}

#[derive(Debug, Clone)]
pub struct SwmaInput<'a> {
    pub data: SwmaData<'a>,
    pub params: SwmaParams,
}

impl<'a> SwmaInput<'a> {
    pub fn from_candles(candles: &'a Candles, source: &'a str, params: SwmaParams) -> Self {
        Self {
            data: SwmaData::Candles { candles, source },
            params,
        }
    }

    pub fn from_slice(slice: &'a [f64], params: SwmaParams) -> Self {
        Self {
            data: SwmaData::Slice(slice),
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles) -> Self {
        Self {
            data: SwmaData::Candles {
                candles,
                source: "close",
            },
            params: SwmaParams::default(),
        }
    }

    pub fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| SwmaParams::default().period.unwrap())
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SwmaError {
    #[error("Swma: All input values are NaN.")]
    AllValuesNaN,

    #[error("SWMA period must be >= 1. Provided: {period}")]
    InvalidPeriod { period: usize },

    #[error("SWMA period cannot exceed data length. Period: {period}, data length: {data_len}")]
    PeriodExceedsLength { period: usize, data_len: usize },
}

#[inline]
pub fn swma(input: &SwmaInput) -> Result<SwmaOutput, SwmaError> {
    let data: &[f64] = match &input.data {
        SwmaData::Candles { candles, source } => source_type(candles, source),
        SwmaData::Slice(slice) => slice,
    };
    let len = data.len();
    let period = input.get_period();

    if data.is_empty() {
        return Ok(SwmaOutput { values: vec![] });
    }

    if period == 0 {
        return Err(SwmaError::InvalidPeriod { period });
    }
    if period > len {
        return Err(SwmaError::PeriodExceedsLength {
            period,
            data_len: len,
        });
    }
    let weights = build_symmetric_triangle(period);

    let mut swma_values = vec![f64::NAN; len];

    for i in (period - 1)..len {
        let window_start = i + 1 - period;
        let window = &data[window_start..=i];
        let mut sum = 0.0;
        for (w_idx, &val) in window.iter().enumerate() {
            sum += val * weights[w_idx];
        }
        swma_values[i] = sum;
    }

    Ok(SwmaOutput {
        values: swma_values,
    })
}

#[inline]
fn build_symmetric_triangle(n: usize) -> Vec<f64> {
    let n = n.max(2);

    let triangle: Vec<f64> = if n == 2 {
        vec![1.0, 1.0]
    } else if n % 2 == 0 {
        let half = n / 2;
        let mut front: Vec<f64> = (1..=half).map(|x| x as f64).collect();
        let mut back = front.clone();
        back.reverse();
        front.extend(back);
        front
    } else {
        let half_plus = ((n + 1) as f64 / 2.0).floor() as usize;
        let mut front: Vec<f64> = (1..=half_plus).map(|x| x as f64).collect();
        let mut tri = front.clone();
        front.pop();
        front.reverse();
        tri.extend(front);
        tri
    };

    let sum: f64 = triangle.iter().sum();
    triangle.into_iter().map(|x| x / sum).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_swma_partial_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let default_params = SwmaParams { period: None };
        let input = SwmaInput::from_candles(&candles, "close", default_params);
        let output = swma(&input).expect("Failed SWMA with default params");
        assert_eq!(output.values.len(), candles.close.len());

        let params_period_10 = SwmaParams { period: Some(10) };
        let input2 = SwmaInput::from_candles(&candles, "hl2", params_period_10);
        let output2 = swma(&input2).expect("Failed SWMA with period=10, source=hl2");
        assert_eq!(output2.values.len(), candles.close.len());

        let params_custom = SwmaParams { period: Some(20) };
        let input3 = SwmaInput::from_candles(&candles, "hlc3", params_custom);
        let output3 = swma(&input3).expect("Failed SWMA fully custom");
        assert_eq!(output3.values.len(), candles.close.len());
    }

    #[test]
    fn test_swma_accuracy() {
        let file_path: &str = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles: Candles =
            read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices: &[f64] = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let default_params = SwmaParams::default();
        let input = SwmaInput::from_candles(&candles, "close", default_params);
        let result = swma(&input).expect("SWMA calculation failed");
        let len = result.values.len();
        assert_eq!(len, close_prices.len(), "Length mismatch");

        let expected_last_five = [
            59288.22222222222,
            59301.99999999999,
            59247.33333333333,
            59179.88888888889,
            59080.99999999999,
        ];
        assert!(
            len >= expected_last_five.len(),
            "Not enough SWMA values for the test"
        );
        let start_index = len - expected_last_five.len();
        let actual_last_five = &result.values[start_index..];
        for (i, (&actual, &expected)) in actual_last_five
            .iter()
            .zip(expected_last_five.iter())
            .enumerate()
        {
            let diff = (actual - expected).abs();
            assert!(
                diff < 1e-8,
                "SWMA mismatch at index {}: expected {:.14}, got {:.14}",
                i,
                expected,
                actual
            );
        }
    }
    #[test]
    fn test_swma_params_with_default_params() {
        let default_params = SwmaParams::default();
        assert_eq!(default_params.period, Some(5));
    }

    #[test]
    fn test_swma_input_with_default_candles() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).unwrap();
        let input = SwmaInput::with_default_candles(&candles);
        match input.data {
            SwmaData::Candles { source, .. } => {
                assert_eq!(source, "close");
            }
            _ => panic!("Expected SwmaData::Candles variant"),
        }
    }

    #[test]
    fn test_swma_empty_data() {
        let input_data: [f64; 0] = [];
        let params = SwmaParams { period: Some(5) };
        let input = SwmaInput::from_slice(&input_data, params);
        let result = swma(&input).unwrap();
        assert_eq!(result.values.len(), 0);
    }

    #[test]
    fn test_swma_with_zero_period() {
        let input_data = [10.0, 20.0, 30.0];
        let params = SwmaParams { period: Some(0) };
        let input = SwmaInput::from_slice(&input_data, params);
        let result = swma(&input);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("SWMA period must be >= 1."));
        }
    }

    #[test]
    fn test_swma_with_period_exceeding_data_length() {
        let input_data = [10.0, 20.0, 30.0];
        let params = SwmaParams { period: Some(10) };
        let input = SwmaInput::from_slice(&input_data, params);
        let result = swma(&input);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e
                .to_string()
                .contains("SWMA period cannot exceed data length."));
        }
    }

    #[test]
    fn test_swma_very_small_data_set() {
        let input_data = [42.0, 43.0];
        let params = SwmaParams { period: Some(2) };
        let input = SwmaInput::from_slice(&input_data, params);
        let result = swma(&input).unwrap();
        assert_eq!(result.values.len(), input_data.len());
        assert!(result.values[0].is_nan());
        assert!(!result.values[1].is_nan());
    }

    #[test]
    fn test_swma_with_slice_data_reinput() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).unwrap();
        let first_params = SwmaParams { period: Some(5) };
        let first_input = SwmaInput::from_candles(&candles, "close", first_params);
        let first_result = swma(&first_input).unwrap();
        assert_eq!(first_result.values.len(), candles.close.len());

        let second_params = SwmaParams { period: Some(3) };
        let second_input = SwmaInput::from_slice(&first_result.values, second_params);
        let second_result = swma(&second_input).unwrap();
        assert_eq!(second_result.values.len(), first_result.values.len());
        for val in &second_result.values[240..] {
            assert!(!val.is_nan());
        }
    }

    #[test]
    fn test_swma_nan_check() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).unwrap();
        let params = SwmaParams { period: Some(5) };
        let input = SwmaInput::from_candles(&candles, "close", params);
        let swma_result = swma(&input).unwrap();
        for &val in &swma_result.values {
            assert!(val.is_nan() || val.is_finite());
        }
    }
}
