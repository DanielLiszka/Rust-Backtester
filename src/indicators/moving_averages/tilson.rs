/// # Tilson T3 Moving Average (T3)
///
/// A specialized moving average that applies multiple iterations of an
/// exponential smoothing algorithm, enhanced by a volume factor (`v_factor`)
/// parameter. This factor controls the overall responsiveness, allowing you to
/// tune between smoother (lower `v_factor`) or more reactive (higher `v_factor`)
/// outputs. Commonly referred to simply as “T3”, it’s favored for its balance
/// of smoothness and minimal lag.
///
/// ## Parameters
/// - **period**: The look-back period for smoothing (defaults to 5).
/// - **volume_factor**: Controls the “depth” of the T3 smoothing. Typically in the
///   range [0.0, 1.0], where higher values yield more aggressive smoothing (defaults to 0.0).
///
/// ## Errors
/// - **EmptyData**: tilson: The input slice is empty.
/// - **InvalidPeriod**: tilson: `period` is zero or exceeds the available data length.
/// - **InvalidVolumeFactor**: tilson: The `volume_factor` is invalid (`NaN` or infinite).
/// - **AllValuesNaN**: tilson: All input data values are `NaN`.
///
/// ## Returns
/// - **`Ok(TilsonOutput)`** on success, containing a `Vec<f64>` matching the input length,
///   with `NaN` values where insufficient data exists for the calculation.
/// - **`Err(TilsonError)`** otherwise.
use crate::utilities::data_loader::{source_type, Candles};

#[derive(Debug, Clone)]
pub enum TilsonData<'a> {
    Candles {
        candles: &'a Candles,
        source: &'a str,
    },
    Slice(&'a [f64]),
}

