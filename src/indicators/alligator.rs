use crate::indicators::data_loader::Candles;
use std::error::Error;

pub struct Alligator {
    pub jaw: Vec<f64>,
    pub teeth: Vec<f64>,
    pub lips: Vec<f64>,
}

pub fn calculate_alligator(data: &[f64]) -> Result<Alligator, Box<dyn Error>> {
    const JAW_PERIOD: usize = 13;
    const JAW_OFFSET: usize = 8;
    const TEETH_PERIOD: usize = 8;
    const TEETH_OFFSET: usize = 5;
    const LIPS_PERIOD: usize = 5;
    const LIPS_OFFSET: usize = 3;

    let len: usize = data.len();

    let mut jaw: Vec<f64> = vec![f64::NAN; len];
    let mut teeth: Vec<f64> = vec![f64::NAN; len];
    let mut lips: Vec<f64> = vec![f64::NAN; len];

    let mut jaw_sum: f64 = 0.0;
    let mut teeth_sum: f64 = 0.0;
    let mut lips_sum: f64 = 0.0;

    let mut jaw_smma: Option<f64> = None;
    let mut teeth_smma: Option<f64> = None;
    let mut lips_smma: Option<f64> = None;

    let jaw_scale = (JAW_PERIOD - 1) as f64;
    let jaw_inv_period = 1.0 / JAW_PERIOD as f64;

    let teeth_scale = (TEETH_PERIOD - 1) as f64;
    let teeth_inv_period = 1.0 / TEETH_PERIOD as f64;

    let lips_scale = (LIPS_PERIOD - 1) as f64;
    let lips_inv_period = 1.0 / LIPS_PERIOD as f64;

    for i in 0..len {
        let data_point = data[i];

        if i < JAW_PERIOD {
            jaw_sum += data_point;
            if i == JAW_PERIOD - 1 {
                jaw_smma = Some(jaw_sum / JAW_PERIOD as f64);
                let shifted_index = i + JAW_OFFSET;
                if shifted_index < len {
                    jaw[shifted_index] = jaw_smma.unwrap();
                }
            }
        } else {
            if let Some(prev_smma) = jaw_smma {
                let new_smma = (prev_smma * jaw_scale + data_point) * jaw_inv_period;
                jaw_smma = Some(new_smma);
                let shifted_index = i + JAW_OFFSET;
                if shifted_index < len {
                    jaw[shifted_index] = new_smma;
                }
            }
        }

        if i < TEETH_PERIOD {
            teeth_sum += data_point;
            if i == TEETH_PERIOD - 1 {
                teeth_smma = Some(teeth_sum / TEETH_PERIOD as f64);
                let shifted_index = i + TEETH_OFFSET;
                if shifted_index < len {
                    teeth[shifted_index] = teeth_smma.unwrap();
                }
            }
        } else {
            if let Some(prev_smma) = teeth_smma {
                let new_smma = (prev_smma * teeth_scale + data_point) * teeth_inv_period;
                teeth_smma = Some(new_smma);
                let shifted_index = i + TEETH_OFFSET;
                if shifted_index < len {
                    teeth[shifted_index] = new_smma;
                }
            }
        }

        if i < LIPS_PERIOD {
            lips_sum += data_point;
            if i == LIPS_PERIOD - 1 {
                lips_smma = Some(lips_sum / LIPS_PERIOD as f64);
                let shifted_index = i + LIPS_OFFSET;
                if shifted_index < len {
                    lips[shifted_index] = lips_smma.unwrap();
                }
            }
        } else {
            if let Some(prev_smma) = lips_smma {
                let new_smma = (prev_smma * lips_scale + data_point) * lips_inv_period;
                lips_smma = Some(new_smma);
                let shifted_index = i + LIPS_OFFSET;
                if shifted_index < len {
                    lips[shifted_index] = new_smma;
                }
            }
        }
    }

    Ok(Alligator { jaw, teeth, lips })
}
mod tests {
    use super::*;
    use crate::indicators::data_loader::{load_test_candles, Candles};

    #[test]
    fn test_alligator_accuracy() {
        let candles = load_test_candles().expect("Failed to load test candles");
        let hl2_prices: Vec<f64> = candles
            .get_calculated_field("hl2")
            .expect("Failed to extract hl2 prices");
        let result: Alligator =
            calculate_alligator(&hl2_prices).expect("Failed to calculate alligator");

        let expected_last_five_jaw_result = vec![60742.4, 60632.6, 60555.1, 60442.7, 60308.7];

        let expected_last_five_teeth_result = vec![59908.0, 59757.2, 59684.3, 59653.5, 59621.1];

        let expected_last_five_lips_result = vec![59355.2, 59371.7, 59376.2, 59334.1, 59316.2];

        let start_index: usize = result.jaw.len() - 5;
        let result_last_five_jaws = &result.jaw[start_index..];
        let result_last_five_teeth = &result.teeth[start_index..];
        let result_last_five_lips = &result.lips[start_index..];

        for (i, &value) in result_last_five_jaws.iter().enumerate() {
            let expected_value = expected_last_five_jaw_result[i];
            assert!(
                (value - expected_value).abs() < 1e-1,
                "alligator jaw value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }

        for (i, &value) in result_last_five_teeth.iter().enumerate() {
            let expected_value = expected_last_five_teeth_result[i];
            assert!(
                (value - expected_value).abs() < 1e-1,
                "alligator teeth value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }

        for (i, &value) in result_last_five_lips.iter().enumerate() {
            let expected_value = expected_last_five_lips_result[i];
            assert!(
                (value - expected_value).abs() < 1e-1,
                "alligator lips value mismatch at index {}: expected {}, got {}",
                i,
                expected_value,
                value
            );
        }
    }
}