/// # Donchian Channel
///
/// Donchian Channels consist of three bands generated by the highest high and the
/// lowest low over a specified period. The upper band is the highest price observed
/// during the period, the lower band is the lowest price, and the middle band is the
/// average of these two values.
///
/// ## Parameters
/// - **period**: The lookback window size for computing the highest high and lowest low.
///   Defaults to 20.
///
/// ## Errors
/// - **EmptyData**: donchian: Input data slice is empty.
/// - **InvalidPeriod**: donchian: `period` is zero or exceeds the data length.
/// - **NotEnoughValidData**: donchian: Fewer than `period` valid (non-`NaN`) data points remain
///   after the first valid index.
/// - **AllValuesNaN**: donchian: All input data values are `NaN`.
/// - **MismatchedLength**: donchian: High and Low data slices have different lengths.
/// - **CandleDataError**: donchian: Error retrieving candle data from file/datasource.
///
/// ## Returns
/// - **`Ok(DonchianOutput)`** on success, containing `upperband`, `middleband`, and `lowerband`
///   vectors each matching the input length, with leading `NaN`s until the window is filled
///   after the first valid index.
/// - **`Err(DonchianError)`** otherwise.
use crate::utilities::data_loader::Candles;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum DonchianData<'a> {
    Candles { candles: &'a Candles },
    Slices { high: &'a [f64], low: &'a [f64] },
}

#[derive(Debug, Clone)]
pub struct DonchianOutput {
    pub upperband: Vec<f64>,
    pub middleband: Vec<f64>,
    pub lowerband: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct DonchianParams {
    pub period: Option<usize>,
}

impl Default for DonchianParams {
    fn default() -> Self {
        Self { period: Some(20) }
    }
}

#[derive(Debug, Clone)]
pub struct DonchianInput<'a> {
    pub data: DonchianData<'a>,
    pub params: DonchianParams,
}

impl<'a> DonchianInput<'a> {
    pub fn from_candles(candles: &'a Candles, params: DonchianParams) -> Self {
        Self {
            data: DonchianData::Candles { candles },
            params,
        }
    }

    pub fn from_slices(high: &'a [f64], low: &'a [f64], params: DonchianParams) -> Self {
        Self {
            data: DonchianData::Slices { high, low },
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles) -> Self {
        Self {
            data: DonchianData::Candles { candles },
            params: DonchianParams::default(),
        }
    }

    pub fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| DonchianParams::default().period.unwrap())
    }
}

#[derive(Debug, Error)]
pub enum DonchianError {
    #[error("donchian: Empty data provided.")]
    EmptyData,
    #[error("donchian: Invalid period: period = {period}, data length = {data_len}")]
    InvalidPeriod { period: usize, data_len: usize },
    #[error("donchian: Not enough valid data: needed = {needed}, valid = {valid}")]
    NotEnoughValidData { needed: usize, valid: usize },
    #[error("donchian: All values are NaN.")]
    AllValuesNaN,
    #[error("donchian: High/Low data slices have different lengths.")]
    MismatchedLength,
}

