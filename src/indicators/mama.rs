use std::error::Error;
use std::f64::consts::PI;

/// Safely emulate `nz(x[shift], x)` from Pine Script:
/// If `x[i - shift]` is not available (i - shift < 0), return `current_val`.
fn nz(arr: &Vec<f64>, i: usize, shift: i32, current_val: f64) -> f64 {
    if shift < 0 {
        // If requested a future index, just return current.
        return current_val;
    }
    let idx = i as i32 - shift;
    if idx < 0 || idx as usize >= arr.len() {
        current_val
    } else {
        arr[idx as usize]
    }
}

/// Compute a Hilbert transform of a single value at index i for a given array `x`.
/// Pine code:
/// out = 0.0962 * x + 0.5769 * nz(x[2]) - 0.5769 * nz(x[4]) - 0.0962 * nz(x[6])
fn hilbert_transform(x: &Vec<f64>, i: usize) -> f64 {
    let current = x[i];
    let val_2 = nz(&x, i, 2, current);
    let val_4 = nz(&x, i, 4, current);
    let val_6 = nz(&x, i, 6, current);
    0.0962 * current + 0.5769 * val_2 - 0.5769 * val_4 - 0.0962 * val_6
}

/// Compute component: out = hilbertTransform(src)*mesaPeriodMult
fn compute_component(x: &Vec<f64>, i: usize, mesa_period_mult: f64) -> f64 {
    hilbert_transform(x, i) * mesa_period_mult
}

/// Smooth component: out = 0.2*src + 0.8*nz(src[1], src)
fn smooth_component(x: &Vec<f64>, i: usize) -> f64 {
    let current = x[i];
    let val_1 = nz(&x, i, 1, current);
    0.2 * current + 0.8 * val_1
}

