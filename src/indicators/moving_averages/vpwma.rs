use std::error::Error;
#[derive(Debug, Clone)]
pub struct VpwmaParams {
    pub period: Option<usize>,
    pub power: Option<f64>,
}

impl Default for VpwmaParams {
    fn default() -> Self {
        VpwmaParams {
            period: Some(14),
            power: Some(0.382),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VpwmaInput<'a> {
    pub data: &'a [f64],
    pub params: VpwmaParams,
}

impl<'a> VpwmaInput<'a> {
    #[inline(always)]
    pub fn new(data: &'a [f64], params: VpwmaParams) -> Self {
        VpwmaInput { data, params }
    }

    #[inline(always)]
    pub fn with_default_params(data: &'a [f64]) -> Self {
        VpwmaInput {
            data,
            params: VpwmaParams::default(),
        }
    }

    #[inline(always)]
    fn get_period(&self) -> usize {
        self.params
            .period
            .unwrap_or_else(|| VpwmaParams::default().period.unwrap())
    }

    #[inline(always)]
    fn get_power(&self) -> f64 {
        self.params
            .power
            .unwrap_or_else(|| VpwmaParams::default().power.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct VpwmaOutput {
    pub values: Vec<f64>,
}

#[inline(always)]
pub fn calculate_vpwma(input: &VpwmaInput) -> Result<VpwmaOutput, Box<dyn Error>> {
    let data = input.data;
    let period = input.get_period();
    let power = input.get_power();

    if data.len() < (period + 1) {
        return Err(format!(
            "Not enough data: length {} < period+1={}",
            data.len(),
            period + 1
        )
        .into());
    }
    if period < 2 {
        return Err("VPWMA period must be >= 2.".into());
    }
    if power.is_nan() {
        return Err("VPWMA power cannot be NaN.".into());
    }

    let len = data.len();
    let mut vpwma_values = data.to_vec();

    let mut weights = Vec::with_capacity(period - 1);
    for i in 0..(period - 1) {
        let w = (period as f64 - i as f64).powf(power);
        weights.push(w);
    }
    let weight_sum: f64 = weights.iter().sum();

    for j in (period + 1)..len {
        let mut my_sum = 0.0;
        for (i, &w) in weights.iter().enumerate() {
            my_sum = data[j - i].mul_add(w, my_sum);
        }
        vpwma_values[j] = my_sum / weight_sum;
    }

    Ok(VpwmaOutput {
        values: vpwma_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_vpwma_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = VpwmaParams {
            period: Some(14),
            power: Some(0.382),
        };
        let input = VpwmaInput::new(close_prices, params);
        let result = calculate_vpwma(&input).expect("Failed to calculate VPWMA");

        assert_eq!(
            result.values.len(),
            close_prices.len(),
            "VPWMA output length should match input length"
        );

        let expected_last_five = [
            59363.927599446455,
            59296.83894519251,
            59196.82476139941,
            59180.8040249446,
            59113.84473799056,
        ];
        let start_index = result.values.len().saturating_sub(5);
        let actual_last_five = &result.values[start_index..];

        for (i, &val) in actual_last_five.iter().enumerate() {
            let exp = expected_last_five[i];
            let diff = (val - exp).abs();
            assert!(
                diff < 1e-2,
                "VPWMA mismatch at last-5 index {}: expected {}, got {}",
                i,
                exp,
                val
            );
        }
    }

    #[test]
    fn test_vpwma_with_defaults() {
        let data = vec![64000.0, 64010.0, 63990.0, 64020.0, 64030.0];
        let input = VpwmaInput::with_default_params(&data);
        let result = calculate_vpwma(&input);
        assert!(result.is_err(), "Should fail due to insufficient data");
    }
}