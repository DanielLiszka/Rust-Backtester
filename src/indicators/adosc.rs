use std::error::Error;
use crate::indicators::data_loader::Candles;

#[derive(Debug, Clone)]
pub struct AdoscParams {
    pub short_period: Option<usize>,
    pub long_period: Option<usize>,
}

impl Default for AdoscParams {
    fn default() -> Self {
        AdoscParams {
            short_period: Some(3),
            long_period: Some(10),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdoscInput<'a> {
    pub candles: &'a Candles,
    pub params: AdoscParams,
}

impl<'a> AdoscInput<'a> {
    pub fn new(candles: &'a Candles, params: AdoscParams) -> Self {
        AdoscInput { candles, params }
    }

    pub fn with_default_params(candles: &'a Candles) -> Self {
        AdoscInput {
            candles,
            params: AdoscParams::default(),
        }
    }

    fn get_short_period(&self) -> usize {
        self.params
            .short_period
            .unwrap_or_else(|| AdoscParams::default().short_period.unwrap())
    }

    fn get_long_period(&self) -> usize {
        self.params
            .long_period
            .unwrap_or_else(|| AdoscParams::default().long_period.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct AdoscOutput {
    pub values: Vec<f64>,
}

pub fn calculate_adosc(input: &AdoscInput) -> Result<AdoscOutput, Box<dyn Error>> {
    let candles = input.candles;
    let short = input.get_short_period();
    let long = input.get_long_period();

    if short == 0 || long == 0 {
        return Err("Invalid period specified for ADOSC calculation.".into());
    }

    if short >= long {
        return Err("Short period must be less than the long period for ADOSC.".into());
    }

    let len = candles.close.len();
    if len == 0 {
        return Err("No candles available.".into());
    }

    if long > len {
        return Err("Not enough data points to calculate ADOSC.".into());
    }

    let high = candles.select_candle_field("high")?;
    let low = candles.select_candle_field("low")?;
    let close = candles.select_candle_field("close")?;
    let volume = candles.select_candle_field("volume")?;

    let alpha_short = 2.0 / (short as f64 + 1.0);
    let alpha_long = 2.0 / (long as f64 + 1.0);

    let mut adosc_values = vec![0.0; len];
    let mut sum_ad = 0.0;

    // Initialize with the first candle
    {
        let h = high[0];
        let l = low[0];
        let c = close[0];
        let v = volume[0];
        let hl = h - l;
        let mfm = if hl != 0.0 {
            ((c - l) - (h - c)) / hl
        } else {
            0.0
        };
        let mfv = mfm * v;
        sum_ad += mfv;

        // Initial EMAs
        let short_ema = sum_ad;
        let long_ema = sum_ad;
        let adosc = short_ema - long_ema;
        adosc_values[0] = adosc;

        // Move variables out of block scope
        // We'll reuse sum_ad. We need to keep track of short_ema and long_ema across iterations.
        // We'll store them mutably here.
        let mut short_ema = short_ema;
        let mut long_ema = long_ema;

        // Now start main loop from i=1
        for i in 1..len {
            let h = high[i];
            let l = low[i];
            let c = close[i];
            let v = volume[i];

            let hl = h - l;
            let mfm = if hl != 0.0 {
                ((c - l) - (h - c)) / hl
            } else {
                0.0
            };
            let mfv = mfm * v;
            sum_ad += mfv;

            // Update short EMA
            short_ema = alpha_short * sum_ad + (1.0 - alpha_short) * short_ema;
            // Update long EMA
            long_ema = alpha_long * sum_ad + (1.0 - alpha_long) * long_ema;

            let adosc = short_ema - long_ema;
            adosc_values[i] = adosc;
        }
    }

    Ok(AdoscOutput { values: adosc_values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::{read_candles_from_csv, Candles};

    #[test]
    fn test_adosc_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        // Use default parameters (short=3, long=10) or specify if needed:
        let input = AdoscInput::with_default_params(&candles);
        let result = calculate_adosc(&input).expect("Failed to calculate ADOSC");

        // The expected last five ADOSC values provided by you:
        let expected_last_five = vec![-166.2175, -148.9983, -144.9052, -128.5921, -142.0772];

        assert!(
            result.values.len() >= 5,
            "Not enough ADOSC values for the test"
        );

        let start_index = result.values.len().saturating_sub(5);
        let result_last_five = &result.values[start_index..];

        for (i, &value) in result_last_five.iter().enumerate() {
            assert!(
                (value - expected_last_five[i]).abs() < 1e-1,
                "ADOSC value mismatch at index {}: expected {}, got {}",
                i,
                expected_last_five[i],
                value
            );
        }

        // Check that all values are finite
        for val in result.values.iter() {
            assert!(val.is_finite(), "ADOSC output should be finite");
        }
    }
}