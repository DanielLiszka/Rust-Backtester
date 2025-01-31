use crate::utilities::data_loader::Candles;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum PatternData<'a> {
    Candles { candles: &'a Candles },
}

impl Default for PatternType {
    fn default() -> Self {
        PatternType::Cdl2Crows
    }
}

#[derive(Debug, Clone, Default)]
pub struct PatternParams {
    pub pattern_type: PatternType,
    pub penetration: f64,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    Cdl2Crows,
    Cdl3BlackCrows,
    Cdl3Inside,
    Cdl3LineStrike,
    Cdl3Outside,
    Cdl3StarsInSouth,
    Cdl3WhiteSoldiers,
    CdlAbandonedBaby,
    CdlAdvanceBlock,
    CdlBeltHold,
    CdlBreakaway,
    CdlClosingMarubozu,
    CdlConcealBabySwall,
    CdlCounterAttack,
    CdlDarkCloudCover,
    CdlDoji,
    CdlDojiStar,
    CdlDragonflyDoji,
    CdlEngulfing,
    CdlEveningDojiStar,
    CdlEveningStar,
    CdlGapSideSideWhite,
    CdlGravestoneDoji,
    CdlHammer,
    CdlHangingMan,
    CdlHarami,
    CdlHaramiCross,
    CdlHighWave,
    CdlHikkake,
    CdlHikkakeMod,
    CdlHomingPigeon,
    CdlIdentical3Crows,
    CdlInNeck,
    CdlInvertedHammer,
    CdlKicking,
    CdlKickingByLength,
    CdlLadderBottom,
    CdlLongLeggedDoji,
    CdlLongLine,
    CdlMarubozu,
    CdlMatchingLow,
    CdlMatHold,
    CdlMorningDojiStar,
    CdlMorningStar,
    CdlOnNeck,
    CdlPiercing,
    CdlRickshawMan,
    CdlRiseFall3Methods,
    CdlSeparatingLines,
    CdlShootingStar,
    CdlShortLine,
    CdlSpinningTop,
    CdlStalledPattern,
    CdlStickSandwich,
    CdlTakuri,
    CdlTasukiGap,
    CdlThrusting,
    CdlTristar,
    CdlUnique3River,
    CdlUpsideGap2Crows,
    CdlXSideGap3Methods,
}

#[derive(Debug, Clone)]
pub struct PatternInput<'a> {
    pub data: PatternData<'a>,
    pub params: PatternParams,
}

impl<'a> PatternInput<'a> {
    pub fn from_candles(candles: &'a Candles, params: PatternParams) -> Self {
        Self {
            data: PatternData::Candles { candles },
            params,
        }
    }

