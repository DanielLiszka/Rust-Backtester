use std::error::Error;

#[derive(Debug, Clone)]
pub struct KamaParams {
    pub period: Option<usize>,
}

impl Default for KamaParams {
    fn default() -> Self {
        KamaParams {
            period: Some(30),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KamaInput<'a> {
    pub data: &'a [f64],
    pub params: KamaParams,
}

impl<'a> KamaInput<'a> {
    pub fn new(data: &'a [f64], params: KamaParams) -> Self {
        KamaInput { data, params }
    }

    pub fn with_default_params(data: &'a [f64]) -> Self {
        KamaInput {
            data,
            params: KamaParams::default(),
        }
    }

    fn get_period(&self) -> usize {
        self.params.period.unwrap_or(30)
    }
}

#[derive(Debug, Clone)]
pub struct KamaOutput {
    pub values: Vec<f64>,
}

pub fn calculate_kama(input: &KamaInput) -> Result<KamaOutput, Box<dyn Error>> {
    let data = input.data;
    let len = data.len();
    let period = input.get_period();
    let mut values = vec![f64::NAN; len];
    if period > len {
        return Ok(KamaOutput { values });
    }
    let lookback = period - 1;
    if lookback >= len {
        return Ok(KamaOutput { values });
    }
    let fastest = 2.0;
    let slowest = 30.0;
    let fast_alpha = 2.0/(fastest+1.0);
    let slow_alpha = 2.0/(slowest+1.0);
    let mut sum_change = 0.0;
    for i in 1..period {
        sum_change += (data[i]-data[i-1]).abs();
    }
    let direction = (data[period-1]-data[0]).abs();
    let er = if sum_change==0.0 {0.0} else {direction/sum_change};
    let sc = (er*(fast_alpha - slow_alpha)+slow_alpha).powi(2);
    values[period-1]=data[period-1];
    let mut kama= data[period-1];
    for i in period..len {
        sum_change -= (data[i-period+1]-data[i-period]).abs();
        sum_change += (data[i]-data[i-1]).abs();
        let direction=(data[i]-data[i-(period-1)]).abs();
        let er=if sum_change==0.0 {0.0} else {direction/sum_change};
        let sc=(er*(fast_alpha - slow_alpha)+slow_alpha).powi(2);
        kama = kama + sc*(data[i]-kama);
        values[i]=kama;
    }
    Ok(KamaOutput { values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_kama_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let data = &candles.close;
        let input = KamaInput::with_default_params(data);
        let result = calculate_kama(&input).expect("Failed to calculate KAMA");
        let expected_last_five = vec![
            60234.925553804125,
            60176.838757545665,
            60115.177367962766,
            60071.37070833558,
            59992.79386218023,
        ];
        assert!(result.values.len()>=5);
        let start_index=result.values.len().saturating_sub(5);
        let last_five=&result.values[start_index..];
        for (i,&val) in last_five.iter().enumerate() {
            let exp=expected_last_five[i];
            assert!((val-exp).abs()<1e-6,"KAMA mismatch at {}: expected {}, got {}",i,exp,val);
        }
    }
}
