use std::error::Error;

#[derive(Debug, Clone)]
pub struct MwdxParams {
    pub factor: Option<f64>,
}

impl Default for MwdxParams {
    fn default() -> Self {
        MwdxParams {
            factor: Some(0.2),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MwdxInput<'a> {
    pub data: &'a [f64],
    pub params: MwdxParams,
}

impl<'a> MwdxInput<'a> {
    #[inline]
    pub fn new(data: &'a [f64], params: MwdxParams) -> Self {
        Self { data, params }
    }

    #[inline]
    pub fn with_default_params(data: &'a [f64]) -> Self {
        Self {
            data,
            params: MwdxParams::default(),
        }
    }

    #[inline]
    fn get_factor(&self) -> f64 {
        self.params
            .factor
            .unwrap_or_else(|| MwdxParams::default().factor.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct MwdxOutput {
    pub values: Vec<f64>,
}

#[inline]
pub fn calculate_mwdx(input: &MwdxInput) -> Result<MwdxOutput, Box<dyn Error>> {
    let data = input.data;
    let n = data.len();
    if n == 0 {
        return Err("Empty data slice for MWDX calculation.".into());
    }

    let factor = input.get_factor();
    if factor <= 0.0 {
        return Err("Factor must be > 0 for MWDX.".into());
    }

    let val2 = (2.0 / factor) - 1.0;
    let fac = 2.0 / (val2 + 1.0);

    let mut output = Vec::with_capacity(n);
    output.extend_from_slice(data);

    for i in 1..n {
        output[i] = fac * data[i] + (1.0 - fac) * output[i - 1];
    }

    Ok(MwdxOutput { values: output })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::data_loader::read_candles_from_csv;

    #[test]
    fn test_mwdx_accuracy() {
        let expected_last_five = [
            59302.181566190935,
            59277.94525295275,
            59230.1562023622,
            59215.124961889764,
            59103.099969511815,
        ];

        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");

        let source = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = MwdxParams { factor: Some(0.2) };
        let input = MwdxInput::new(source, params);

        let result = calculate_mwdx(&input).expect("Failed to calculate MWDX");
        assert_eq!(result.values.len(), source.len(), "Output length mismatch");

        assert!(
            result.values.len() >= 5,
            "Not enough data to compare last 5 MWDX values"
        );
        let start_idx = result.values.len() - 5;
        let actual_last_five = &result.values[start_idx..];

        for (i, &val) in actual_last_five.iter().enumerate() {
            let exp_val = expected_last_five[i];
            assert!(
                (val - exp_val).abs() < 1e-5,
                "MWDX mismatch at index {}, expected {}, got {}",
                i,
                exp_val,
                val
            );
        }
    }
}