use std::error::Error;

#[derive(Debug, Clone)]
pub struct T3Params {
    pub period: Option<usize>,
    pub volume_factor: Option<f64>,
}

impl Default for T3Params {
    fn default() -> Self {
        T3Params {
            period: Some(5),
            volume_factor: Some(0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct T3Input<'a> {
    pub data: &'a [f64],
    pub params: T3Params,
}

impl<'a> T3Input<'a> {
    pub fn new(data: &'a [f64], params: T3Params) -> Self {
        T3Input { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        T3Input {
            data,
            params: T3Params::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params.period.unwrap_or(14)
    }

    fn get_volume_factor(&self) -> f64 {
        self.params.volume_factor.unwrap_or(0.7)
    }
}

#[derive(Debug, Clone)]
pub struct T3Output {
    pub values: Vec<f64>,
}

#[inline]
pub fn calculate_t3(input: &T3Input) -> Result<T3Output, Box<dyn Error>> {
    let data = input.data;
    let p = input.get_period();
    let v = input.get_volume_factor();
    if p == 0 || p > data.len() {
        return Err("Invalid period specified.".into());
    }

    let c1 = -v * v * v;
    let c2 = 3.0 * v * v + 3.0 * v * v * v;
    let c3 = -6.0 * v * v - 3.0 * v - 3.0 * v * v * v;
    let c4 = 1.0 + 3.0 * v + 3.0 * v * v + v * v * v;

    let mut e1 = Vec::with_capacity(data.len());
    let mut e2 = Vec::with_capacity(data.len());
    let mut e3 = Vec::with_capacity(data.len());
    let mut e4 = Vec::with_capacity(data.len());
    let mut e5 = Vec::with_capacity(data.len());
    let mut e6 = Vec::with_capacity(data.len());

    let alpha = 2.0 / (p as f64 + 1.0);

    {
        let mut ema = data[0];
        e1.push(ema);
        for &val in &data[1..] {
            ema = (val - ema) * alpha + ema;
            e1.push(ema);
        }
    }

    {
        let mut ema = e1[0];
        e2.push(ema);
        for &val in &e1[1..] {
            ema = (val - ema) * alpha + ema;
            e2.push(ema);
        }
    }

    {
        let mut ema = e2[0];
        e3.push(ema);
        for &val in &e2[1..] {
            ema = (val - ema) * alpha + ema;
            e3.push(ema);
        }
    }

    {
        let mut ema = e3[0];
        e4.push(ema);
        for &val in &e3[1..] {
            ema = (val - ema) * alpha + ema;
            e4.push(ema);
        }
    }

    {
        let mut ema = e4[0];
        e5.push(ema);
        for &val in &e4[1..] {
            ema = (val - ema) * alpha + ema;
            e5.push(ema);
        }
    }

    {
        let mut ema = e5[0];
        e6.push(ema);
        for &val in &e5[1..] {
            ema = (val - ema) * alpha + ema;
            e6.push(ema);
        }
    }

    let len = data.len();
    let mut t3_values = Vec::with_capacity(len);
    for i in 0..len {
        if i < (p * 5) {
            t3_values.push(std::f64::NAN);
        } else {
            t3_values.push(
                e6[i] * c1 +
                e5[i] * c2 +
                e4[i] * c3 +
                e3[i] * c4
            );
        }
    }

    Ok(T3Output {
        values: t3_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::{read_candles_from_csv};

    #[test]
    fn test_t3_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = T3Params {
            period: Some(5),
            volume_factor: Some(0.0),
        };

        let input = T3Input::new(&close_prices, params);
        let t3_result = calculate_t3(&input).expect("Failed to calculate T3");

        let expected_last_five_t3 = vec![
            59304.716332473254,
            59283.56868015526,
            59261.16173577631,
            59240.25895948583,
            59203.544843167765,
        ];

        assert!(t3_result.values.len() >= 5);
        let start_index = t3_result.values.len() - 5;
        let result_last_five_t3 = &t3_result.values[start_index..];

        for (i, &value) in result_last_five_t3.iter().enumerate() {
            let expected_value = expected_last_five_t3[i];
            assert!(
                (value - expected_value).abs() < 1e-10,
                "T3 value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }

        let default_input = T3Input::with_default_params(&close_prices);
        let default_t3_result = calculate_t3(&default_input)
            .expect("Failed to calculate T3 with defaults");
        assert!(!default_t3_result.values.is_empty());
    }
}
