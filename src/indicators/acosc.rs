/// # Accelerator Oscillator (ACOSC)
///
/// A momentum-based indicator attributed to Bill Williams, designed to measure the
/// acceleration or deceleration of the current driving force behind the price action.
/// It is derived from the difference between a short-term and medium-term moving average
/// of the median price (`(high + low) / 2`), followed by a further smoothing step.
/// This results in two outputs:
/// - **`osc`**: The oscillator values themselves.
/// - **`change`**: The one-period difference of the oscillator values, representing
///   a rate of change or “momentum” of the AC oscillator.
///
/// ## Parameters
/// - None. The AC Oscillator uses fixed settings (5- and 34-period simple moving averages).
///   You can provide data via [`AcoscData::Candles`] or [`AcoscData::Slices`].
///
/// ## Errors
/// - **CandleFieldError**: Failed to retrieve the required candle fields (`high`/`low`).
/// - **LengthMismatch**: `high` and `low` slices have different lengths.
/// - **NotEnoughData**: Insufficient number of data points to compute the AC oscillator
///   (requires at least 39 data points).
///
/// ## Returns
/// - **`Ok(AcoscOutput)`** on success, containing:
///   - `osc`: A `Vec<f64>` of AC oscillator values.
///   - `change`: A `Vec<f64>` of rate-of-change values.
/// - **`Err(AcoscError)`** otherwise.
use crate::utilities::data_loader::Candles;

#[derive(Debug, Clone)]
pub enum AcoscData<'a> {
    Candles { candles: &'a Candles },
    Slices { high: &'a [f64], low: &'a [f64] },
}

#[derive(Debug, Clone, Default)]
pub struct AcoscParams {}

#[derive(Debug, Clone)]
pub struct AcoscInput<'a> {
    pub data: AcoscData<'a>,
    pub params: AcoscParams,
}

impl<'a> AcoscInput<'a> {
    pub fn from_candles(candles: &'a Candles, params: AcoscParams) -> Self {
        Self {
            data: AcoscData::Candles { candles },
            params,
        }
    }

