use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

#[inline(always)]
fn flip_sign_nonnan(x: f64, val: f64) -> f64 {
    if x.is_sign_negative() {
        -val
    } else {
        val
    }
}

#[inline(always)]
pub fn atan_raw64(x: f64) -> f64 {
    const N2: f64 = 0.273;
    (FRAC_PI_4 + N2 - N2 * x.abs()) * x
}

#[inline(always)]
pub fn atan64(x: f64) -> f64 {
    if x.abs() > 1.0 {
        debug_assert!(!x.is_nan());
        flip_sign_nonnan(x, FRAC_PI_2) - atan_raw64(1.0 / x)
    } else {
        atan_raw64(x)
    }
}
