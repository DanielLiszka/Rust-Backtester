use std::error::Error;

#[derive(Debug, Clone)]
pub struct DemaParams {
    pub period: Option<usize>,
}

impl Default for DemaParams {
    fn default() -> Self {
        DemaParams {
            period: Some(30), // default period = 30
        }
    }
}

#[derive(Debug, Clone)]
pub struct DemaInput<'a> {
    pub data: &'a [f64],
    pub params: DemaParams,
}

impl<'a> DemaInput<'a> {
    pub fn new(data: &'a [f64], params: DemaParams) -> Self {
        DemaInput { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        DemaInput {
            data,
            params: DemaParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params.period.unwrap_or(30)
    }
}

#[derive(Debug, Clone)]
pub struct DemaOutput {
    pub values: Vec<f64>,
}

pub fn calculate_dema(input: &DemaInput) -> Result<DemaOutput, Box<dyn Error>> {
    let data = input.data;
    let len = data.len();
    let period = input.get_period();

    if period == 0 {
        return Err("Period cannot be zero for DEMA calculation.".into());
    }

    let mut values = vec![f64::NAN; len];
    if period > len {
        // Not enough data for even one EMA
        return Ok(DemaOutput { values });
    }

    // Precompute constants
    let period_f = period as f64;
    let alpha = 2.0/(period_f+1.0);
    let lookback_ema = period - 1;
    let lookback_total = lookback_ema*2;

    // first_ema and second_ema arrays
    let mut first_ema = vec![f64::NAN; len];
    let mut second_ema = vec![f64::NAN; len];

    // Accumulators for first EMA initialization (SMA)
    let mut sum_first = 0.0;
    let mut prev_first_ema = f64::NAN;

    // For second EMA initialization
    let mut sum_second = 0.0;
    let mut prev_second_ema = f64::NAN;
    let mut second_ema_init_done = false;

    // Indices for clarity
    let start_first_ema = lookback_ema;      // i=period-1
    let start_second_ema_init = start_first_ema; 
    let start_second_ema_stable = lookback_total; // i=2*(period-1)

    for i in 0..len {
        let val = data[i];

        // Compute first EMA
        if i < period {
            sum_first += val;
            if i == lookback_ema {
                // initial SMA for first EMA
                let sma = sum_first/period_f;
                first_ema[i] = sma;
                prev_first_ema = sma;
            }
        } else {
            // EMA formula after initial SMA
            let ema_val = (val - prev_first_ema)*alpha + prev_first_ema;
            first_ema[i] = ema_val;
            prev_first_ema = ema_val;
        }

        // Once we have first_ema stable (i≥period-1), start accumulating for second EMA init.
        if i >= start_first_ema && i < start_first_ema+period {
            let fe_val = first_ema[i];
            sum_second += fe_val;
            if i == start_first_ema+period-1 {
                // second EMA initial SMA at i=(period-1)+(period-1)=2*(period-1)
                let sma_second = sum_second/period_f;
                second_ema[i] = sma_second;
                prev_second_ema = sma_second;
                second_ema_init_done = true;
            }
        } else if i > start_first_ema+period-1 && second_ema_init_done {
            // After initial SMA for second EMA, apply EMA formula
            let fe_val = first_ema[i];
            let ema_val = (fe_val - prev_second_ema)*alpha + prev_second_ema;
            second_ema[i] = ema_val;
            prev_second_ema = ema_val;
        }

        // Compute DEMA if i≥2*(period-1)
        if i >= start_second_ema_stable {
            let fe = first_ema[i];
            let se = second_ema[i];
            // If both are ready, compute DEMA
            if fe.is_finite() && se.is_finite() {
                values[i] = 2.0*fe - se;
            }
        }
    }

    Ok(DemaOutput { values })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_dema_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let data = &candles.close;

        let input = DemaInput::with_default_params(data);
        let result = calculate_dema(&input).expect("Failed to calculate DEMA");

        // Provided test values for the last 5 DEMA:
        // 59189.73193987478
        // 59129.24920772847
        // 59058.80282420511
        // 59011.5555611042
        // 58908.370159946775
        let expected_last_five = vec![
            59189.73193987478,
            59129.24920772847,
            59058.80282420511,
            59011.5555611042,
            58908.370159946775,
        ];

        assert!(result.values.len() >= 5);
        let start_index = result.values.len().saturating_sub(5);
        let last_five = &result.values[start_index..];
        for (i, &val) in last_five.iter().enumerate() {
            let exp = expected_last_five[i];
            assert!(
                (val - exp).abs() < 1e-6,
                "DEMA mismatch at {}: expected {}, got {}",
                i, exp, val
            );
        }

        // Some initial values are NaN until stable. This is expected.
        // Once stable (after 2*(period-1) bars), DEMA matches expected values.
    }
}