    pub fn from_slices(high: &'a [f64], low: &'a [f64], params: AcoscParams) -> Self {
        Self {
            data: AcoscData::Slices { high, low },
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles) -> Self {
        Self {
            data: AcoscData::Candles { candles },
            params: AcoscParams::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AcoscOutput {
    pub osc: Vec<f64>,
    pub change: Vec<f64>,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AcoscError {
    #[error(transparent)]
    CandleFieldError(#[from] Box<dyn std::error::Error>),

    #[error(
        "acosc: Mismatch in high/low candle data lengths: high_len={high_len}, low_len={low_len}"
    )]
    LengthMismatch { high_len: usize, low_len: usize },

    #[error(
        "acosc: Not enough data points to calculate AC oscillator: required={required}, actual={actual}"
    )]
    NotEnoughData { required: usize, actual: usize },
}

#[inline]
pub fn acosc(input: &AcoscInput) -> Result<AcoscOutput, AcoscError> {
    let (high_prices, low_prices) = match &input.data {
        AcoscData::Candles { candles } => {
            let high = candles.select_candle_field("high")?;
            let low = candles.select_candle_field("low")?;
            (high, low)
        }
        AcoscData::Slices { high, low } => (*high, *low),
    };

    if high_prices.len() != low_prices.len() {
        return Err(AcoscError::LengthMismatch {
            high_len: high_prices.len(),
            low_len: low_prices.len(),
        });
    }

    let len = low_prices.len();
    const PERIOD_SMA5: usize = 5;
    const PERIOD_SMA34: usize = 34;
    const INV_PERIOD_SMA5: f64 = 1.0 / PERIOD_SMA5 as f64;
    const INV_PERIOD_SMA34: f64 = 1.0 / PERIOD_SMA34 as f64;
    const REQUIRED_LENGTH: usize = PERIOD_SMA34 + PERIOD_SMA5;

    if len < REQUIRED_LENGTH {
        return Err(AcoscError::NotEnoughData {
            required: REQUIRED_LENGTH,
            actual: len,
        });
    }

    let mut osc = vec![f64::NAN; len];
    let mut change = vec![f64::NAN; len];

    let mut queue_sma5 = [0.0; PERIOD_SMA5];
    let mut queue_sma34 = [0.0; PERIOD_SMA34];
    let mut queue_sma5_ao = [0.0; PERIOD_SMA5];

    let mut sum_sma5 = 0.0;
    let mut sum_sma34 = 0.0;
    let mut sum_sma5_ao = 0.0;

    let mut idx_sma5 = 0;
    let mut idx_sma34 = 0;
    let mut idx_sma5_ao = 0;

    for i in 0..PERIOD_SMA34 {
        let medprice = (high_prices[i] + low_prices[i]) * 0.5;

        sum_sma34 += medprice;
        queue_sma34[i] = medprice;

        if i < PERIOD_SMA5 {
            sum_sma5 += medprice;
            queue_sma5[i] = medprice;
        }
    }

    for i in PERIOD_SMA34..(PERIOD_SMA34 + PERIOD_SMA5 - 1) {
        let medprice = (high_prices[i] + low_prices[i]) * 0.5;

        sum_sma34 += medprice - queue_sma34[idx_sma34];
        queue_sma34[idx_sma34] = medprice;
        idx_sma34 += 1;
        if idx_sma34 == PERIOD_SMA34 {
            idx_sma34 = 0;
        }
        let sma34 = sum_sma34 * INV_PERIOD_SMA34;

        sum_sma5 += medprice - queue_sma5[idx_sma5];
        queue_sma5[idx_sma5] = medprice;
        idx_sma5 += 1;
        if idx_sma5 == PERIOD_SMA5 {
            idx_sma5 = 0;
        }
        let sma5 = sum_sma5 * INV_PERIOD_SMA5;

        let ao = sma5 - sma34;

        sum_sma5_ao += ao;
        queue_sma5_ao[idx_sma5_ao] = ao;
        idx_sma5_ao += 1;
    }
    if idx_sma5_ao == PERIOD_SMA5 {
        idx_sma5_ao = 0;
    }

    let mut prev_res = 0.0;

    for i in (PERIOD_SMA34 + PERIOD_SMA5 - 1)..len {
        let medprice = (high_prices[i] + low_prices[i]) * 0.5;

        sum_sma34 += medprice - queue_sma34[idx_sma34];
        queue_sma34[idx_sma34] = medprice;
        idx_sma34 += 1;
        if idx_sma34 == PERIOD_SMA34 {
            idx_sma34 = 0;
        }
        let sma34 = sum_sma34 * INV_PERIOD_SMA34;

        sum_sma5 += medprice - queue_sma5[idx_sma5];
        queue_sma5[idx_sma5] = medprice;
        idx_sma5 += 1;
        if idx_sma5 == PERIOD_SMA5 {
            idx_sma5 = 0;
        }
        let sma5 = sum_sma5 * INV_PERIOD_SMA5;

        let ao = sma5 - sma34;

        let old_ao = queue_sma5_ao[idx_sma5_ao];
        sum_sma5_ao += ao - old_ao;
        queue_sma5_ao[idx_sma5_ao] = ao;
        idx_sma5_ao += 1;
        if idx_sma5_ao == PERIOD_SMA5 {
            idx_sma5_ao = 0;
        }
        let sma5_ao = sum_sma5_ao * INV_PERIOD_SMA5;

        let res = ao - sma5_ao;
        let mom = res - prev_res;
        prev_res = res;

        osc[i] = res;
        change[i] = mom;
    }

    Ok(AcoscOutput { osc, change })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_acosc_partial_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let default_params = AcoscParams::default();
        let input_default = AcoscInput::from_candles(&candles, default_params);
        let output_default = acosc(&input_default).expect("Failed ACOSC with default params");
        assert_eq!(output_default.osc.len(), candles.close.len());
        assert_eq!(output_default.change.len(), candles.close.len());
    }

    #[test]
    fn test_acosc_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let input = AcoscInput::with_default_candles(&candles);
        let acosc_result = acosc(&input).expect("Failed to calculate acosc");

        assert_eq!(
            acosc_result.osc.len(),
            candles.close.len(),
            "ACOSC output length (osc) does not match input length"
        );
        assert_eq!(
            acosc_result.change.len(),
            candles.close.len(),
            "ACOSC output length (change) does not match input length"
        );

        let expected_last_five_acosc_osc = [273.30, 383.72, 357.7, 291.25, 176.84];
        let expected_last_five_acosc_change = [49.6, 110.4, -26.0, -66.5, -114.4];

        assert!(acosc_result.osc.len() >= 5);
        assert!(acosc_result.change.len() >= 5);

        let start_index_osc = acosc_result.osc.len().saturating_sub(5);
        let result_last_five_acosc_osc = &acosc_result.osc[start_index_osc..];

        let start_index_change = acosc_result.change.len().saturating_sub(5);
        let result_last_five_acosc_change = &acosc_result.change[start_index_change..];

        for (i, &value) in result_last_five_acosc_osc.iter().enumerate() {
            assert!(
                (value - expected_last_five_acosc_osc[i]).abs() < 1e-1,
                "acosc osc value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five_acosc_osc[i],
                value
            );
        }

        for (i, &value) in result_last_five_acosc_change.iter().enumerate() {
            assert!(
                (value - expected_last_five_acosc_change[i]).abs() < 1e-1,
                "acosc change value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five_acosc_change[i],
                value
            );
        }
    }
    #[test]
    fn test_acosc_params_with_default_params() {
        let default_params = AcoscParams::default();
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let input = AcoscInput::from_candles(&candles, default_params);
        let result = acosc(&input).expect("Failed acosc with default params");
        assert_eq!(result.osc.len(), candles.close.len());
        assert_eq!(result.change.len(), candles.close.len());
    }

