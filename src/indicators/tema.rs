use std::error::Error;

#[derive(Debug, Clone)]
pub struct TemaParams {
    pub period: Option<usize>,
}

impl Default for TemaParams {
    fn default() -> Self {
        TemaParams { period: Some(9) }
    }
}

#[derive(Debug, Clone)]
pub struct TemaInput<'a> {
    pub data: &'a [f64],
    pub params: TemaParams,
}

impl<'a> TemaInput<'a> {
    pub fn new(data: &'a [f64], params: TemaParams) -> Self {
        TemaInput { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        TemaInput {
            data,
            params: TemaParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params.period.unwrap_or(9)
    }
}

#[derive(Debug, Clone)]
pub struct TemaOutput {
    pub values: Vec<f64>,
}

pub fn calculate_tema(input: &TemaInput) -> Result<TemaOutput, Box<dyn Error>> {
    let data = input.data;
    let len = data.len();
    let period = input.get_period();

    if period == 0 || period > len {
        return Err("Invalid period specified for TEMA calculation.".into());
    }

    let lookback_ema = period - 1;
    let lookback_total = 3 * lookback_ema;

    // Arrays for each EMA and final TEMA
    let mut ema1 = vec![f64::NAN; len];
    let mut ema2 = vec![f64::NAN; len];
    let mut ema3 = vec![f64::NAN; len];
    let mut tema_values = vec![f64::NAN; len];

    // EMA alpha
    let alpha = 2.0 / (period as f64 + 1.0);

    // Initialize for first EMA
    let mut sum_ema1 = 0.0;
    let mut prev_ema1 = f64::NAN;

    // For second EMA initialization
    let mut sum_ema2 = 0.0;
    let mut prev_ema2 = f64::NAN;
    let mut ema2_init_done = false;

    // For third EMA initialization
    let mut sum_ema3 = 0.0;
    let mut prev_ema3 = f64::NAN;
    let mut ema3_init_done = false;

    for i in 0..len {
        let price = data[i];

        // Compute first EMA
        if i < period {
            sum_ema1 += price;
            if i == period - 1 {
                let sma = sum_ema1 / period as f64;
                ema1[i] = sma;
                prev_ema1 = sma;
            }
        } else {
            let ema_val = (price - prev_ema1) * alpha + prev_ema1;
            ema1[i] = ema_val;
            prev_ema1 = ema_val;
        }

        // Once first EMA stable at i=period-1, start second EMA accumulation
        if i >= (period - 1) && i < (period - 1) + period {
            let val = ema1[i];
            if val.is_finite() {
                sum_ema2 += val;
            }
            if i == (period - 1) + (period - 1) {
                // second EMA initial SMA
                let sma2 = sum_ema2 / period as f64;
                ema2[i] = sma2;
                prev_ema2 = sma2;
                ema2_init_done = true;
            }
        } else if i > (period - 1) + (period - 1) && ema2_init_done {
            // second EMA
            let val = ema1[i];
            let ema_val = (val - prev_ema2) * alpha + prev_ema2;
            ema2[i] = ema_val;
            prev_ema2 = ema_val;
        }

        // Once second EMA stable at i=2*(period-1), start third EMA accumulation
        if i >= 2 * (period - 1) && i < 2*(period - 1) + period {
            let val = ema2[i];
            if val.is_finite() {
                sum_ema3 += val;
            }
            if i == 3*(period - 1) {
                // third EMA initial SMA
                let sma3 = sum_ema3 / period as f64;
                ema3[i] = sma3;
                prev_ema3 = sma3;
                ema3_init_done = true;
            }
        } else if i > 3*(period - 1) && ema3_init_done {
            // third EMA
            let val = ema2[i];
            let ema_val = (val - prev_ema3)*alpha + prev_ema3;
            ema3[i] = ema_val;
            prev_ema3 = ema_val;
        }

        // TEMA stable at i≥3*(period-1)
        if i >= 3*(period - 1) && ema3_init_done {
            let fe = ema1[i];
            let se = ema2[i];
            let te = ema3[i];
            if fe.is_finite() && se.is_finite() && te.is_finite() {
                let tema = 3.0 * fe - 3.0 * se + te;
                tema_values[i] = tema;
            }
        }
    }

    Ok(TemaOutput {
        values: tema_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_tema_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = TemaParams { period: Some(9) };
        let input = TemaInput::new(&close_prices, params);
        let tema_result = calculate_tema(&input).expect("Failed to calculate TEMA");

        let expected_last_five_tema = vec![
            59281.895570662884,
            59257.25021607971,
            59172.23342859784,
            59175.218345941066,
            58934.24395798363,
        ];

        assert!(tema_result.values.len() >= 5);
        let start_index = tema_result.values.len() - 5;
        let result_last_five_tema = &tema_result.values[start_index..];

        for (i, &value) in result_last_five_tema.iter().enumerate() {
            let expected_value = expected_last_five_tema[i];
            assert!(
                (value - expected_value).abs() < 1e-8,
                "TEMA value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }
    }
}
