use std::error::Error;

#[derive(Debug, Clone)]
pub struct TemaParams {
    pub period: Option<usize>,
}

impl Default for TemaParams {
    fn default() -> Self {
        // Default to a 9-period TEMA if not specified
        TemaParams { period: Some(9) }
    }
}

#[derive(Debug, Clone)]
pub struct TemaInput<'a> {
    pub data: &'a [f64],
    pub params: TemaParams,
}

impl<'a> TemaInput<'a> {
    #[inline]
    pub fn new(data: &'a [f64], params: TemaParams) -> Self {
        TemaInput { data, params }
    }

    #[inline]
    pub fn with_default_params(data: &'a [f64]) -> Self {
        TemaInput {
            data,
            params: TemaParams::default(),
        }
    }

    #[inline]
    fn get_period(&self) -> usize {
        self.params.period.unwrap_or_else(|| TemaParams::default().period.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct TemaOutput {
    pub values: Vec<f64>,
}

#[inline]
fn compute_ema(
    input_data: &[f64],
    period: usize,
    output: &mut Vec<f64>,
) {
    // Initialization: simple average of first `period` points
    let sum: f64 = input_data[..period].iter().sum();
    let initial = sum / period as f64;
    output.push(initial);

    let alpha = 2.0 / (period as f64 + 1.0);

    // Compute EMA
    let mut prev_ema = initial;
    for &value in &input_data[period..] {
        let new_ema = (value - prev_ema) * alpha + prev_ema;
        output.push(new_ema);
        prev_ema = new_ema;
    }
}

#[inline]
pub fn calculate_tema(input: &TemaInput) -> Result<TemaOutput, Box<dyn Error>> {
    let data = input.data;
    let period = input.get_period();

    if period == 0 || period > data.len() {
        return Err("Invalid period specified for TEMA calculation.".into());
    }

    // Length calculations:
    // ema1_len = data.len() - (period - 1)
    // ema2_len = ema1_len - (period - 1) = data.len() - 2*(period - 1)
    // ema3_len = ema2_len - (period - 1) = data.len() - 3*(period - 1)
    let ema1_len = data.len() - period + 1;
    let ema2_len = ema1_len - period + 1;
    let ema3_len = ema2_len - period + 1;

    // Pre-allocate vectors with exact capacity
    let mut ema1 = Vec::with_capacity(ema1_len);
    let mut ema2 = Vec::with_capacity(ema2_len);
    let mut ema3 = Vec::with_capacity(ema3_len);

    // Compute EMA1 directly from data
    compute_ema(data, period, &mut ema1);

    // Compute EMA2 from EMA1
    compute_ema(&ema1, period, &mut ema2);

    // Compute EMA3 from EMA2
    compute_ema(&ema2, period, &mut ema3);

    // Construct TEMA
    // TEMA[i] = 3*ema1[i+2*(period-1)] - 3*ema2[i+(period-1)] + ema3[i]
    let shift_ema1 = 2 * (period - 1);
    let shift_ema2 = (period - 1);

    let mut tema_values = Vec::with_capacity(ema3_len);
    for i in 0..ema3_len {
        let val = 3.0 * ema1[i + shift_ema1] - 3.0 * ema2[i + shift_ema2] + ema3[i];
        tema_values.push(val);
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

        assert!(
            tema_result.values.len() >= 5,
            "Not enough TEMA values for the test"
        );

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
