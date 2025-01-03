/// # Chaikin Accumulation/Distribution (AD)
///
/// A volume-based indicator that measures the cumulative flow of money into or out
/// of a security. For each bar (candle), it computes a “Money Flow Multiplier” (MFM),
/// derived from the position of the close relative to the high-low range, and then
/// multiplies this by the volume to determine “Money Flow Volume” (MFV). The AD
/// line is the cumulative sum of these MFVs over time.
///
/// ## Parameters
/// - This function does not have adjustable parameters beyond the required input data.
///
/// ## Errors
/// - **CandleFieldError**: ad: Failure retrieving one of the candle fields needed
///   (high, low, close, or volume).
/// - **DataLengthMismatch**: ad: The provided high, low, close, and volume slices
///   must all be of equal length.
/// - **NotEnoughData**: ad: The data length is zero; cannot compute AD values.
///
/// ## Returns
/// - **`Ok(AdOutput)`** on success, containing a `Vec<f64>` with the cumulative AD line.
/// - **`Err(AdError)`** otherwise.
use crate::utilities::data_loader::Candles;

#[derive(Debug, Clone)]
pub enum AdData<'a> {
    Candles {
        candles: &'a Candles,
    },
    Slices {
        high: &'a [f64],
        low: &'a [f64],
        close: &'a [f64],
        volume: &'a [f64],
    },
}

#[derive(Debug, Clone, Default)]
pub struct AdParams {}

#[derive(Debug, Clone)]
pub struct AdInput<'a> {
    pub data: AdData<'a>,
    pub params: AdParams,
}

impl<'a> AdInput<'a> {
    pub fn from_candles(candles: &'a Candles, params: AdParams) -> Self {
        Self {
            data: AdData::Candles { candles },
            params,
        }
    }

    pub fn from_slices(
        high: &'a [f64],
        low: &'a [f64],
        close: &'a [f64],
        volume: &'a [f64],
        params: AdParams,
    ) -> Self {
        Self {
            data: AdData::Slices {
                high,
                low,
                close,
                volume,
            },
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles) -> Self {
        Self {
            data: AdData::Candles { candles },
            params: AdParams::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdOutput {
    pub values: Vec<f64>,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdError {
    #[error(transparent)]
    CandleFieldError(#[from] Box<dyn std::error::Error>),
    #[error("ad: Data length mismatch for AD calculation: high={high_len}, low={low_len}, close={close_len}, volume={volume_len}")]
    DataLengthMismatch {
        high_len: usize,
        low_len: usize,
        close_len: usize,
        volume_len: usize,
    },
    #[error("ad: Not enough data points to calculate AD. Length={len}")]
    NotEnoughData { len: usize },
}

#[inline]
pub fn ad(input: &AdInput) -> Result<AdOutput, AdError> {
    let (high, low, close, volume) = match &input.data {
        AdData::Candles { candles } => {
            let high = candles.select_candle_field("high")?;
            let low = candles.select_candle_field("low")?;
            let close = candles.select_candle_field("close")?;
            let volume = candles.select_candle_field("volume")?;
            (high, low, close, volume)
        }
        AdData::Slices {
            high,
            low,
            close,
            volume,
        } => (*high, *low, *close, *volume),
    };

    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(AdError::DataLengthMismatch {
            high_len: high.len(),
            low_len: low.len(),
            close_len: close.len(),
            volume_len: volume.len(),
        });
    }

    let size = high.len();
    if size < 1 {
        return Err(AdError::NotEnoughData { len: size });
    }

    let mut output = Vec::with_capacity(size);
    let mut sum = 0.0;

    for ((&h, &l), (&c, &v)) in high
        .iter()
        .zip(low.iter())
        .zip(close.iter().zip(volume.iter()))
    {
        let hl = h - l;

        if hl != 0.0 {
            let mfm: f64 = ((c - l) - (h - c)) / hl;
            let mfv: f64 = mfm * v;
            sum += mfv;
        }
        output.push(sum);
    }

    Ok(AdOutput { values: output })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_ad_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let input = AdInput::with_default_candles(&candles);
        let ad_result = ad(&input).expect("Failed to calculate AD");

        assert_eq!(
            ad_result.values.len(),
            candles.close.len(),
            "AD output length does not match input length"
        );

        let expected_last_five_ad = [1645918.16, 1645876.11, 1645824.27, 1645828.87, 1645728.78];

        assert!(
            ad_result.values.len() >= 5,
            "Not enough AD values for the test"
        );
        let start_index = ad_result.values.len() - 5;
        let result_last_five_ad = &ad_result.values[start_index..];

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
    #[test]
    fn test_ad_params_with_default_params() {
        let default_params = AdParams::default();
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AdInput::from_candles(&candles, default_params);
        let result = ad(&input).expect("Failed to calculate AD");
        assert_eq!(result.values.len(), candles.close.len());
    }

    #[test]
    fn test_ad_input_with_default_candles() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AdInput::with_default_candles(&candles);
        match input.data {
            AdData::Candles { .. } => {}
            _ => panic!("Expected AdData::Candles variant"),
        }
    }

    #[test]
    fn test_ad_partial_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let partial_params = AdParams {};
        let input = AdInput::from_candles(&candles, partial_params);
        let result = ad(&input).expect("Failed to calculate AD with partial params");
        assert_eq!(result.values.len(), candles.close.len());
    }

    #[test]
    fn test_ad_with_slice_data_reinput() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let first_input = AdInput::with_default_candles(&candles);
        let first_result = ad(&first_input).expect("Failed to calculate first AD");
        let second_input = AdInput::from_slices(
            &first_result.values,
            &first_result.values,
            &first_result.values,
            &first_result.values,
            AdParams::default(),
        );
        let second_result = ad(&second_input).expect("Failed to calculate second AD");
        assert_eq!(second_result.values.len(), first_result.values.len());
        for i in 240..second_result.values.len() {
            assert!(!second_result.values[i].is_nan());
        }
    }

    #[test]
    fn test_ad_accuracy_nan_check() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let input = AdInput::with_default_candles(&candles);
        let ad_result = ad(&input).expect("Failed to calculate AD");
        assert_eq!(ad_result.values.len(), candles.close.len());
        if ad_result.values.len() > 50 {
            for i in 50..ad_result.values.len() {
                assert!(
                    !ad_result.values[i].is_nan(),
                    "Expected no NaN after index 50, but found NaN at index {}",
                    i
                );
            }
        }
    }
}
