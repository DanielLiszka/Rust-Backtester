use std::error::Error;

#[derive(Debug, Clone)]
pub struct MamaParams {
    pub fast_limit: f64,
    pub slow_limit: f64,
}

impl Default for MamaParams {
    fn default() -> Self {
        MamaParams {
            fast_limit: 0.5,
            slow_limit: 0.05,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MamaInput<'a> {
    pub data: &'a [f64],
    pub params: MamaParams,
}

impl<'a> MamaInput<'a> {
    #[inline]
    pub fn new(data: &'a [f64], params: MamaParams) -> Self {
        MamaInput { data, params }
    }

    #[inline]
    pub fn with_default_params(data: &'a [f64]) -> Self {
        MamaInput {
            data,
            params: MamaParams::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MamaOutput {
    pub mama_values: Vec<f64>,
    pub fama_values: Vec<f64>,
}

#[inline]
pub fn calculate_mama(input: &MamaInput) -> Result<MamaOutput, Box<dyn Error>> {
    let data = input.data;
    let fast_limit = input.params.fast_limit;
    let slow_limit = input.params.slow_limit;
    if data.len() < 50 {
        return Err("Not enough data".into());
    }

    let mut mama_values = Vec::with_capacity(data.len());
    let mut fama_values = Vec::with_capacity(data.len());
    let mut smooth_price = [0.0; 50];
    let mut detrender_odd = [0.0; 3];
    let mut detrender_even = [0.0; 3];
    let mut q1_odd = [0.0; 3];
    let mut q1_even = [0.0; 3];
    let mut jI_odd = [0.0; 3];
    let mut jI_even = [0.0; 3];
    let mut jQ_odd = [0.0; 3];
    let mut jQ_even = [0.0; 3];
    let mut period = 0.0;
    let mut smooth = 0.0;
    let mut i1_for_even = 0.0;
    let mut i1_for_odd = 0.0;
    let mut q2 = 0.0;
    let mut i2 = 0.0;
    let mut re = 0.0;
    let mut im = 0.0;
    let mut prev_phase = 0.0;
    let mut mama = 0.0;
    let mut fama = 0.0;
    let mut result_mama = Vec::new();
    let mut result_fama = Vec::new();
    let mut hilbert_idx = 0;
    let mut trailing_wpr = 0.0;
    let mut odd = false;

    let mut smooth_price_idx = 0;
    let mut today = 0;
    let mut adjusted_prev_period = 0.0;
    let mut i1 = 0.0;
    let mut q1 = 0.0;
    let mut jI = 0.0;
    let mut jQ = 0.0;

    let mut hilbert_values: [f64; 7] = [0.0; 7];

    while smooth_price_idx < 50 {
        smooth_price[smooth_price_idx] = 0.0;
        smooth_price_idx += 1;
    }

    while today < data.len() {
        let price = data[today];
        if today > 0 {
            smooth = (4.0 * price + 3.0 * data[today.saturating_sub(1)] + 2.0 * data[today.saturating_sub(2)] + data[today.saturating_sub(3)]) / 10.0;
        } else {
            smooth = price;
        }

        if today < 4 {
            mama_values.push(0.0);
            fama_values.push(0.0);
            today += 1;
            continue;
        }

        hilbert_idx = (hilbert_idx + 1) % 3;
        odd = (today % 2) == 1;

        if odd {
            detrender_odd[hilbert_idx] = (0.0962 * smooth) + (0.5769 * hilbert_values[0]) - (0.5769 * hilbert_values[2]) - (0.0962 * hilbert_values[4]);
            i1_for_odd = detrender_odd[hilbert_idx];
            q1_odd[hilbert_idx] = (0.0962 * i1_for_even) + (0.5769 * hilbert_values[1]) - (0.5769 * hilbert_values[3]) - (0.0962 * hilbert_values[5]);
            jI_odd[hilbert_idx] = (0.0962 * q1) + (0.5769 * jI) - (0.5769 * q2) - (0.0962 * i2);
            jQ_odd[hilbert_idx] = (0.0962 * i1) + (0.5769 * jQ) - (0.5769 * im) - (0.0962 * re);
        } else {
            detrender_even[hilbert_idx] = (0.0962 * smooth) + (0.5769 * hilbert_values[0]) - (0.5769 * hilbert_values[2]) - (0.0962 * hilbert_values[4]);
            i1_for_even = detrender_even[hilbert_idx];
            q1_even[hilbert_idx] = (0.0962 * i1_for_odd) + (0.5769 * hilbert_values[1]) - (0.5769 * hilbert_values[3]) - (0.0962 * hilbert_values[5]);
            jI_even[hilbert_idx] = (0.0962 * q1) + (0.5769 * jI) - (0.5769 * q2) - (0.0962 * i2);
            jQ_even[hilbert_idx] = (0.0962 * i1) + (0.5769 * jQ) - (0.5769 * im) - (0.0962 * re);
        }

        if odd {
            i1 = i1_for_odd;
            q1 = q1_odd[hilbert_idx];
            jI = jI_odd[hilbert_idx];
            jQ = jQ_odd[hilbert_idx];
        } else {
            i1 = i1_for_even;
            q1 = q1_even[hilbert_idx];
            jI = jI_even[hilbert_idx];
            jQ = jQ_even[hilbert_idx];
        }

        i2 = i1 - jQ;
        q2 = q1 + jI;
        im = (0.2 * i2) + (0.8 * im);
        re = (0.2 * q2) + (0.8 * re);

        if im != 0.0 && re != 0.0 {
            period = 360.0 / (std::f64::consts::PI * (re.atan2(im)));
        }

        if period > 1.5 * adjusted_prev_period {
            period = 1.5 * adjusted_prev_period;
        }
        if period < 0.67 * adjusted_prev_period {
            period = 0.67 * adjusted_prev_period;
        }
        if period < 6.0 {
            period = 6.0;
        } else if period > 50.0 {
            period = 50.0;
        }

        adjusted_prev_period = period;
        let smooth_period = 0.075 * period + 0.54;
        let delta_phase = (q1.atan2(i1) - prev_phase).abs();
        prev_phase = q1.atan2(i1);
        let mut alpha = fast_limit / (delta_phase + 1e-10);
        if alpha < slow_limit {
            alpha = slow_limit;
        } else if alpha > fast_limit {
            alpha = fast_limit;
        }

        mama = alpha * price + (1.0 - alpha) * mama;
        fama = 0.5 * alpha * mama + (1.0 - 0.5 * alpha) * fama;
        hilbert_values.copy_within(0..6, 1);
        hilbert_values[0] = smooth;
        if today >= 32 {
            result_mama.push(mama);
            result_fama.push(fama);
        } else {
            result_mama.push(0.0);
            result_fama.push(0.0);
        }

        today += 1;
    }

    Ok(MamaOutput {
        mama_values: result_mama,
        fama_values: result_fama,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_mama_accuracy() {
        let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
        let candles = read_candles_from_csv(file_path).expect("Failed to load test candles");
        let close_prices = candles
            .select_candle_field("close")
            .expect("Failed to extract close prices");

        let params = MamaParams { fast_limit: 0.5, slow_limit: 0.05 };
        let input = MamaInput::new(&close_prices, params);
        let result = calculate_mama(&input).expect("Failed to calculate MAMA");

        let mama_vals = &result.mama_values;
        let fama_vals = &result.fama_values;
        assert!(mama_vals.len() > 5 && fama_vals.len() > 5);

        let last_idx = mama_vals.len() - 5;
        let expected = vec![
            (59272.6126101837, 59904.82955384927),
            (59268.03197967452, 59888.90961449489),
            (59153.51598983726, 59705.06120833049),
            (59153.59019034539, 59691.27443288086),
            (59128.66068082812, 59677.20908907954),
        ];

        for (i, &(exp_mama, exp_fama)) in expected.iter().enumerate() {
            let got_mama = mama_vals[last_idx + i];
            let got_fama = fama_vals[last_idx + i];
            assert!((got_mama - exp_mama).abs() < 1e-8, "MAMA mismatch at {}: expected {}, got {}", i, exp_mama, got_mama);
            assert!((got_fama - exp_fama).abs() < 1e-8, "FAMA mismatch at {}: expected {}, got {}", i, exp_fama, got_fama);
        }
    }
}