#[derive(Debug, Clone)]
pub struct TilsonOutput {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct TilsonParams {
    pub period: Option<usize>,
    pub volume_factor: Option<f64>,
}

impl Default for TilsonParams {
    fn default() -> Self {
        Self {
            period: Some(5),
            volume_factor: Some(0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TilsonInput<'a> {
    pub data: TilsonData<'a>,
    pub params: TilsonParams,
}

impl<'a> TilsonInput<'a> {
    pub fn from_candles(candles: &'a Candles, source: &'a str, params: TilsonParams) -> Self {
        Self {
            data: TilsonData::Candles { candles, source },
            params,
        }
    }

    pub fn from_slice(slice: &'a [f64], params: TilsonParams) -> Self {
        Self {
            data: TilsonData::Slice(slice),
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles) -> Self {
        Self {
            data: TilsonData::Candles {
                candles,
                source: "close",
            },
            params: TilsonParams::default(),
        }
    }

    pub fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| TilsonParams::default().period.unwrap())
    }

    pub fn get_volume_factor(&self) -> f64 {
        self.params
            .volume_factor
            .unwrap_or_else(|| TilsonParams::default().volume_factor.unwrap())
    }
}

use std::f64;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TilsonError {
    #[error("tilson: No data available: data length = 0 for Tilson calculation.")]
    EmptyData,
    #[error("tilson: Invalid period: period = {period}, data length = {data_len} for Tilson calculation.")]
    InvalidPeriod { period: usize, data_len: usize },
    #[error("tilson: Invalid volume factor: {v_factor} for Tilson calculation.")]
    InvalidVolumeFactor { v_factor: f64 },
    #[error("tilson: All values are NaN during Tilson calculation..")]
    AllValuesNaN,
}

#[inline]
pub fn tilson(input: &TilsonInput) -> Result<TilsonOutput, TilsonError> {
    let data: &[f64] = match &input.data {
        TilsonData::Candles { candles, source } => source_type(candles, source),
        TilsonData::Slice(slice) => slice,
    };

    let first_valid_idx = match data.iter().position(|&x| !x.is_nan()) {
        Some(idx) => idx,
        None => return Err(TilsonError::AllValuesNaN),
    };

    let length = data.len();
    if length == 0 {
        return Err(TilsonError::EmptyData);
    }

    let opt_in_time_period = input.get_period();
    if opt_in_time_period == 0 || opt_in_time_period > length {
        return Err(TilsonError::InvalidPeriod {
            period: opt_in_time_period,
            data_len: length,
        });
    }

    let opt_in_v_factor = input.get_volume_factor();
    if opt_in_v_factor.is_nan() || opt_in_v_factor.is_infinite() {
        return Err(TilsonError::InvalidVolumeFactor {
            v_factor: opt_in_v_factor,
        });
    }

    let lookback_total = 6 * (opt_in_time_period - 1);
    let mut out_values = vec![f64::NAN; length];

    if lookback_total >= length {
        return Err(TilsonError::InvalidPeriod {
            period: opt_in_time_period,
            data_len: length,
        });
    }

    let start_idx = first_valid_idx + lookback_total;
    let end_idx = length - 1;

    let k = 2.0 / (opt_in_time_period as f64 + 1.0);
    let one_minus_k = 1.0 - k;

    let temp = opt_in_v_factor * opt_in_v_factor;
    let c1 = -(temp * opt_in_v_factor);
    let c2 = 3.0 * (temp - c1);
    let c3 = -6.0 * temp - 3.0 * (opt_in_v_factor - c1);
    let c4 = 1.0 + 3.0 * opt_in_v_factor - c1 + 3.0 * temp;

    let mut today = 0_usize;
    let mut temp_real;
    let mut e1;
    let mut e2;
    let mut e3;
    let mut e4;
    let mut e5;
    let mut e6;

    temp_real = 0.0;
    for i in 0..opt_in_time_period {
        temp_real += data[first_valid_idx + today + i];
    }
    e1 = temp_real / (opt_in_time_period as f64);
    today += opt_in_time_period;

    temp_real = e1;
    for _ in 1..opt_in_time_period {
        e1 = k * data[first_valid_idx + today] + one_minus_k * e1;
        temp_real += e1;
        today += 1;
    }
    e2 = temp_real / (opt_in_time_period as f64);

    temp_real = e2;
    for _ in 1..opt_in_time_period {
        e1 = k * data[first_valid_idx + today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        temp_real += e2;
        today += 1;
    }
    e3 = temp_real / (opt_in_time_period as f64);

    temp_real = e3;
    for _ in 1..opt_in_time_period {
        e1 = k * data[first_valid_idx + today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        temp_real += e3;
        today += 1;
    }
    e4 = temp_real / (opt_in_time_period as f64);

    temp_real = e4;
    for _ in 1..opt_in_time_period {
        e1 = k * data[first_valid_idx + today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        temp_real += e4;
        today += 1;
    }
    e5 = temp_real / (opt_in_time_period as f64);

    temp_real = e5;
    for _ in 1..opt_in_time_period {
        e1 = k * data[first_valid_idx + today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        e5 = k * e4 + one_minus_k * e5;
        temp_real += e5;
        today += 1;
    }
    e6 = temp_real / (opt_in_time_period as f64);

    while (first_valid_idx + today) <= start_idx {
        e1 = k * data[first_valid_idx + today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        e5 = k * e4 + one_minus_k * e5;
        e6 = k * e5 + one_minus_k * e6;
        today += 1;
    }

    if start_idx < length {
        out_values[start_idx] = c1 * e6 + c2 * e5 + c3 * e4 + c4 * e3;
    }

    let mut out_idx = start_idx + 1;
    while (first_valid_idx + today) <= end_idx {
        e1 = k * data[first_valid_idx + today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        e5 = k * e4 + one_minus_k * e5;
        e6 = k * e5 + one_minus_k * e6;

        out_values[out_idx] = c1 * e6 + c2 * e5 + c3 * e4 + c4 * e3;

        today += 1;
        out_idx += 1;
    }

    Ok(TilsonOutput { values: out_values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_tilson_partial_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let default_params = TilsonParams {
            period: None,
            volume_factor: None,
        };
        let input_default = TilsonInput::from_candles(&candles, "close", default_params);
        let output_default = tilson(&input_default).expect("Failed T3/Tilson with default params");
        assert_eq!(output_default.values.len(), candles.close.len());

        let params_custom_period = TilsonParams {
            period: Some(10),
            volume_factor: None,
        };
        let input_custom_period = TilsonInput::from_candles(&candles, "hl2", params_custom_period);
        let output_custom_period =
            tilson(&input_custom_period).expect("Failed T3/Tilson with period=10, source=hl2");
        assert_eq!(output_custom_period.values.len(), candles.close.len());

        let params_fully_custom = TilsonParams {
            period: Some(7),
            volume_factor: Some(0.9),
        };
        let input_fully_custom = TilsonInput::from_candles(&candles, "hlc3", params_fully_custom);
        let output_fully_custom =
            tilson(&input_fully_custom).expect("Failed T3/Tilson fully custom");
        assert_eq!(output_fully_custom.values.len(), candles.close.len());
    }

    #[test]
    fn test_tilson_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = TilsonParams {
            period: Some(5),
            volume_factor: Some(0.0),
        };
        let input = TilsonInput::from_candles(&candles, "close", params);
        let t3_result = tilson(&input).expect("Failed to calculate T3/Tilson");

        let expected_last_five_t3 = [
            59304.716332473254,
            59283.56868015526,
            59261.16173577631,
            59240.25895948583,
            59203.544843167765,
        ];
        assert!(t3_result.values.len() >= 5);
        assert_eq!(t3_result.values.len(), close_prices.len());

        let start_index = t3_result.values.len() - 5;
        let result_last_five_t3 = &t3_result.values[start_index..];
        for (i, &value) in result_last_five_t3.iter().enumerate() {
            let expected_value = expected_last_five_t3[i];
            assert!(
                (value - expected_value).abs() < 1e-10,
                "T3 mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }

        let default_input = TilsonInput::with_default_candles(&candles);
        let default_t3_result =
            tilson(&default_input).expect("Failed to calculate T3 with defaults");
        assert_eq!(default_t3_result.values.len(), close_prices.len());
    }

    #[test]
    fn test_tilson_with_default_candles() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = TilsonInput::with_default_candles(&candles);
        match input.data {
            TilsonData::Candles { source, .. } => {
                assert_eq!(source, "close");
            }
            _ => panic!("Expected TilsonData::Candles"),
        }
        let period = input.params.period.unwrap_or(5);
        let v_factor = input.params.volume_factor.unwrap_or(0.0);
        assert_eq!(period, 5);
        assert!(v_factor.abs() < f64::EPSILON);
    }

    #[test]
    fn test_tilson_with_default_params() {
        let default_params = TilsonParams::default();
        assert_eq!(default_params.period, Some(5));
        assert_eq!(default_params.volume_factor, Some(0.0));
    }

    #[test]
    fn test_tilson_with_no_data() {
        let data: [f64; 0] = [];
        let params = TilsonParams {
            period: Some(5),
            volume_factor: Some(0.0),
        };
        let input = TilsonInput::from_slice(&data, params);
        let result = tilson(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_tilson_very_small_data_set() {
        let data = [42.0];
        let params = TilsonParams {
            period: Some(5),
            volume_factor: Some(0.0),
        };
        let input = TilsonInput::from_slice(&data, params);
        let result = tilson(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_tilson_with_slice_data_reinput() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let first_input = TilsonInput::from_candles(
            &candles,
            "close",
            TilsonParams {
                period: Some(5),
                volume_factor: Some(0.0),
            },
        );
        let first_result = tilson(&first_input).expect("First T3/Tilson failed");
        let second_input = TilsonInput::from_slice(
            &first_result.values,
            TilsonParams {
                period: Some(3),
                volume_factor: Some(0.7),
            },
        );
        let second_result = tilson(&second_input).expect("Second T3/Tilson failed");
        assert_eq!(second_result.values.len(), first_result.values.len());
        for i in 240..second_result.values.len() {
            assert!(second_result.values[i].is_finite());
        }
    }

    #[test]
    fn test_tilson_accuracy_nan_check() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = TilsonInput::from_candles(
            &candles,
            "close",
            TilsonParams {
                period: Some(5),
                volume_factor: Some(0.0),
            },
        );
        let result = tilson(&input).expect("T3/Tilson calculation failed");
        assert_eq!(result.values.len(), candles.close.len());
        for (idx, &val) in result.values.iter().enumerate().skip(50) {
            assert!(val.is_finite(), "NaN found at index {}", idx);
        }
    }
}
