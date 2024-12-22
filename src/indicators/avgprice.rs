use crate::indicators::data_loader::Candles;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AvgPriceParams;

impl Default for AvgPriceParams {
    fn default() -> Self {
        AvgPriceParams
    }
}

#[derive(Debug, Clone)]
pub struct AvgPriceInput<'a> {
    pub candles: &'a Candles,
    pub params: AvgPriceParams,
}

impl<'a> AvgPriceInput<'a> {
    pub fn new(candles: &'a Candles, params: AvgPriceParams) -> Self {
        AvgPriceInput { candles, params }
    }

    pub fn with_default_params(candles: &'a Candles) -> Self {
        AvgPriceInput {
            candles,
            params: AvgPriceParams,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AvgPriceOutput {
    pub values: Vec<f64>,
}

pub fn calculate_avgprice(input: &AvgPriceInput) -> Result<AvgPriceOutput, Box<dyn Error>> {
    let candles = input.candles;
    let len = candles.close.len();
    if len == 0 {
        return Err("No candles available.".into());
    }

    let open = candles.select_candle_field("open")?;
    let high = candles.select_candle_field("high")?;
    let low = candles.select_candle_field("low")?;
    let close = candles.select_candle_field("close")?;

    let mut values = Vec::with_capacity(len);
    for i in 0..len {
        let sum = open[i] + high[i] + low[i] + close[i];
        values.push(sum / 4.0);
    }

    Ok(AvgPriceOutput { values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::Candles;

    #[test]
    fn test_avgprice_accuracy() {
        // We'll create a mock Candles dataset for testing.
        // Using the sample data from above:
        // Candle 1: O=100,H=110,L=90,C=105 => 101.25
        // Candle 2: O=101,H=111,L=91,C=106 => 102.25
        // Candle 3: O=102,H=112,L=92,C=107 => 103.25
        // Candle 4: O=103,H=113,L=93,C=108 => 104.25
        // Candle 5: O=104,H=114,L=94,C=109 => 105.25

        let candles = Candles {
            timestamp: vec![1., 2., 3., 4., 5.],
            open: vec![100., 101., 102., 103., 104.],
            high: vec![110., 111., 112., 113., 114.],
            low: vec![90., 91., 92., 93., 94.],
            close: vec![105., 106., 107., 108., 109.],
            volume: vec![1000., 1000., 1000., 1000., 1000.],
        };

        let input = AvgPriceInput::with_default_params(&candles);
        let result = calculate_avgprice(&input).expect("Failed to calculate avgprice");

        let expected = [101.25, 102.25, 103.25, 104.25, 105.25];

        assert_eq!(result.values.len(), 5);
        for (i, &val) in result.values.iter().enumerate() {
            assert!(
                (val - expected[i]).abs() < 1e-2,
                "Mismatch at index {}: expected {}, got {}",
                i,
                expected[i],
                val
            );
        }
    }
}
