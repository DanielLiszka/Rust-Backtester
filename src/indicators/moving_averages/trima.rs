/// # Triangular Moving Average (TRIMA)
///
/// A moving average computed by averaging an underlying Simple Moving Average (SMA) over
/// the specified `period`, resulting in a smoother output than a single SMA.
/// For instance, a TRIMA with a period of 14 first computes a 14-period SMA, then
/// applies additional averaging steps to produce a triangular shape of weights.
///
/// ## Parameters
/// - **period**: Window size (must be > 3).
///
/// ## Errors
/// - **NotEnoughData**: trima: Not enough data points for the requested `period`.
/// - **PeriodTooSmall**: trima: `period` ≤ 3.
/// - **AllValuesNaN**: trima: All data values are `NaN`.
/// - **NoData**: trima: No data provided.
///
/// ## Returns
/// - **`Ok(TrimaOutput)`** on success, containing a `Vec<f64>` of length matching the input.
/// - **`Err(TrimaError)`** otherwise.
use crate::utilities::data_loader::{source_type, Candles};

#[derive(Debug, Clone)]
pub enum TrimaData<'a> {
    Candles {
        candles: &'a Candles,
        source: &'a str,
    },
    Slice(&'a [f64]),
}

#[derive(Debug, Clone)]
pub struct TrimaOutput {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct TrimaParams {
    pub period: Option<usize>,
}

impl Default for TrimaParams {
    fn default() -> Self {
        Self { period: Some(14) }
    }
}

#[derive(Debug, Clone)]
pub struct TrimaInput<'a> {
    pub data: TrimaData<'a>,
    pub params: TrimaParams,
}

impl<'a> TrimaInput<'a> {
    pub fn from_candles(candles: &'a Candles, source: &'a str, params: TrimaParams) -> Self {
        Self {
            data: TrimaData::Candles { candles, source },
            params,
        }
    }