    #[test]
    fn test_acosc_input_with_default_candles() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let input = AcoscInput::with_default_candles(&candles);
        match input.data {
            AcoscData::Candles { .. } => {}
            _ => panic!("Expected AcoscData::Candles variant"),
        }
    }

    #[test]
    fn test_acosc_with_small_data_set() {
        let high = [100.0, 101.0];
        let low = [99.0, 98.0];
        let params = AcoscParams::default();
        let input = AcoscInput::from_slices(&high, &low, params);
        let result = acosc(&input);
        assert!(result.is_err(), "Expected error for not enough data");
    }

    #[test]
    fn test_acosc_with_slice_data_reinput() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let first_input = AcoscInput::with_default_candles(&candles);
        let first_result = acosc(&first_input).expect("Failed first acosc");

        assert_eq!(first_result.osc.len(), candles.close.len());
        assert_eq!(first_result.change.len(), candles.close.len());

        let high_reinput = &candles.high;
        let low_reinput = &candles.low;
        let second_params = AcoscParams::default();
        let second_input = AcoscInput::from_slices(high_reinput, low_reinput, second_params);
        let second_result = acosc(&second_input).expect("Failed second acosc");

        assert_eq!(second_result.osc.len(), candles.close.len());
        assert_eq!(second_result.change.len(), candles.close.len());
        if second_result.osc.len() > 240 {
            for i in 240..second_result.osc.len() {
                assert!(
                    !second_result.osc[i].is_nan(),
                    "Found NaN in osc at index {}",
                    i
                );
                assert!(
                    !second_result.change[i].is_nan(),
                    "Found NaN in change at index {}",
                    i
                );
            }
        }
    }

    #[test]
    fn test_acosc_accuracy_nan_check() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let input = AcoscInput::with_default_candles(&candles);
        let acosc_result = acosc(&input).expect("Failed to calculate acosc");
        assert_eq!(acosc_result.osc.len(), candles.close.len());
        assert_eq!(acosc_result.change.len(), candles.close.len());

        if acosc_result.osc.len() > 240 {
            for i in 240..acosc_result.osc.len() {
                assert!(!acosc_result.osc[i].is_nan(), "Found NaN in osc at {}", i);
                assert!(
                    !acosc_result.change[i].is_nan(),
                    "Found NaN in change at {}",
                    i
                );
            }
        }
    }
}