    pub fn with_default_candles(candles: &'a Candles, pattern_type: PatternType) -> Self {
        Self {
            data: PatternData::Candles { candles },
            params: PatternParams {
                pattern_type,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatternOutput {
    pub values: Vec<i8>,
}

#[derive(Debug, Error)]
pub enum PatternError {
    #[error("pattern_recognition: Not enough data points. Length={len}, pattern={pattern:?}")]
    NotEnoughData { len: usize, pattern: PatternType },

    #[error("pattern_recognition: Candle field error: {0}")]
    CandleFieldError(String),

    #[error("pattern_recognition: Unknown error occurred.")]
    Unknown,
}

#[inline(always)]
fn candle_color(open: f64, close: f64) -> i32 {
    if close >= open {
        1
    } else {
        -1
    }
}

#[inline(always)]
fn real_body(open: f64, close: f64) -> f64 {
    (close - open).abs()
}

#[inline(always)]
fn candle_range(open: f64, close: f64) -> f64 {
    real_body(open, close)
}

#[inline]
pub fn cdl2crows(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    const BODY_LONG_PERIOD: usize = 10;

    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let lookback_total = 2 + BODY_LONG_PERIOD;

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];

    let mut body_long_period_total = 0.0;
    let body_long_trailing_start = 0;
    let body_long_trailing_end = BODY_LONG_PERIOD;
    for i in body_long_trailing_start..body_long_trailing_end {
        body_long_period_total += candle_range(open[i], close[i]);
    }

    for i in lookback_total..size {
        let first_color = candle_color(open[i - 2], close[i - 2]);
        let first_body = real_body(open[i - 2], close[i - 2]);
        let body_long_avg = body_long_period_total / (BODY_LONG_PERIOD as f64);

        let second_color = candle_color(open[i - 1], close[i - 1]);
        let third_color = candle_color(open[i], close[i]);

        let second_rb_min = open[i - 1].min(close[i - 1]);
        let first_rb_max = open[i - 2].max(close[i - 2]);
        let real_body_gap_up = second_rb_min > first_rb_max;

        let third_opens_in_2nd_body = open[i] < open[i - 1] && open[i] > close[i - 1];

        let third_closes_in_1st_body = close[i] > open[i - 2] && close[i] < close[i - 2];

        if (first_color == 1)
            && (first_body > body_long_avg)
            && (second_color == -1)
            && real_body_gap_up
            && (third_color == -1)
            && third_opens_in_2nd_body
            && third_closes_in_1st_body
        {
            out[i] = -100;
        } else {
            out[i] = 0;
        }

        let old_idx = i - lookback_total;
        let new_idx = i - 2;
        body_long_period_total += candle_range(open[new_idx], close[new_idx])
            - candle_range(open[old_idx], close[old_idx]);
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdl3blackcrows(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    const SHADOW_VERY_SHORT_PERIOD: usize = 10;

    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let lookback_total = 3 + SHADOW_VERY_SHORT_PERIOD;
    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];

    fn candle_color(o: f64, c: f64) -> i8 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    fn lower_shadow(o: f64, c: f64, l: f64) -> f64 {
        if c < o {
            c - l
        } else {
            o - l
        }
    }

    let mut sum2 = 0.0;
    let mut sum1 = 0.0;
    let mut sum0 = 0.0;
    for i in 0..SHADOW_VERY_SHORT_PERIOD {
        sum2 += lower_shadow(open[i], close[i], low[i]);
        sum1 += lower_shadow(open[i + 1], close[i + 1], low[i + 1]);
        sum0 += lower_shadow(open[i + 2], close[i + 2], low[i + 2]);
    }

    for i in lookback_total..size {
        let avg2 = sum2 / (SHADOW_VERY_SHORT_PERIOD as f64);
        let avg1 = sum1 / (SHADOW_VERY_SHORT_PERIOD as f64);
        let avg0 = sum0 / (SHADOW_VERY_SHORT_PERIOD as f64);

        if candle_color(open[i - 3], close[i - 3]) == 1
            && candle_color(open[i - 2], close[i - 2]) == -1
            && lower_shadow(open[i - 2], close[i - 2], low[i - 2]) < avg2
            && candle_color(open[i - 1], close[i - 1]) == -1
            && lower_shadow(open[i - 1], close[i - 1], low[i - 1]) < avg1
            && candle_color(open[i], close[i]) == -1
            && lower_shadow(open[i], close[i], low[i]) < avg0
            && open[i - 1] < open[i - 2]
            && open[i - 1] > close[i - 2]
            && open[i] < open[i - 1]
            && open[i] > close[i - 1]
            && high[i - 3] > close[i - 2]
            && close[i - 2] > close[i - 1]
            && close[i - 1] > close[i]
        {
            out[i] = -100;
        } else {
            out[i] = 0;
        }

        let old_idx2 = i - lookback_total;
        let new_idx2 = i - 2;
        sum2 += lower_shadow(open[new_idx2], close[new_idx2], low[new_idx2])
            - lower_shadow(open[old_idx2], close[old_idx2], low[old_idx2]);

        let old_idx1 = i - lookback_total + 1;
        let new_idx1 = i - 1;
        sum1 += lower_shadow(open[new_idx1], close[new_idx1], low[new_idx1])
            - lower_shadow(open[old_idx1], close[old_idx1], low[old_idx1]);

        let old_idx0 = i - lookback_total + 2;
        let new_idx0 = i;
        sum0 += lower_shadow(open[new_idx0], close[new_idx0], low[new_idx0])
            - lower_shadow(open[old_idx0], close[old_idx0], low[old_idx0]);
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdl3inside(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    const BODY_LONG_PERIOD: usize = 10;
    const BODY_SHORT_PERIOD: usize = 10;

    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let lookback_total = 2 + BODY_LONG_PERIOD.max(BODY_SHORT_PERIOD);
    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    fn candle_color(o: f64, c: f64) -> i8 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    fn candle_range(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    fn real_body(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    fn max2(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }

    fn min2(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }

    let mut out = vec![0i8; size];

    let mut body_long_period_total = 0.0;
    let mut body_short_period_total = 0.0;

    for i in 0..BODY_LONG_PERIOD {
        body_long_period_total += candle_range(open[i], close[i]);
    }
    for i in 0..BODY_SHORT_PERIOD {
        body_short_period_total += candle_range(open[i], close[i]);
    }

    for i in lookback_total..size {
        let avg_body_long = body_long_period_total / BODY_LONG_PERIOD as f64;
        let avg_body_short = body_short_period_total / BODY_SHORT_PERIOD as f64;

        if real_body(open[i - 2], close[i - 2]) > avg_body_long
            && real_body(open[i - 1], close[i - 1]) <= avg_body_short
            && max2(close[i - 1], open[i - 1]) < max2(close[i - 2], open[i - 2])
            && min2(close[i - 1], open[i - 1]) > min2(close[i - 2], open[i - 2])
            && ((candle_color(open[i - 2], close[i - 2]) == 1
                && candle_color(open[i], close[i]) == -1
                && close[i] < open[i - 2])
                || (candle_color(open[i - 2], close[i - 2]) == -1
                    && candle_color(open[i], close[i]) == 1
                    && close[i] > open[i - 2]))
        {
            out[i] = -candle_color(open[i - 2], close[i - 2]) * 100;
        } else {
            out[i] = 0;
        }

        let old_idx_long = i - lookback_total;
        body_long_period_total += candle_range(open[i - 2], close[i - 2])
            - candle_range(open[old_idx_long], close[old_idx_long]);

        let old_idx_short = i - lookback_total + 1;
        body_short_period_total += candle_range(open[i - 1], close[i - 1])
            - candle_range(open[old_idx_short], close[old_idx_short]);
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdl3linestrike(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    const NEAR_PERIOD: usize = 10;

    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let lookback_total = 3 + NEAR_PERIOD;
    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    fn candle_color(o: f64, c: f64) -> i8 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    fn candle_range(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    fn max2(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }

    fn min2(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }

    let mut out = vec![0i8; size];
    let mut sum3 = 0.0;
    let mut sum2 = 0.0;

    for i in 0..NEAR_PERIOD {
        sum3 += candle_range(open[i], close[i]);
        sum2 += candle_range(open[i + 1], close[i + 1]);
    }

    for i in lookback_total..size {
        let avg3 = sum3 / (NEAR_PERIOD as f64);
        let avg2 = sum2 / (NEAR_PERIOD as f64);

        if candle_color(open[i - 3], close[i - 3]) == candle_color(open[i - 2], close[i - 2])
            && candle_color(open[i - 2], close[i - 2]) == candle_color(open[i - 1], close[i - 1])
            && candle_color(open[i], close[i]) == -candle_color(open[i - 1], close[i - 1])
            && open[i - 2] >= min2(open[i - 3], close[i - 3]) - avg3
            && open[i - 2] <= max2(open[i - 3], close[i - 3]) + avg3
            && open[i - 1] >= min2(open[i - 2], close[i - 2]) - avg2
            && open[i - 1] <= max2(open[i - 2], close[i - 2]) + avg2
            && ((candle_color(open[i - 1], close[i - 1]) == 1
                && close[i - 1] > close[i - 2]
                && close[i - 2] > close[i - 3]
                && open[i] > close[i - 1]
                && close[i] < open[i - 3])
                || (candle_color(open[i - 1], close[i - 1]) == -1
                    && close[i - 1] < close[i - 2]
                    && close[i - 2] < close[i - 3]
                    && open[i] < close[i - 1]
                    && close[i] > open[i - 3]))
        {
            out[i] = candle_color(open[i - 1], close[i - 1]) * 100;
        } else {
            out[i] = 0;
        }

        let old_idx3 = i - lookback_total;
        let new_idx3 = i - 3;
        sum3 += candle_range(open[new_idx3], close[new_idx3])
            - candle_range(open[old_idx3], close[old_idx3]);

        let old_idx2 = i - lookback_total + 1;
        let new_idx2 = i - 2;
        sum2 += candle_range(open[new_idx2], close[new_idx2])
            - candle_range(open[old_idx2], close[old_idx2]);
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdl3outside(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    fn candle_color(o: f64, c: f64) -> i8 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    let size = open.len();
    let lookback_total = 2;

    if size < lookback_total + 1 {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];

    for i in lookback_total..size {
        let second_candle_color = candle_color(open[i - 1], close[i - 1]);
        let first_candle_color = candle_color(open[i - 2], close[i - 2]);

        let white_engulfs_black = second_candle_color == 1
            && first_candle_color == -1
            && close[i - 1] > open[i - 2]
            && open[i - 1] < close[i - 2]
            && close[i] > close[i - 1];

        let black_engulfs_white = second_candle_color == -1
            && first_candle_color == 1
            && open[i - 1] > close[i - 2]
            && close[i - 1] < open[i - 2]
            && close[i] < close[i - 1];

        if white_engulfs_black || black_engulfs_white {
            out[i] = second_candle_color * 100;
        } else {
            out[i] = 0;
        }
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdl3starsinsouth(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    const BODY_LONG_PERIOD: usize = 10;
    const SHADOW_LONG_PERIOD: usize = 10;
    const SHADOW_VERY_SHORT_PERIOD: usize = 10;
    const BODY_SHORT_PERIOD: usize = 10;

    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            (open, high, low, close)
        }
    };

    fn candle_color(o: f64, c: f64) -> i8 {
        if c >= o {
            1
        } else {
            -1
        }
    }
    fn real_body(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }
    fn lower_shadow(o: f64, c: f64, l: f64) -> f64 {
        if c < o {
            c - l
        } else {
            o - l
        }
    }
    fn upper_shadow(o: f64, c: f64, h: f64) -> f64 {
        if c < o {
            h - o
        } else {
            h - c
        }
    }
    fn candle_range(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    let size = open.len();
    let lookback_total = 2 + BODY_LONG_PERIOD
        .max(SHADOW_LONG_PERIOD)
        .max(SHADOW_VERY_SHORT_PERIOD)
        .max(BODY_SHORT_PERIOD);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];

    let mut body_long_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_very_short_sum_1 = 0.0;
    let mut shadow_very_short_sum_0 = 0.0;
    let mut body_short_sum = 0.0;

    let body_long_trail_start = lookback_total - BODY_LONG_PERIOD;
    for idx in body_long_trail_start..lookback_total {
        let ref_index = if idx >= 2 { idx - 2 } else { 0 };
        body_long_sum += candle_range(open[ref_index], close[ref_index]);
    }

    let shadow_long_trail_start = lookback_total - SHADOW_LONG_PERIOD;
    for idx in shadow_long_trail_start..lookback_total {
        let ref_index = if idx >= 2 { idx - 2 } else { 0 };
        shadow_long_sum += candle_range(open[ref_index], close[ref_index]);
    }

    let shadow_very_short_trail_start = lookback_total - SHADOW_VERY_SHORT_PERIOD;
    for idx in shadow_very_short_trail_start..lookback_total {
        let ref_index_1 = if idx >= 1 { idx - 1 } else { 0 };
        shadow_very_short_sum_1 +=
            lower_shadow(open[ref_index_1], close[ref_index_1], low[ref_index_1]);

        shadow_very_short_sum_0 += lower_shadow(open[idx], close[idx], low[idx]);
    }

    let body_short_trail_start = lookback_total - BODY_SHORT_PERIOD;
    for idx in body_short_trail_start..lookback_total {
        body_short_sum += candle_range(open[idx], close[idx]);
    }
    for i in lookback_total..size {
        let avg_body_long = body_long_sum / (BODY_LONG_PERIOD as f64);
        let avg_shadow_long = shadow_long_sum / (SHADOW_LONG_PERIOD as f64);
        let avg_shadow_very_short_1 = shadow_very_short_sum_1 / (SHADOW_VERY_SHORT_PERIOD as f64);
        let avg_shadow_very_short_0 = shadow_very_short_sum_0 / (SHADOW_VERY_SHORT_PERIOD as f64);
        let avg_body_short = body_short_sum / (BODY_SHORT_PERIOD as f64);

        if candle_color(open[i - 2], close[i - 2]) == -1
            && candle_color(open[i - 1], close[i - 1]) == -1
            && candle_color(open[i], close[i]) == -1
            && real_body(open[i - 2], close[i - 2]) > avg_body_long
            && lower_shadow(open[i - 2], close[i - 2], low[i - 2]) > avg_shadow_long
            && real_body(open[i - 1], close[i - 1]) < real_body(open[i - 2], close[i - 2])
            && open[i - 1] > close[i - 2]
            && open[i - 1] <= high[i - 2]
            && low[i - 1] < close[i - 2]
            && low[i - 1] >= low[i - 2]
            && lower_shadow(open[i - 1], close[i - 1], low[i - 1]) > avg_shadow_very_short_1
            && real_body(open[i], close[i]) < avg_body_short
            && lower_shadow(open[i], close[i], low[i]) < avg_shadow_very_short_0
            && upper_shadow(open[i], close[i], high[i]) < avg_shadow_very_short_0
            && low[i] > low[i - 1]
            && high[i] < high[i - 1]
        {
            out[i] = 100;
        } else {
            out[i] = 0;
        }

        let old_idx = i - lookback_total;

        {
            let old_ref = if old_idx >= 2 { old_idx - 2 } else { 0 };
            let new_ref = i - 2;
            body_long_sum += candle_range(open[new_ref], close[new_ref])
                - candle_range(open[old_ref], close[old_ref]);
        }

        {
            let old_ref = if old_idx >= 2 { old_idx - 2 } else { 0 };
            let new_ref = i - 2;
            shadow_long_sum += candle_range(open[new_ref], close[new_ref])
                - candle_range(open[old_ref], close[old_ref]);
        }

        {
            let old_ref_1 = if old_idx >= 1 { old_idx - 1 } else { 0 };
            let new_ref_1 = i - 1;
            shadow_very_short_sum_1 +=
                lower_shadow(open[new_ref_1], close[new_ref_1], low[new_ref_1])
                    - lower_shadow(open[old_ref_1], close[old_ref_1], low[old_ref_1]);
        }
        {
            shadow_very_short_sum_0 += lower_shadow(open[i], close[i], low[i])
                - lower_shadow(open[old_idx], close[old_idx], low[old_idx]);
        }

        {
            body_short_sum +=
                candle_range(open[i], close[i]) - candle_range(open[old_idx], close[old_idx]);
        }
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdl3whitesoldiers(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    const SHADOW_VERY_SHORT_PERIOD: usize = 10;
    const NEAR_PERIOD: usize = 10;
    const FAR_PERIOD: usize = 10;
    const BODY_SHORT_PERIOD: usize = 10;

    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    fn candle_color(o: f64, c: f64) -> i8 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    fn candle_range(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    fn real_body(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    fn upper_shadow(o: f64, c: f64, h: f64) -> f64 {
        if c < o {
            h - o
        } else {
            h - c
        }
    }

    fn max2(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }

    let size = open.len();
    let lookback_total = 2 + SHADOW_VERY_SHORT_PERIOD
        .max(NEAR_PERIOD)
        .max(FAR_PERIOD)
        .max(BODY_SHORT_PERIOD);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut shadow_very_short_sum = [0.0; 3];
    let mut near_sum = [0.0; 3];
    let mut far_sum = [0.0; 3];
    let mut body_short_sum = 0.0;

    for i in 0..SHADOW_VERY_SHORT_PERIOD {
        shadow_very_short_sum[2] += upper_shadow(open[i], close[i], high[i]);
        if i + 1 < size {
            shadow_very_short_sum[1] += upper_shadow(open[i + 1], close[i + 1], high[i + 1]);
        }
        if i + 2 < size {
            shadow_very_short_sum[0] += upper_shadow(open[i + 2], close[i + 2], high[i + 2]);
        }
    }
    for i in 0..NEAR_PERIOD {
        if i + 2 < size {
            near_sum[2] += candle_range(open[i + 2], close[i + 2]);
        }
        if i + 1 < size {
            near_sum[1] += candle_range(open[i + 1], close[i + 1]);
        }
    }
    for i in 0..FAR_PERIOD {
        if i + 2 < size {
            far_sum[2] += candle_range(open[i + 2], close[i + 2]);
        }
        if i + 1 < size {
            far_sum[1] += candle_range(open[i + 1], close[i + 1]);
        }
    }
    for i in 0..BODY_SHORT_PERIOD {
        body_short_sum += candle_range(open[i], close[i]);
    }

    for i in lookback_total..size {
        let avg_sv_2 = shadow_very_short_sum[2] / (SHADOW_VERY_SHORT_PERIOD as f64);
        let avg_sv_1 = shadow_very_short_sum[1] / (SHADOW_VERY_SHORT_PERIOD as f64);
        let avg_sv_0 = shadow_very_short_sum[0] / (SHADOW_VERY_SHORT_PERIOD as f64);
        let avg_near_2 = near_sum[2] / (NEAR_PERIOD as f64);
        let avg_near_1 = near_sum[1] / (NEAR_PERIOD as f64);
        let avg_far_2 = far_sum[2] / (FAR_PERIOD as f64);
        let avg_far_1 = far_sum[1] / (FAR_PERIOD as f64);
        let avg_body_short = body_short_sum / (BODY_SHORT_PERIOD as f64);

        if candle_color(open[i - 2], close[i - 2]) == 1
            && upper_shadow(open[i - 2], close[i - 2], high[i - 2]) < avg_sv_2
            && candle_color(open[i - 1], close[i - 1]) == 1
            && upper_shadow(open[i - 1], close[i - 1], high[i - 1]) < avg_sv_1
            && candle_color(open[i], close[i]) == 1
            && upper_shadow(open[i], close[i], high[i]) < avg_sv_0
            && close[i] > close[i - 1]
            && close[i - 1] > close[i - 2]
            && open[i - 1] > open[i - 2]
            && open[i - 1] <= close[i - 2] + avg_near_2
            && open[i] > open[i - 1]
            && open[i] <= close[i - 1] + avg_near_1
            && real_body(open[i - 1], close[i - 1])
                > real_body(open[i - 2], close[i - 2]) - avg_far_2
            && real_body(open[i], close[i]) > real_body(open[i - 1], close[i - 1]) - avg_far_1
            && real_body(open[i], close[i]) > avg_body_short
        {
            out[i] = 100;
        } else {
            out[i] = 0;
        }

        let old_idx = i - lookback_total;
        shadow_very_short_sum[2] += upper_shadow(open[i - 2], close[i - 2], high[i - 2])
            - upper_shadow(
                open[old_idx.saturating_sub(2)],
                close[old_idx.saturating_sub(2)],
                high[old_idx.saturating_sub(2)],
            );
        shadow_very_short_sum[1] += upper_shadow(open[i - 1], close[i - 1], high[i - 1])
            - upper_shadow(
                open[old_idx.saturating_sub(1)],
                close[old_idx.saturating_sub(1)],
                high[old_idx.saturating_sub(1)],
            );
        shadow_very_short_sum[0] += upper_shadow(open[i], close[i], high[i])
            - upper_shadow(open[old_idx], close[old_idx], high[old_idx]);

        far_sum[2] += candle_range(open[i - 2], close[i - 2])
            - candle_range(
                open[old_idx.saturating_sub(2)],
                close[old_idx.saturating_sub(2)],
            );
        far_sum[1] += candle_range(open[i - 1], close[i - 1])
            - candle_range(
                open[old_idx.saturating_sub(1)],
                close[old_idx.saturating_sub(1)],
            );

        near_sum[2] += candle_range(open[i - 2], close[i - 2])
            - candle_range(
                open[old_idx.saturating_sub(2)],
                close[old_idx.saturating_sub(2)],
            );
        near_sum[1] += candle_range(open[i - 1], close[i - 1])
            - candle_range(
                open[old_idx.saturating_sub(1)],
                close[old_idx.saturating_sub(1)],
            );

        body_short_sum +=
            candle_range(open[i], close[i]) - candle_range(open[old_idx], close[old_idx]);
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdlabandonedbaby(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    const BODY_LONG_PERIOD: usize = 10;
    const BODY_DOJI_PERIOD: usize = 10;
    const BODY_SHORT_PERIOD: usize = 10;

    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let penetration = input.params.penetration;

    fn candle_color(o: f64, c: f64) -> i8 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    fn real_body(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    fn candle_range(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    fn candle_gap_up(idx1: usize, idx2: usize, low: &[f64], high: &[f64]) -> bool {
        low[idx1] > high[idx2]
    }

    fn candle_gap_down(idx1: usize, idx2: usize, low: &[f64], high: &[f64]) -> bool {
        high[idx1] < low[idx2]
    }

    let size = open.len();
    let lookback_total = 2 + BODY_LONG_PERIOD
        .max(BODY_DOJI_PERIOD)
        .max(BODY_SHORT_PERIOD);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let mut body_short_sum = 0.0;

    for i in 0..BODY_LONG_PERIOD {
        body_long_sum += candle_range(open[i], close[i]);
    }
    for i in 0..BODY_DOJI_PERIOD {
        body_doji_sum += candle_range(open[i], close[i]);
    }
    for i in 0..BODY_SHORT_PERIOD {
        body_short_sum += candle_range(open[i], close[i]);
    }

    for i in lookback_total..size {
        let avg_body_long = body_long_sum / BODY_LONG_PERIOD as f64;
        let avg_body_doji = body_doji_sum / BODY_DOJI_PERIOD as f64;
        let avg_body_short = body_short_sum / BODY_SHORT_PERIOD as f64;

        if real_body(open[i - 2], close[i - 2]) > avg_body_long
            && real_body(open[i - 1], close[i - 1]) <= avg_body_doji
            && real_body(open[i], close[i]) > avg_body_short
            && ((candle_color(open[i - 2], close[i - 2]) == 1
                && candle_color(open[i], close[i]) == -1
                && close[i] < close[i - 2] - real_body(open[i - 2], close[i - 2]) * penetration
                && candle_gap_up(i - 1, i - 2, &low, &high)
                && candle_gap_down(i, i - 1, &low, &high))
                || (candle_color(open[i - 2], close[i - 2]) == -1
                    && candle_color(open[i], close[i]) == 1
                    && close[i]
                        > close[i - 2] + real_body(open[i - 2], close[i - 2]) * penetration
                    && candle_gap_down(i - 1, i - 2, &low, &high)
                    && candle_gap_up(i, i - 1, &low, &high)))
        {
            out[i] = candle_color(open[i], close[i]) * 100;
        } else {
            out[i] = 0;
        }

        let old_idx = i - lookback_total;
        body_long_sum += candle_range(open[i - 2], close[i - 2])
            - candle_range(
                open[old_idx.saturating_sub(2)],
                close[old_idx.saturating_sub(2)],
            );
        body_doji_sum += candle_range(open[i - 1], close[i - 1])
            - candle_range(
                open[old_idx.saturating_sub(1)],
                close[old_idx.saturating_sub(1)],
            );
        body_short_sum +=
            candle_range(open[i], close[i]) - candle_range(open[old_idx], close[old_idx]);
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdladvanceblock(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let shadow_short_period = 10;
    let shadow_long_period = 10;
    let near_period = 5;
    let far_period = 5;
    let body_long_period = 10;
    let lookback_total = 2 + shadow_short_period
        .max(shadow_long_period)
        .max(near_period)
        .max(far_period)
        .max(body_long_period);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];

    let mut shadow_short_period_total = [0.0; 3];
    let mut shadow_long_period_total = [0.0; 2];
    let mut near_period_total = [0.0; 3];
    let mut far_period_total = [0.0; 3];
    let mut body_long_period_total = 0.0;

    #[inline(always)]
    fn upper_shadow(o: f64, h: f64, c: f64) -> f64 {
        if c >= o {
            h - c
        } else {
            h - o
        }
    }

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let start_idx = lookback_total;
    let mut shadow_short_trailing_idx = start_idx.saturating_sub(shadow_short_period);
    let mut shadow_long_trailing_idx = start_idx.saturating_sub(shadow_long_period);
    let mut near_trailing_idx = start_idx.saturating_sub(near_period);
    let mut far_trailing_idx = start_idx.saturating_sub(far_period);
    let mut body_long_trailing_idx = start_idx.saturating_sub(body_long_period);

    let mut i = shadow_short_trailing_idx;
    while i < start_idx {
        shadow_short_period_total[2] += upper_shadow(
            open[i.saturating_sub(2)],
            high[i.saturating_sub(2)],
            close[i.saturating_sub(2)],
        );
        shadow_short_period_total[1] += upper_shadow(
            open[i.saturating_sub(1)],
            high[i.saturating_sub(1)],
            close[i.saturating_sub(1)],
        );
        shadow_short_period_total[0] += upper_shadow(open[i], high[i], close[i]);
        i += 1;
    }
    i = shadow_long_trailing_idx;
    while i < start_idx {
        shadow_long_period_total[1] += upper_shadow(
            open[i.saturating_sub(1)],
            high[i.saturating_sub(1)],
            close[i.saturating_sub(1)],
        );
        shadow_long_period_total[0] += upper_shadow(open[i], high[i], close[i]);
        i += 1;
    }
    i = near_trailing_idx;
    while i < start_idx {
        near_period_total[2] += real_body(open[i.saturating_sub(2)], close[i.saturating_sub(2)]);
        near_period_total[1] += real_body(open[i.saturating_sub(1)], close[i.saturating_sub(1)]);
        i += 1;
    }
    i = far_trailing_idx;
    while i < start_idx {
        far_period_total[2] += real_body(open[i.saturating_sub(2)], close[i.saturating_sub(2)]);
        far_period_total[1] += real_body(open[i.saturating_sub(1)], close[i.saturating_sub(1)]);
        i += 1;
    }
    i = body_long_trailing_idx;
    while i < start_idx {
        body_long_period_total += real_body(open[i.saturating_sub(2)], close[i.saturating_sub(2)]);
        i += 1;
    }

    let mut idx = start_idx;
    while idx < size {
        let c1_color = candle_color(open[idx - 2], close[idx - 2]);
        let c2_color = candle_color(open[idx - 1], close[idx - 1]);
        let c3_color = candle_color(open[idx], close[idx]);

        if c1_color == 1
            && c2_color == 1
            && c3_color == 1
            && close[idx] > close[idx - 1]
            && close[idx - 1] > close[idx - 2]
            && open[idx - 1] > open[idx - 2]
            && open[idx - 1] <= close[idx - 2] + candle_average(near_period_total[2], near_period)
            && open[idx] > open[idx - 1]
            && open[idx] <= close[idx - 1] + candle_average(near_period_total[1], near_period)
            && real_body(open[idx - 2], close[idx - 2])
                > candle_average(body_long_period_total, body_long_period)
            && upper_shadow(open[idx - 2], high[idx - 2], close[idx - 2])
                < candle_average(shadow_short_period_total[2], shadow_short_period)
            && ((real_body(open[idx - 1], close[idx - 1])
                < real_body(open[idx - 2], close[idx - 2])
                    - candle_average(far_period_total[2], far_period)
                && real_body(open[idx], close[idx])
                    < real_body(open[idx - 1], close[idx - 1])
                        + candle_average(near_period_total[1], near_period))
                || (real_body(open[idx], close[idx])
                    < real_body(open[idx - 1], close[idx - 1])
                        - candle_average(far_period_total[1], far_period))
                || (real_body(open[idx], close[idx]) < real_body(open[idx - 1], close[idx - 1])
                    && real_body(open[idx - 1], close[idx - 1])
                        < real_body(open[idx - 2], close[idx - 2])
                    && (upper_shadow(open[idx], high[idx], close[idx])
                        > candle_average(shadow_short_period_total[0], shadow_short_period)
                        || upper_shadow(open[idx - 1], high[idx - 1], close[idx - 1])
                            > candle_average(shadow_short_period_total[1], shadow_short_period)))
                || (real_body(open[idx], close[idx]) < real_body(open[idx - 1], close[idx - 1])
                    && upper_shadow(open[idx], high[idx], close[idx])
                        > candle_average(shadow_long_period_total[0], shadow_long_period)))
        {
            out[idx] = -100;
        }

        for tot_idx in (0..=2).rev() {
            if tot_idx < 3 {
                shadow_short_period_total[tot_idx] += upper_shadow(
                    open[idx.saturating_sub(tot_idx)],
                    high[idx.saturating_sub(tot_idx)],
                    close[idx.saturating_sub(tot_idx)],
                ) - upper_shadow(
                    open[shadow_short_trailing_idx.saturating_sub(tot_idx)],
                    high[shadow_short_trailing_idx.saturating_sub(tot_idx)],
                    close[shadow_short_trailing_idx.saturating_sub(tot_idx)],
                );
            }
        }

        for tot_idx in (0..=1).rev() {
            shadow_long_period_total[tot_idx] += upper_shadow(
                open[idx.saturating_sub(tot_idx)],
                high[idx.saturating_sub(tot_idx)],
                close[idx.saturating_sub(tot_idx)],
            ) - upper_shadow(
                open[shadow_long_trailing_idx.saturating_sub(tot_idx)],
                high[shadow_long_trailing_idx.saturating_sub(tot_idx)],
                close[shadow_long_trailing_idx.saturating_sub(tot_idx)],
            );
        }

        for tot_idx in (1..=2).rev() {
            far_period_total[tot_idx] += real_body(
                open[idx.saturating_sub(tot_idx)],
                close[idx.saturating_sub(tot_idx)],
            ) - real_body(
                open[far_trailing_idx.saturating_sub(tot_idx)],
                close[far_trailing_idx.saturating_sub(tot_idx)],
            );
            near_period_total[tot_idx] += real_body(
                open[idx.saturating_sub(tot_idx)],
                close[idx.saturating_sub(tot_idx)],
            ) - real_body(
                open[near_trailing_idx.saturating_sub(tot_idx)],
                close[near_trailing_idx.saturating_sub(tot_idx)],
            );
        }

        body_long_period_total += real_body(open[idx - 2], close[idx - 2])
            - real_body(
                open[body_long_trailing_idx.saturating_sub(2)],
                close[body_long_trailing_idx.saturating_sub(2)],
            );

        idx += 1;
        shadow_short_trailing_idx += 1;
        shadow_long_trailing_idx += 1;
        near_trailing_idx += 1;
        far_trailing_idx += 1;
        body_long_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdlbelthold(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_long_period = 10;
    let shadow_very_short_period = 10;
    let lookback_total = body_long_period.max(shadow_very_short_period);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_long_period_total = 0.0;
    let mut shadow_very_short_period_total = 0.0;

    #[inline(always)]
    fn lower_shadow(o: f64, l: f64, c: f64) -> f64 {
        if c >= o {
            o - l
        } else {
            c - l
        }
    }

    #[inline(always)]
    fn upper_shadow(o: f64, h: f64, c: f64) -> f64 {
        if c >= o {
            h - c
        } else {
            h - o
        }
    }

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let mut start_idx = lookback_total;
    let mut body_long_trailing_idx = start_idx.saturating_sub(body_long_period);
    let mut shadow_very_short_trailing_idx = start_idx.saturating_sub(shadow_very_short_period);

    let mut i = body_long_trailing_idx;
    while i < start_idx {
        body_long_period_total += real_body(open[i], close[i]);
        i += 1;
    }

    i = shadow_very_short_trailing_idx;
    while i < start_idx {
        let color = candle_color(open[i], close[i]);
        shadow_very_short_period_total += if color == 1 {
            lower_shadow(open[i], low[i], close[i])
        } else {
            upper_shadow(open[i], high[i], close[i])
        };
        i += 1;
    }

    while start_idx < size {
        let color = candle_color(open[start_idx], close[start_idx]);
        if real_body(open[start_idx], close[start_idx])
            > candle_average(body_long_period_total, body_long_period)
            && ((color == 1
                && lower_shadow(open[start_idx], low[start_idx], close[start_idx])
                    < candle_average(shadow_very_short_period_total, shadow_very_short_period))
                || (color == -1
                    && upper_shadow(open[start_idx], high[start_idx], close[start_idx])
                        < candle_average(shadow_very_short_period_total, shadow_very_short_period)))
        {
            out[start_idx] = (color as i8) * 100;
        }

        body_long_period_total += real_body(open[start_idx], close[start_idx])
            - real_body(open[body_long_trailing_idx], close[body_long_trailing_idx]);

        let trailing_color = candle_color(open[start_idx], close[start_idx]);
        let new_range = if trailing_color == 1 {
            lower_shadow(open[start_idx], low[start_idx], close[start_idx])
        } else {
            upper_shadow(open[start_idx], high[start_idx], close[start_idx])
        };
        let old_range_color = candle_color(
            open[shadow_very_short_trailing_idx],
            close[shadow_very_short_trailing_idx],
        );
        let old_range = if old_range_color == 1 {
            lower_shadow(
                open[shadow_very_short_trailing_idx],
                low[shadow_very_short_trailing_idx],
                close[shadow_very_short_trailing_idx],
            )
        } else {
            upper_shadow(
                open[shadow_very_short_trailing_idx],
                high[shadow_very_short_trailing_idx],
                close[shadow_very_short_trailing_idx],
            )
        };

        shadow_very_short_period_total += new_range - old_range;

        start_idx += 1;
        body_long_trailing_idx += 1;
        shadow_very_short_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdlbreakaway(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_long_period = 10;
    let lookback_total = 4 + body_long_period;

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_long_period_total = 0.0;

    #[inline(always)]
    fn candle_range(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    #[inline(always)]
    fn gap_up(op_curr: f64, cl_curr: f64, op_prev: f64, cl_prev: f64) -> bool {
        op_curr.min(cl_curr) > op_prev.max(cl_prev)
    }

    #[inline(always)]
    fn gap_down(op_curr: f64, cl_curr: f64, op_prev: f64, cl_prev: f64) -> bool {
        op_curr.max(cl_curr) < op_prev.min(cl_prev)
    }

    let mut start_idx = lookback_total;
    let mut body_long_trailing_idx = start_idx.saturating_sub(body_long_period);

    let mut i = body_long_trailing_idx;
    while i < start_idx {
        body_long_period_total += candle_range(open[i - 4], close[i - 4]);
        i += 1;
    }

    while start_idx < size {
        let first_long = (close[start_idx - 4] - open[start_idx - 4]).abs()
            > body_long_period_total / body_long_period as f64;
        let c1 = candle_color(open[start_idx - 4], close[start_idx - 4]);
        let c2 = candle_color(open[start_idx - 3], close[start_idx - 3]);
        let c3 = candle_color(open[start_idx - 2], close[start_idx - 2]);
        let c4 = candle_color(open[start_idx - 1], close[start_idx - 1]);
        let c5 = candle_color(open[start_idx], close[start_idx]);

        if first_long
            && c1 == c2
            && c2 == c4
            && c4 == -c5
            && ((c1 == -1
                && gap_down(
                    open[start_idx - 3],
                    close[start_idx - 3],
                    open[start_idx - 4],
                    close[start_idx - 4],
                )
                && high[start_idx - 2] < high[start_idx - 3]
                && low[start_idx - 2] < low[start_idx - 3]
                && high[start_idx - 1] < high[start_idx - 2]
                && low[start_idx - 1] < low[start_idx - 2]
                && close[start_idx] > open[start_idx - 3]
                && close[start_idx] < close[start_idx - 4])
                || (c1 == 1
                    && gap_up(
                        open[start_idx - 3],
                        close[start_idx - 3],
                        open[start_idx - 4],
                        close[start_idx - 4],
                    )
                    && high[start_idx - 2] > high[start_idx - 3]
                    && low[start_idx - 2] > low[start_idx - 3]
                    && high[start_idx - 1] > high[start_idx - 2]
                    && low[start_idx - 1] > low[start_idx - 2]
                    && close[start_idx] < open[start_idx - 3]
                    && close[start_idx] > close[start_idx - 4]))
        {
            out[start_idx] = (c5 as i8) * 100;
        }

        body_long_period_total += candle_range(open[start_idx - 4], close[start_idx - 4])
            - candle_range(
                open[body_long_trailing_idx - 4],
                close[body_long_trailing_idx - 4],
            );

        start_idx += 1;
        body_long_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdlclosingmarubozu(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_long_period = 10;
    let shadow_very_short_period = 10;
    let lookback_total = body_long_period.max(shadow_very_short_period);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_long_period_total = 0.0;
    let mut shadow_very_short_period_total = 0.0;

    #[inline(always)]
    fn lower_shadow(o: f64, l: f64, c: f64) -> f64 {
        if c >= o {
            o - l
        } else {
            c - l
        }
    }

    #[inline(always)]
    fn upper_shadow(o: f64, h: f64, c: f64) -> f64 {
        if c >= o {
            h - c
        } else {
            h - o
        }
    }

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let mut start_idx = lookback_total;
    let mut body_long_trailing_idx = start_idx.saturating_sub(body_long_period);
    let mut shadow_very_short_trailing_idx = start_idx.saturating_sub(shadow_very_short_period);

    let mut i = body_long_trailing_idx;
    while i < start_idx {
        body_long_period_total += real_body(open[i], close[i]);
        i += 1;
    }

    i = shadow_very_short_trailing_idx;
    while i < start_idx {
        let color = candle_color(open[i], close[i]);
        shadow_very_short_period_total += if color == 1 {
            upper_shadow(open[i], high[i], close[i])
        } else {
            lower_shadow(open[i], low[i], close[i])
        };
        i += 1;
    }

    while start_idx < size {
        let color = candle_color(open[start_idx], close[start_idx]);
        if real_body(open[start_idx], close[start_idx])
            > candle_average(body_long_period_total, body_long_period)
            && ((color == 1
                && upper_shadow(open[start_idx], high[start_idx], close[start_idx])
                    < candle_average(shadow_very_short_period_total, shadow_very_short_period))
                || (color == -1
                    && lower_shadow(open[start_idx], low[start_idx], close[start_idx])
                        < candle_average(shadow_very_short_period_total, shadow_very_short_period)))
        {
            out[start_idx] = (color as i8) * 100;
        }

        body_long_period_total += real_body(open[start_idx], close[start_idx])
            - real_body(open[body_long_trailing_idx], close[body_long_trailing_idx]);

        let trailing_color = candle_color(open[start_idx], close[start_idx]);
        let new_shadow = if trailing_color == 1 {
            upper_shadow(open[start_idx], high[start_idx], close[start_idx])
        } else {
            lower_shadow(open[start_idx], low[start_idx], close[start_idx])
        };

        let old_color = candle_color(
            open[shadow_very_short_trailing_idx],
            close[shadow_very_short_trailing_idx],
        );
        let old_shadow = if old_color == 1 {
            upper_shadow(
                open[shadow_very_short_trailing_idx],
                high[shadow_very_short_trailing_idx],
                close[shadow_very_short_trailing_idx],
            )
        } else {
            lower_shadow(
                open[shadow_very_short_trailing_idx],
                low[shadow_very_short_trailing_idx],
                close[shadow_very_short_trailing_idx],
            )
        };

        shadow_very_short_period_total += new_shadow - old_shadow;

        start_idx += 1;
        body_long_trailing_idx += 1;
        shadow_very_short_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdlconcealbabyswall(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let shadow_very_short_period = 10;
    let lookback_total = 3 + shadow_very_short_period;

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut shadow_very_short_period_total = [0.0; 4];

    #[inline(always)]
    fn upper_shadow(o: f64, h: f64, c: f64) -> f64 {
        if c >= o {
            h - c
        } else {
            h - o
        }
    }

    #[inline(always)]
    fn lower_shadow(o: f64, l: f64, c: f64) -> f64 {
        if c >= o {
            o - l
        } else {
            c - l
        }
    }

    #[inline(always)]
    fn candle_color(o: f64, c: f64) -> i32 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    #[inline(always)]
    fn real_body_gap_down(o1: f64, c1: f64, o2: f64, c2: f64) -> bool {
        o1.max(c1) < o2.min(c2)
    }

    let mut start_idx = lookback_total;
    let mut shadow_very_short_trailing_idx = start_idx.saturating_sub(shadow_very_short_period);

    let mut i = shadow_very_short_trailing_idx;
    while i < start_idx {
        shadow_very_short_period_total[3] += upper_shadow(open[i - 3], high[i - 3], close[i - 3])
            .max(lower_shadow(open[i - 3], low[i - 3], close[i - 3]));
        shadow_very_short_period_total[2] += upper_shadow(open[i - 2], high[i - 2], close[i - 2])
            .max(lower_shadow(open[i - 2], low[i - 2], close[i - 2]));
        shadow_very_short_period_total[1] += upper_shadow(open[i - 1], high[i - 1], close[i - 1])
            .max(lower_shadow(open[i - 1], low[i - 1], close[i - 1]));
        i += 1;
    }

    while start_idx < size {
        let c1 = candle_color(open[start_idx - 3], close[start_idx - 3]);
        let c2 = candle_color(open[start_idx - 2], close[start_idx - 2]);
        let c3 = candle_color(open[start_idx - 1], close[start_idx - 1]);
        let c4 = candle_color(open[start_idx], close[start_idx]);

        let upper1 = upper_shadow(
            open[start_idx - 3],
            high[start_idx - 3],
            close[start_idx - 3],
        );
        let lower1 = lower_shadow(
            open[start_idx - 3],
            low[start_idx - 3],
            close[start_idx - 3],
        );
        let upper2 = upper_shadow(
            open[start_idx - 2],
            high[start_idx - 2],
            close[start_idx - 2],
        );
        let lower2 = lower_shadow(
            open[start_idx - 2],
            low[start_idx - 2],
            close[start_idx - 2],
        );
        let upper3 = upper_shadow(
            open[start_idx - 1],
            high[start_idx - 1],
            close[start_idx - 1],
        );

        if c1 == -1
            && c2 == -1
            && c3 == -1
            && c4 == -1
            && lower1 < candle_average(shadow_very_short_period_total[3], shadow_very_short_period)
            && upper1 < candle_average(shadow_very_short_period_total[3], shadow_very_short_period)
            && lower2 < candle_average(shadow_very_short_period_total[2], shadow_very_short_period)
            && upper2 < candle_average(shadow_very_short_period_total[2], shadow_very_short_period)
            && real_body_gap_down(
                open[start_idx - 1],
                close[start_idx - 1],
                open[start_idx - 2],
                close[start_idx - 2],
            )
            && upper3 > candle_average(shadow_very_short_period_total[1], shadow_very_short_period)
            && high[start_idx - 1] > close[start_idx - 2]
            && high[start_idx] > high[start_idx - 1]
            && low[start_idx] < low[start_idx - 1]
        {
            out[start_idx] = 100;
        }

        for tot_idx in (1..=3).rev() {
            let current_upper = upper_shadow(
                open[start_idx - tot_idx],
                high[start_idx - tot_idx],
                close[start_idx - tot_idx],
            );
            let current_lower = lower_shadow(
                open[start_idx - tot_idx],
                low[start_idx - tot_idx],
                close[start_idx - tot_idx],
            );
            let new_val = current_upper.max(current_lower);

            let old_upper = upper_shadow(
                open[shadow_very_short_trailing_idx - tot_idx],
                high[shadow_very_short_trailing_idx - tot_idx],
                close[shadow_very_short_trailing_idx - tot_idx],
            );
            let old_lower = lower_shadow(
                open[shadow_very_short_trailing_idx - tot_idx],
                low[shadow_very_short_trailing_idx - tot_idx],
                close[shadow_very_short_trailing_idx - tot_idx],
            );
            let old_val = old_upper.max(old_lower);

            shadow_very_short_period_total[tot_idx] += new_val - old_val;
        }

        start_idx += 1;
        shadow_very_short_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdlcounterattack(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_long_period = 10;
    let equal_period = 10;
    let lookback_total = 1 + body_long_period.max(equal_period);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut equal_period_total = 0.0;
    let mut body_long_period_total = [0.0; 2];

    #[inline(always)]
    fn real_body(o: f64, c: f64) -> f64 {
        (c - o).abs()
    }

    #[inline(always)]
    fn candle_color(o: f64, c: f64) -> i32 {
        if c >= o {
            1
        } else {
            -1
        }
    }

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let mut start_idx = lookback_total;
    let mut equal_trailing_idx = start_idx.saturating_sub(equal_period);
    let mut body_long_trailing_idx = start_idx.saturating_sub(body_long_period);

    let mut i = equal_trailing_idx;
    while i < start_idx {
        equal_period_total += real_body(open[i - 1], close[i - 1]);
        i += 1;
    }

    i = body_long_trailing_idx;
    while i < start_idx {
        body_long_period_total[1] += real_body(open[i - 1], close[i - 1]);
        body_long_period_total[0] += real_body(open[i], close[i]);
        i += 1;
    }

    while start_idx < size {
        let c1 = candle_color(open[start_idx - 1], close[start_idx - 1]);
        let c2 = candle_color(open[start_idx], close[start_idx]);
        let rb1 = real_body(open[start_idx - 1], close[start_idx - 1]);
        let rb2 = real_body(open[start_idx], close[start_idx]);
        let eq_avg = candle_average(equal_period_total, equal_period);
        let body1_avg = candle_average(body_long_period_total[1], body_long_period);
        let body2_avg = candle_average(body_long_period_total[0], body_long_period);

        if c1 == -c2
            && rb1 > body1_avg
            && rb2 > body2_avg
            && close[start_idx] <= close[start_idx - 1] + eq_avg
            && close[start_idx] >= close[start_idx - 1] - eq_avg
        {
            out[start_idx] = (c2 as i8) * 100;
        }

        equal_period_total += real_body(open[start_idx - 1], close[start_idx - 1])
            - real_body(open[equal_trailing_idx - 1], close[equal_trailing_idx - 1]);

        for tot_idx in (0..=1).rev() {
            body_long_period_total[tot_idx] +=
                real_body(open[start_idx - tot_idx], close[start_idx - tot_idx])
                    - real_body(
                        open[body_long_trailing_idx - tot_idx],
                        close[body_long_trailing_idx - tot_idx],
                    );
        }

        start_idx += 1;
        equal_trailing_idx += 1;
        body_long_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdldarkcloudcover(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, _, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_long_period = 10;
    let penetration = if input.params.penetration == 0.0 {
        0.5
    } else {
        input.params.penetration
    };
    let lookback_total = 1 + body_long_period;

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_long_period_total = 0.0;

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let mut start_idx = lookback_total;
    let mut body_long_trailing_idx = start_idx.saturating_sub(body_long_period);

    let mut i = body_long_trailing_idx;
    while i < start_idx {
        body_long_period_total += real_body(open[i - 1], close[i - 1]);
        i += 1;
    }

    while start_idx < size {
        if candle_color(open[start_idx - 1], close[start_idx - 1]) == 1
            && real_body(open[start_idx - 1], close[start_idx - 1])
                > candle_average(body_long_period_total, body_long_period)
            && candle_color(open[start_idx], close[start_idx]) == -1
            && open[start_idx] > high[start_idx - 1]
            && close[start_idx] > open[start_idx - 1]
            && close[start_idx]
                < close[start_idx - 1]
                    - real_body(open[start_idx - 1], close[start_idx - 1]) * penetration
        {
            out[start_idx] = -100;
        }

        body_long_period_total += real_body(open[start_idx - 1], close[start_idx - 1])
            - real_body(
                open[body_long_trailing_idx - 1],
                close[body_long_trailing_idx - 1],
            );

        start_idx += 1;
        body_long_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdldoji(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, _, _, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_doji_period = 10;
    let lookback_total = body_doji_period;

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_doji_period_total = 0.0;

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let mut start_idx = lookback_total;
    let mut body_doji_trailing_idx = start_idx.saturating_sub(body_doji_period);

    let mut i = body_doji_trailing_idx;
    while i < start_idx {
        body_doji_period_total += candle_range(open[i], close[i]);
        i += 1;
    }

    while start_idx < size {
        let avg_body = candle_average(body_doji_period_total, body_doji_period);
        if real_body(open[start_idx], close[start_idx]) <= avg_body {
            out[start_idx] = 100;
        }

        body_doji_period_total += candle_range(open[start_idx], close[start_idx])
            - candle_range(open[body_doji_trailing_idx], close[body_doji_trailing_idx]);

        start_idx += 1;
        body_doji_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdldojistar(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_long_period = 10;
    let body_doji_period = 10;
    let lookback_total = 1 + body_long_period.max(body_doji_period);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_long_period_total = 0.0;
    let mut body_doji_period_total = 0.0;

    #[inline(always)]
    fn gap_up(current_open: f64, current_close: f64, prev_open: f64, prev_close: f64) -> bool {
        current_open.min(current_close) > prev_open.max(prev_close)
    }

    #[inline(always)]
    fn gap_down(current_open: f64, current_close: f64, prev_open: f64, prev_close: f64) -> bool {
        current_open.max(current_close) < prev_open.min(prev_close)
    }

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let mut start_idx = lookback_total;
    let mut body_long_trailing_idx = start_idx.saturating_sub(1 + body_long_period);
    let mut body_doji_trailing_idx = start_idx.saturating_sub(body_doji_period);

    let mut i = body_long_trailing_idx;
    while i < start_idx - 1 {
        body_long_period_total += real_body(open[i], close[i]);
        i += 1;
    }

    i = body_doji_trailing_idx;
    while i < start_idx {
        body_doji_period_total += real_body(open[i], close[i]);
        i += 1;
    }

    while start_idx < size {
        if real_body(open[start_idx - 1], close[start_idx - 1])
            > candle_average(body_long_period_total, body_long_period)
            && real_body(open[start_idx], close[start_idx])
                <= candle_average(body_doji_period_total, body_doji_period)
            && ((candle_color(open[start_idx - 1], close[start_idx - 1]) == 1
                && gap_up(
                    open[start_idx],
                    close[start_idx],
                    open[start_idx - 1],
                    close[start_idx - 1],
                ))
                || (candle_color(open[start_idx - 1], close[start_idx - 1]) == -1
                    && gap_down(
                        open[start_idx],
                        close[start_idx],
                        open[start_idx - 1],
                        close[start_idx - 1],
                    )))
        {
            out[start_idx] = -candle_color(open[start_idx - 1], close[start_idx - 1]) as i8 * 100;
        }

        body_long_period_total += real_body(open[start_idx - 1], close[start_idx - 1])
            - real_body(open[body_long_trailing_idx], close[body_long_trailing_idx]);

        body_doji_period_total += real_body(open[start_idx], close[start_idx])
            - real_body(open[body_doji_trailing_idx], close[body_doji_trailing_idx]);

        start_idx += 1;
        body_long_trailing_idx += 1;
        body_doji_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdldragonflydoji(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, high, low, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;

            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_doji_period = 10;
    let shadow_very_short_period = 10;
    let lookback_total = body_doji_period.max(shadow_very_short_period);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_doji_period_total = 0.0;
    let mut shadow_very_short_period_total = 0.0;

    #[inline(always)]
    fn upper_shadow(o: f64, h: f64, c: f64) -> f64 {
        if c >= o {
            h - c
        } else {
            h - o
        }
    }

    #[inline(always)]
    fn lower_shadow(o: f64, l: f64, c: f64) -> f64 {
        if c >= o {
            o - l
        } else {
            c - l
        }
    }

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    let mut start_idx = lookback_total;
    let mut body_doji_trailing_idx = start_idx.saturating_sub(body_doji_period);
    let mut shadow_very_short_trailing_idx = start_idx.saturating_sub(shadow_very_short_period);

    let mut i = body_doji_trailing_idx;
    while i < start_idx {
        body_doji_period_total += candle_range(open[i], close[i]);
        i += 1;
    }

    i = shadow_very_short_trailing_idx;
    while i < start_idx {
        shadow_very_short_period_total +=
            (upper_shadow(open[i], high[i], close[i])).max(lower_shadow(open[i], low[i], close[i]));
        i += 1;
    }

    while start_idx < size {
        let rb = real_body(open[start_idx], close[start_idx]);
        let us = upper_shadow(open[start_idx], high[start_idx], close[start_idx]);
        let ls = lower_shadow(open[start_idx], low[start_idx], close[start_idx]);
        let avg_body_doji = candle_average(body_doji_period_total, body_doji_period);
        let avg_shadow_very_short =
            candle_average(shadow_very_short_period_total, shadow_very_short_period);

        if rb <= avg_body_doji && us < avg_shadow_very_short && ls > avg_shadow_very_short {
            out[start_idx] = 100;
        }

        body_doji_period_total += candle_range(open[start_idx], close[start_idx])
            - candle_range(open[body_doji_trailing_idx], close[body_doji_trailing_idx]);

        let current_shadow_sum = (upper_shadow(open[start_idx], high[start_idx], close[start_idx]))
            .max(lower_shadow(
                open[start_idx],
                low[start_idx],
                close[start_idx],
            ));
        let trailing_shadow_sum = (upper_shadow(
            open[shadow_very_short_trailing_idx],
            high[shadow_very_short_trailing_idx],
            close[shadow_very_short_trailing_idx],
        ))
        .max(lower_shadow(
            open[shadow_very_short_trailing_idx],
            low[shadow_very_short_trailing_idx],
            close[shadow_very_short_trailing_idx],
        ));

        shadow_very_short_period_total += current_shadow_sum - trailing_shadow_sum;

        start_idx += 1;
        body_doji_trailing_idx += 1;
        shadow_very_short_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdlengulfing(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, _, _, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            (open, high, low, close)
        }
    };

    let size = open.len();
    if size < 2 {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    for i in 1..size {
        let c1 = candle_color(open[i - 1], close[i - 1]);
        let c2 = candle_color(open[i], close[i]);
        if (c2 == 1
            && c1 == -1
            && ((close[i] >= open[i - 1] && open[i] < close[i - 1])
                || (close[i] > open[i - 1] && open[i] <= close[i - 1])))
            || (c2 == -1
                && c1 == 1
                && ((open[i] >= close[i - 1] && close[i] < open[i - 1])
                    || (open[i] > close[i - 1] && close[i] <= open[i - 1])))
        {
            if (open[i] - close[i - 1]).abs() > f64::EPSILON
                && (close[i] - open[i - 1]).abs() > f64::EPSILON
            {
                out[i] = (c2 as i8) * 100;
            } else {
                out[i] = (c2 as i8) * 80;
            }
        }
    }

    Ok(PatternOutput { values: out })
}

#[inline]
pub fn cdleveningdojistar(input: &PatternInput) -> Result<PatternOutput, PatternError> {
    let (open, _, _, close) = match &input.data {
        PatternData::Candles { candles } => {
            let open = candles
                .select_candle_field("open")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let high = candles
                .select_candle_field("high")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let low = candles
                .select_candle_field("low")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            let close = candles
                .select_candle_field("close")
                .map_err(|e| PatternError::CandleFieldError(e.to_string()))?;
            (open, high, low, close)
        }
    };

    let size = open.len();
    let body_long_period = 10;
    let body_doji_period = 10;
    let body_short_period = 10;
    let penetration = if input.params.penetration == 0.0 {
        0.3
    } else {
        input.params.penetration
    };
    let lookback_total = 2 + body_long_period
        .max(body_doji_period)
        .max(body_short_period);

    if size < lookback_total {
        return Err(PatternError::NotEnoughData {
            len: size,
            pattern: input.params.pattern_type.clone(),
        });
    }

    let mut out = vec![0i8; size];
    let mut body_long_period_total = 0.0;
    let mut body_doji_period_total = 0.0;
    let mut body_short_period_total = 0.0;

    #[inline(always)]
    fn candle_average(sum: f64, period: usize) -> f64 {
        if period == 0 {
            0.0
        } else {
            sum / period as f64
        }
    }

    #[inline(always)]
    fn gap_up(current_open: f64, current_close: f64, prev_open: f64, prev_close: f64) -> bool {
        current_open.min(current_close) > prev_open.max(prev_close)
    }

    let mut start_idx = lookback_total;
    let mut body_long_trailing_idx = start_idx.saturating_sub(2 + body_long_period);
    let mut body_doji_trailing_idx = start_idx.saturating_sub(1 + body_doji_period);
    let mut body_short_trailing_idx = start_idx.saturating_sub(body_short_period);

    let mut i = body_long_trailing_idx;
    while i < start_idx - 2 {
        body_long_period_total += real_body(open[i], close[i]);
        i += 1;
    }

    i = body_doji_trailing_idx;
    while i < start_idx - 1 {
        body_doji_period_total += real_body(open[i], close[i]);
        i += 1;
    }

    i = body_short_trailing_idx;
    while i < start_idx {
        body_short_period_total += real_body(open[i], close[i]);
        i += 1;
    }

    while start_idx < size {
        if real_body(open[start_idx - 2], close[start_idx - 2])
            > candle_average(body_long_period_total, body_long_period)
            && candle_color(open[start_idx - 2], close[start_idx - 2]) == 1
            && real_body(open[start_idx - 1], close[start_idx - 1])
                <= candle_average(body_doji_period_total, body_doji_period)
            && gap_up(
                open[start_idx - 1],
                close[start_idx - 1],
                open[start_idx - 2],
                close[start_idx - 2],
            )
            && real_body(open[start_idx], close[start_idx])
                > candle_average(body_short_period_total, body_short_period)
            && candle_color(open[start_idx], close[start_idx]) == -1
            && close[start_idx]
                < close[start_idx - 2]
                    - real_body(open[start_idx - 2], close[start_idx - 2]) * penetration
        {
            out[start_idx] = -100;
        }

        body_long_period_total += real_body(open[start_idx - 2], close[start_idx - 2])
            - real_body(open[body_long_trailing_idx], close[body_long_trailing_idx]);

        body_doji_period_total += real_body(open[start_idx - 1], close[start_idx - 1])
            - real_body(open[body_doji_trailing_idx], close[body_doji_trailing_idx]);

        body_short_period_total += real_body(open[start_idx], close[start_idx])
            - real_body(
                open[body_short_trailing_idx],
                close[body_short_trailing_idx],
            );

        start_idx += 1;
        body_long_trailing_idx += 1;
        body_doji_trailing_idx += 1;
        body_short_trailing_idx += 1;
    }

    Ok(PatternOutput { values: out })
}