/// The main function to compute MAMA & FAMA similarly to the provided Pine Script code.
/// `src` is your input price series (e.g. hl2).
/// Returns (mama_values, fama_values)
pub fn calculate_mama_fama(src: &[f64], fast_limit: f64, slow_limit: f64) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    if src.len() < 10 {
        return Err("Not enough data".into());
    }

    // Prepare arrays:
    let len = src.len();

    // We'll store all intermediate series:
    let mut smooth = vec![0.0; len];
    let mut mesa_period = vec![0.0; len];
    let mut detrender = vec![0.0; len];
    let mut I1 = vec![0.0; len];
    let mut Q1 = vec![0.0; len];
    let mut jI = vec![0.0; len];
    let mut jQ = vec![0.0; len];
    let mut I2 = vec![0.0; len];
    let mut Q2 = vec![0.0; len];
    let mut Re = vec![0.0; len];
    let mut Im = vec![0.0; len];
    let mut phase = vec![0.0; len];
    let mut delta_phase = vec![0.0; len];
    let mut alpha = vec![0.0; len];
    let mut mama = vec![0.0; len];
    let mut fama = vec![0.0; len];

    // Initialization:
    // Pine sets mama and fama starting from zero and uses nz() logic.
    // We'll start from the first bar.
    // On the first bar, we don't have previous data, so results might need a warm-up.
    // After a few bars, it should stabilize and match the Pine calculations.
    mama[0] = src[0];
    fama[0] = src[0];
    // mesa_period is set to 0.0 initially as in the code.

    for i in 0..len {
        // Compute mesaPeriodMult:
        let prev_mesa = if i > 0 { mesa_period[i-1] } else { 0.0 };
        let mesa_period_mult = 0.075 * prev_mesa + 0.54;

        // smooth = (4*src + 3*nz(src[1]) + 2*nz(src[2]) + nz(src[3])) / 10
        let src_i_1 = if i > 0 { src[i-1] } else { src[i] };
        let src_i_2 = if i > 1 { src[i-2] } else { src[i] };
        let src_i_3 = if i > 2 { src[i-3] } else { src[i] };
        smooth[i] = (4.0*src[i] + 3.0*src_i_1 + 2.0*src_i_2 + src_i_3)/10.0;

        // detrender = compute_component(smooth, mesaPeriodMult)
        detrender[i] = compute_component(&smooth, i, mesa_period_mult);

        // I1 = nz(detrender[3], detrender)
        let I1_val = if i >= 3 {
            detrender[i-3]
        } else {
            // nz fallback: use current detrender[i]
            detrender[i]
        };
        I1[i] = I1_val;

        // Q1 = compute_component(detrender, mesaPeriodMult) at current bar
        Q1[i] = compute_component(&detrender, i, mesa_period_mult);

        // jI = compute_component(I1, mesaPeriodMult)
        jI[i] = compute_component(&I1, i, mesa_period_mult);

        // jQ = compute_component(Q1, mesaPeriodMult)
        jQ[i] = compute_component(&Q1, i, mesa_period_mult);

        // I2 = I1 - jQ
        I2[i] = I1[i] - jQ[i];
        // Q2 = Q1 + jI
        Q2[i] = Q1[i] + jI[i];

        // Smooth I2 and Q2:
        // I2 = smooth_component(I2)
        // Q2 = smooth_component(Q2)
        // But smooth_component needs arrays updated:
        // We'll do a trick: compute temp first, then assign.
        let I2_sm = {
            let cur = I2[i];
            let prev = nz(&I2, i, 1, cur);
            0.2 * cur + 0.8 * prev
        };
        I2[i] = I2_sm;

        let Q2_sm = {
            let cur = Q2[i];
            let prev = nz(&Q2, i, 1, cur);
            0.2 * cur + 0.8 * prev
        };
        Q2[i] = Q2_sm;

        // Re = I2 * nz(I2[1]) + Q2 * nz(Q2[1])
        let I2_1 = nz(&I2, i, 1, I2[i]);
        let Q2_1 = nz(&Q2, i, 1, Q2[i]);
        Re[i] = I2[i]*I2_1 + Q2[i]*Q2_1;
        Im[i] = I2[i]*Q2_1 - Q2[i]*I2_1;

        // Smooth Re and Im:
        let Re_sm = {
            let cur = Re[i];
            let prev = nz(&Re, i, 1, cur);
            0.2 * cur + 0.8 * prev
        };
        Re[i] = Re_sm;

        let Im_sm = {
            let cur = Im[i];
            let prev = nz(&Im, i, 1, cur);
            0.2 * cur + 0.8 * prev
        };
        Im[i] = Im_sm;

        // if Re !=0 and Im !=0 mesaPeriod = 2*PI/atan(Im/Re)
        let mut cur_mesa = mesa_period[i]; // start from 0.0 as assigned
        if Re[i] != 0.0 && Im[i] != 0.0 {
            cur_mesa = 2.0 * PI / (Im[i]/Re[i]).atan();
        }

        // mesaPeriod = min(mesaPeriod, 1.5 * nz(mesaPeriod[1], mesaPeriod))
        let mp_1 = if i>0 { mesa_period[i-1] } else { cur_mesa };
        cur_mesa = cur_mesa.min(1.5 * mp_1);

        // mesaPeriod = max(mesaPeriod, 0.67 * nz(mesaPeriod[1], mesaPeriod))
        cur_mesa = cur_mesa.max(0.67 * mp_1);

        // mesaPeriod = min(max(mesaPeriod,6),50)
        cur_mesa = cur_mesa.max(6.0).min(50.0);

        // mesaPeriod = smooth_component(mesaPeriod)
        let mp_sm = {
            let prev = if i>0 { mesa_period[i-1] } else { cur_mesa };
            0.2 * cur_mesa + 0.8 * prev
        };
        mesa_period[i] = mp_sm;

        // phase = 0
        // if I1 !=0 phase=(180/PI)*atan(Q1/I1)
        let mut cur_phase = 0.0;
        if I1[i] != 0.0 {
            cur_phase = (180.0/PI)*(Q1[i]/I1[i]).atan();
        }
        phase[i] = cur_phase;

        // deltaPhase = nz(phase[1], phase)-phase
        let ph_1 = nz(&phase, i, 1, phase[i]);
        let mut dp = ph_1 - phase[i];
        dp = dp.max(1.0); // deltaPhase = max(deltaPhase,1)
        delta_phase[i] = dp;

        let mut cur_alpha = (fast_limit/dp).max(slow_limit);
        alpha[i] = cur_alpha;

        // Now compute MAMA & FAMA:
        // mama = alpha*src+(1-alpha)*nz(mama[1],src)
        let mama_1 = if i>0 { mama[i-1] } else { src[i] };
        mama[i] = alpha[i]*src[i] + (1.0 - alpha[i])*mama_1;

        // fama = (alpha/2)*mama+(1-(alpha/2))*nz(fama[1],mama)
        let a2 = alpha[i]/2.0;
        let fama_1 = if i>0 { fama[i-1] } else { mama[i] };
        fama[i] = a2 * mama[i] + (1.0 - a2)*fama_1;
    }

    Ok((mama, fama))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::data_loader::read_candles_from_csv;

    #[test]
    fn test_mama_accuracy_with_tolerance() {
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

        // Test expected values with a 0.1% tolerance
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

            let mama_diff = (got_mama - exp_mama).abs() / exp_mama * 100.0;
            let fama_diff = (got_fama - exp_fama).abs() / exp_fama * 100.0;
            print!("{}: {} {}\n", i, got_mama, got_fama);
            assert!(
                mama_diff < 0.1,
                "MAMA mismatch at {}: expected {}, got {}, diff {}%",
                i,
                exp_mama,
                got_mama,
                mama_diff
            );
            assert!(
                fama_diff < 0.1,
                "FAMA mismatch at {}: expected {}, got {}, diff {}%",
                i,
                exp_fama,
                got_fama,
                fama_diff
            );
        }
    }
}