#[inline]
pub fn donchian(input: &DonchianInput) -> Result<DonchianOutput, DonchianError> {
    let (high, low) = match &input.data {
        DonchianData::Candles { candles } => {
            let high = candles.select_candle_field("high").unwrap();
            let low = candles.select_candle_field("low").unwrap();
            (high, low)
        }
        DonchianData::Slices { high, low } => {
            if high.is_empty() || low.is_empty() {
                return Err(DonchianError::EmptyData);
            }
            if high.len() != low.len() {
                return Err(DonchianError::MismatchedLength);
            }
            (*high, *low)
        }
    };

    if high.is_empty() || low.is_empty() {
        return Err(DonchianError::EmptyData);
    }
    if high.len() != low.len() {
        return Err(DonchianError::MismatchedLength);
    }

    let period = input.get_period();
    if period == 0 || period > high.len() {
        return Err(DonchianError::InvalidPeriod {
            period,
            data_len: high.len(),
        });
    }

    let first_valid_high = high.iter().position(|&x| !x.is_nan());
    let first_valid_low = low.iter().position(|&x| !x.is_nan());
    let first_valid_idx = match (first_valid_high, first_valid_low) {
        (Some(h_idx), Some(l_idx)) => h_idx.min(l_idx),
        _ => return Err(DonchianError::AllValuesNaN),
    };

    if (high.len() - first_valid_idx) < period {
        return Err(DonchianError::NotEnoughValidData {
            needed: period,
            valid: high.len() - first_valid_idx,
        });
    }

    let mut upperband = vec![f64::NAN; high.len()];
    let mut middleband = vec![f64::NAN; high.len()];
    let mut lowerband = vec![f64::NAN; high.len()];

    let mut max_idx_ring = vec![0_usize; period];
    let mut min_idx_ring = vec![0_usize; period];
    let mut max_head = 0;
    let mut max_tail = 0;
    let mut min_head = 0;
    let mut min_tail = 0;

    let start_idx = first_valid_idx + period - 1;

    for i in first_valid_idx..high.len() {
        while max_head != max_tail && max_idx_ring[max_head] <= i.saturating_sub(period) {
            max_head = (max_head + 1) % period;
        }
        while min_head != min_tail && min_idx_ring[min_head] <= i.saturating_sub(period) {
            min_head = (min_head + 1) % period;
        }

        let hv = high[i];
        let lv = low[i];

        while max_head != max_tail {
            let last_pos = if max_tail == 0 {
                period - 1
            } else {
                max_tail - 1
            };
            let idx = max_idx_ring[last_pos];
            if high[idx] <= hv {
                max_tail = last_pos;
            } else {
                break;
            }
        }

        while min_head != min_tail {
            let last_pos = if min_tail == 0 {
                period - 1
            } else {
                min_tail - 1
            };
            let idx = min_idx_ring[last_pos];
            if low[idx] >= lv {
                min_tail = last_pos;
            } else {
                break;
            }
        }

        max_idx_ring[max_tail] = i;
        min_idx_ring[min_tail] = i;
        max_tail = (max_tail + 1) % period;
        min_tail = (min_tail + 1) % period;

        if i >= start_idx {
            let max_idx = max_idx_ring[max_head];
            let min_idx = min_idx_ring[min_head];
            let ub = high[max_idx];
            let lb = low[min_idx];
            upperband[i] = ub;
            lowerband[i] = lb;
            middleband[i] = (ub + lb) * 0.5;
        }
    }

    Ok(DonchianOutput {
        upperband,
        middleband,
        lowerband,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_donchian_with_default_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let input = DonchianInput::with_default_candles(&candles);
        let output = donchian(&input).expect("Failed Donchian with default params");

        assert_eq!(output.upperband.len(), candles.close.len());
        assert_eq!(output.middleband.len(), candles.close.len());
        assert_eq!(output.lowerband.len(), candles.close.len());
    }

    #[test]
    fn test_donchian_custom_params() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let params = DonchianParams { period: Some(14) };
        let input = DonchianInput::from_candles(&candles, params);
        let output = donchian(&input).expect("Failed Donchian with period=14");

        assert_eq!(output.upperband.len(), candles.close.len());
        assert_eq!(output.middleband.len(), candles.close.len());
        assert_eq!(output.lowerband.len(), candles.close.len());
    }

    #[test]
    fn test_donchian_accuracy_fixed_values() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let params = DonchianParams { period: Some(20) };
        let input = DonchianInput::from_candles(&candles, params);
        let output = donchian(&input).expect("Failed to calculate Donchian");

        let expected_last_five_upper = [61290.0, 61290.0, 61290.0, 61290.0, 61290.0];
        let expected_last_five_middle = [59583.0, 59583.0, 59583.0, 59583.0, 59583.0];
        let expected_last_five_lower = [57876.0, 57876.0, 57876.0, 57876.0, 57876.0];

        assert!(output.upperband.len() >= 5);
        let start_index = output.upperband.len() - 5;

        for i in 0..5 {
            let u = output.upperband[start_index + i];
            let m = output.middleband[start_index + i];
            let l = output.lowerband[start_index + i];
            assert!((u - expected_last_five_upper[i]).abs() < 1e-1);
            assert!((m - expected_last_five_middle[i]).abs() < 1e-1);
            assert!((l - expected_last_five_lower[i]).abs() < 1e-1);
        }
    }

    #[test]
    fn test_donchian_with_zero_period() {
        let high_data = [10.0, 20.0, 30.0];
        let low_data = [5.0, 3.0, 2.0];
        let params = DonchianParams { period: Some(0) };
        let input = DonchianInput::from_slices(&high_data, &low_data, params);

        let result = donchian(&input);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Invalid period"));
        }
    }

    #[test]
    fn test_donchian_with_period_exceeding_data_length() {
        let high_data = [10.0, 20.0, 30.0];
        let low_data = [5.0, 3.0, 2.0];
        let params = DonchianParams { period: Some(10) };
        let input = DonchianInput::from_slices(&high_data, &low_data, params);

        let result = donchian(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_donchian_very_small_data_set() {
        let high_data = [100.0];
        let low_data = [90.0];
        let params = DonchianParams { period: Some(20) };
        let input = DonchianInput::from_slices(&high_data, &low_data, params);

        let result = donchian(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_donchian_mismatched_length() {
        let high_data = [10.0, 20.0, 30.0];
        let low_data = [5.0, 3.0];
        let params = DonchianParams { period: Some(2) };
        let input = DonchianInput::from_slices(&high_data, &low_data, params);

        let result = donchian(&input);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("different lengths"));
        }
    }

    #[test]
    fn test_donchian_all_nan_data() {
        let high_data = [f64::NAN, f64::NAN];
        let low_data = [f64::NAN, f64::NAN];
        let params = DonchianParams { period: Some(2) };
        let input = DonchianInput::from_slices(&high_data, &low_data, params);

        let result = donchian(&input);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("All values are NaN"));
        }
    }

    #[test]
    fn test_donchian_partial_computation() {
        let high_data = [f64::NAN, 3.0, 5.0, 8.0, 8.5, 9.0, 2.0, 1.0];
        let low_data = [f64::NAN, 2.0, 1.0, 4.0, 4.5, 1.0, 1.0, 0.5];
        let params = DonchianParams { period: Some(3) };
        let input = DonchianInput::from_slices(&high_data, &low_data, params);

        let output = donchian(&input).expect("Donchian calculation failed");
        assert_eq!(output.upperband.len(), high_data.len());
        assert_eq!(output.middleband.len(), high_data.len());
        assert_eq!(output.lowerband.len(), high_data.len());
        assert!(output.upperband[2].is_nan());
        assert!(output.middleband[2].is_nan());
        assert!(output.lowerband[2].is_nan());
        assert!(!output.upperband[3].is_nan());
        assert!(!output.middleband[3].is_nan());
        assert!(!output.lowerband[3].is_nan());
    }
}