    pub fn from_slice(slice: &'a [f64], params: TrimaParams) -> Self {
        Self {
            data: TrimaData::Slice(slice),
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles) -> Self {
        Self {
            data: TrimaData::Candles {
                candles,
                source: "close",
            },
            params: TrimaParams::default(),
        }
    }

    pub fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| TrimaParams::default().period.unwrap())
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrimaError {
    #[error("Not enough data points to calculate TRIMA. Needed: {needed}, found: {found}")]
    NotEnoughData { needed: usize, found: usize },

    #[error("TRIMA period must be greater than 3. Provided: {period}")]
    PeriodTooSmall { period: usize },

    #[error("All values are NaN for Trima calculation.")]
    AllValuesNaN,

    #[error("No data provided for Trima calculation.")]
    NoData,
}
impl From<crate::indicators::sma::SmaError> for TrimaError {
    fn from(err: crate::indicators::sma::SmaError) -> Self {
        TrimaError::NoData
    }
}

#[inline]
pub fn trima(input: &TrimaInput) -> Result<TrimaOutput, TrimaError> {
    let data: &[f64] = match &input.data {
        TrimaData::Candles { candles, source } => source_type(candles, source),
        TrimaData::Slice(slice) => slice,
    };
    if let TrimaData::Slice(_) = input.data {
        return trima_from_slice(input);
    }
    let n: usize = data.len();
    let period: usize = input.get_period();

    if period > n {
        return Err(TrimaError::NotEnoughData {
            needed: period,
            found: n,
        });
    }
    if period <= 3 {
        return Err(TrimaError::PeriodTooSmall { period });
    }
    if n == 0 {
        return Err(TrimaError::NoData);
    }

    let first_valid_idx = match data.iter().position(|&x| !x.is_nan()) {
        Some(idx) => idx,
        None => return Err(TrimaError::AllValuesNaN),
    };

    if (n - first_valid_idx) < period {
        return Err(TrimaError::NotEnoughData {
            needed: period,
            found: n - first_valid_idx,
        });
    }

    let mut out = vec![f64::NAN; n];

    let sum_of_weights = if period % 2 == 1 {
        let half = period / 2 + 1;
        (half * half) as f64
    } else {
        let half_up = period / 2 + 1;
        let half_down = period / 2;
        (half_up * half_down) as f64
    };
    let inv_weights = 1.0 / sum_of_weights;

    let lead_period = if period % 2 == 1 {
        period / 2
    } else {
        (period / 2) - 1
    };
    let trail_period = lead_period + 1;

    let mut weight_sum = 0.0;
    let mut lead_sum = 0.0;
    let mut trail_sum = 0.0;

    let mut w = 1;

    for i in 0..(period - 1) {
        let idx = first_valid_idx + i;
        let val = data[idx];

        weight_sum += val * (w as f64);

        if i + 1 > period - lead_period {
            lead_sum += val;
        }

        if i < trail_period {
            trail_sum += val;
        }

        if i + 1 < trail_period {
            w += 1;
        }
        if i + 1 >= (period - lead_period) {
            w -= 1;
        }
    }

    let mut lsi = (period - 1) as isize - lead_period as isize + 1;
    let mut tsi1 = (period - 1) as isize - period as isize + 1 + trail_period as isize;
    let mut tsi2 = (period - 1) as isize - period as isize + 1;

    for i in (first_valid_idx + (period - 1))..n {
        let val = data[i];
        weight_sum += val;

        out[i] = weight_sum * inv_weights;

        lead_sum += val;
        weight_sum += lead_sum;
        weight_sum -= trail_sum;

        if lsi >= 0 {
            lead_sum -= data[lsi as usize];
        }
        if tsi1 >= 0 {
            trail_sum += data[tsi1 as usize];
        }
        if tsi2 >= 0 {
            trail_sum -= data[tsi2 as usize];
        }

        lsi += 1;
        tsi1 += 1;
        tsi2 += 1;
    }

    Ok(TrimaOutput { values: out })
}

use crate::indicators::sma::{sma, SmaData, SmaInput, SmaParams};

#[inline]
pub fn trima_from_slice(input: &TrimaInput) -> Result<TrimaOutput, TrimaError> {
    let data: &[f64] = match &input.data {
        TrimaData::Candles { candles, source } => source_type(candles, source),
        TrimaData::Slice(slice) => slice,
    };

    let n = data.len();
    let period = input.get_period().max(1);

    if n == 0 {
        return Err(TrimaError::NoData);
    }
    if period > n {
        return Err(TrimaError::NotEnoughData {
            needed: period,
            found: n,
        });
    }
    if period <= 3 {
        return Err(TrimaError::PeriodTooSmall { period });
    }

    let m1 = (period + 1) / 2;
    let m2 = period - m1 + 1;
    let input1 = SmaInput {
        data: SmaData::Slice(&data),
        params: SmaParams { period: Some(m1) },
    };
    let pass1 = sma(&input1)?.values;
    let input2 = SmaInput {
        data: SmaData::Slice(&pass1),
        params: SmaParams { period: Some(m2) },
    };
    let pass2 = sma(&input2)?;

    let pass2_values = pass2.values;
    Ok(TrimaOutput {
        values: pass2_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_trima_partial_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let default_params = TrimaParams { period: None };
        let input = TrimaInput::from_candles(&candles, "close", default_params);
        let output = trima(&input).expect("Failed TRIMA with default params");
        assert_eq!(output.values.len(), candles.close.len());
        let params_period_10 = TrimaParams { period: Some(10) };
        let input2 = TrimaInput::from_candles(&candles, "hl2", params_period_10);
        let output2 = trima(&input2).expect("Failed TRIMA with period=10, source=hl2");
        assert_eq!(output2.values.len(), candles.close.len());
        let params_custom = TrimaParams { period: Some(30) };
        let input3 = TrimaInput::from_candles(&candles, "hlc3", params_custom);
        let output3 = trima(&input3).expect("Failed TRIMA fully custom");
        assert_eq!(output3.values.len(), candles.close.len());
    }

    #[test]
    fn test_trima_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");
        let params = TrimaParams { period: Some(30) };
        let input = TrimaInput::from_candles(&candles, "close", params);
        let trima_result = trima(&input).expect("Failed to calculate TRIMA");
        assert_eq!(
            trima_result.values.len(),
            close_prices.len(),
            "TRIMA output length should match input data length"
        );
        let expected_last_five_trima = [
            59957.916666666664,
            59846.770833333336,
            59750.620833333334,
            59665.2125,
            59581.612499999996,
        ];
        assert!(
            trima_result.values.len() >= 5,
            "Not enough TRIMA values for the test"
        );
        let start_index = trima_result.values.len() - 5;
        let result_last_five_trima = &trima_result.values[start_index..];
        for (i, &value) in result_last_five_trima.iter().enumerate() {
            let expected_value = expected_last_five_trima[i];
            assert!(
                (value - expected_value).abs() < 1e-6,
                "TRIMA value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }
        let period = input.params.period.unwrap_or(14);
        for i in 0..(period - 1) {
            assert!(
                trima_result.values[i].is_nan(),
                "Expected NaN at early index {} for TRIMA, got {}",
                i,
                trima_result.values[i]
            );
        }
        let default_input = TrimaInput::with_default_candles(&candles);
        let default_trima_result =
            trima(&default_input).expect("Failed to calculate TRIMA with defaults");
        assert!(
            !default_trima_result.values.is_empty(),
            "Should produce some TRIMA values with default params"
        );
    }
    #[test]
    fn test_trima_params_with_default_params() {
        let default_params = TrimaParams::default();
        assert_eq!(default_params.period, Some(14));
    }

    #[test]
    fn test_trima_input_with_default_candles() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).unwrap();
        let input = TrimaInput::with_default_candles(&candles);
        match input.data {
            TrimaData::Candles { source, .. } => {
                assert_eq!(source, "close");
            }
            _ => panic!("Unexpected data variant"),
        }
    }

    #[test]
    fn test_trima_with_insufficient_data() {
        let input_data = [10.0, 20.0, 30.0];
        let params = TrimaParams { period: Some(14) };
        let input = TrimaInput::from_slice(&input_data, params);
        let result = trima(&input);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Not enough data points"));
        }
    }

    #[test]
    fn test_trima_period_too_small() {
        let input_data = [10.0, 20.0, 30.0, 40.0];
        let params = TrimaParams { period: Some(3) };
        let input = TrimaInput::from_slice(&input_data, params);
        let result = trima(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_trima_slice_data_reinput() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).unwrap();
        let first_params = TrimaParams { period: Some(14) };
        let first_input = TrimaInput::from_candles(&candles, "close", first_params);
        let first_result = trima(&first_input).unwrap();
        assert_eq!(first_result.values.len(), candles.close.len());

        let second_params = TrimaParams { period: Some(10) };
        let second_input = TrimaInput::from_slice(&first_result.values, second_params);
        let second_result = trima(&second_input).unwrap();
        assert_eq!(second_result.values.len(), first_result.values.len());
        for val in &second_result.values[240..] {
            assert!(val.is_finite());
        }
    }

    #[test]
    fn test_trima_nan_check() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).unwrap();
        let params = TrimaParams { period: Some(14) };
        let input = TrimaInput::from_candles(&candles, "close", params);
        let trima_result = trima(&input).unwrap();
        for &val in &trima_result.values {
            if !val.is_nan() {
                assert!(val.is_finite());
            }
        }
    }
}
