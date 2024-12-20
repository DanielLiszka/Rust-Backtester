extern crate criterion;
extern crate lazy_static;
extern crate my_project;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use my_project::indicators::data_loader::{read_candles_from_csv, Candles};

use my_project::indicators::{
    acosc::{calculate_acosc, AcoscInput},
    ad::{calculate_ad, AdInput},
    adx::{calculate_adx, AdxInput},
    adxr::{calculate_adxr, AdxrInput},
    alligator::{calculate_alligator, AlligatorInput},
    ema::{calculate_ema, EmaInput},
    rsi::{calculate_rsi, RsiInput},
    sma::{calculate_sma, SmaInput},
    zlema::{calculate_zlema, ZlemaInput},
    adosc::{calculate_adosc, AdoscInput},
    alma::{calculate_alma, AlmaInput},
    ao::{calculate_ao, AoInput},
    apo::{calculate_apo, ApoInput},
    aroon::{calculate_aroon, AroonInput},
    aroonosc::{calculate_aroon_osc, AroonOscInput},
    atr::{calculate_atr, AtrInput},
    avgprice::{calculate_avgprice, AvgPriceInput},
    highpass::{calculate_highpass, HighPassInput},
    bandpass::{calculate_bandpass, BandPassInput},
};
use std::time::Duration;

fn benchmark_indicators(c: &mut Criterion) {
    // Load candles once
    let candles = read_candles_from_csv("src/data/bitfinex btc-usd 100,000 candles ends 09-01-24.csv")
        .expect("Failed to load candles");

    // Pre-extract derived data if needed
    // But now we always construct Inputs directly from Candles or slices
    let close_prices = candles
        .select_candle_field("close")
        .expect("Failed to extract close prices");
    let hl2_prices = candles
        .get_calculated_field("hl2")
        .expect("Failed to extract hl2 prices");

    let mut group = c.benchmark_group("Indicator Benchmarks");
    group.measurement_time(Duration::new(10, 0));
    group.warm_up_time(Duration::new(5, 0));

    // BANDPASS
    group.bench_function(BenchmarkId::new("BANDPASS", 0), |b| {
        let input = BandPassInput::with_default_params(&close_prices);
        b.iter(|| calculate_bandpass(black_box(&input)).expect("Failed to calculate BANDPASS"))
    });

    // HIGHPASS
    group.bench_function(BenchmarkId::new("HIGHPASS", 0), |b| {
        let input = HighPassInput::with_default_params(&close_prices);
        b.iter(|| calculate_highpass(black_box(&input)).expect("Failed to calculate HIGHPASS"))
    });

    // AVGPRICE
    group.bench_function(BenchmarkId::new("AVGPRICE", 0), |b| {
        let input = AvgPriceInput::with_default_params(&candles);
        b.iter(|| calculate_avgprice(&input).expect("Failed to calculate AVGPRICE"))
    });

    // ATR  
    group.bench_function(BenchmarkId::new("ATR", 0), |b| {
        let input = AtrInput::with_default_params(&candles);
        b.iter(|| calculate_atr(black_box(&input)).expect("Failed to calculate ATR"))
    });

    // AROONOSC
    group.bench_function(BenchmarkId::new("AROONOSC", 0), |b| {
        let input = AroonOscInput::with_default_params(&candles);
        b.iter(|| calculate_aroon_osc(black_box(&input)).expect("Failed to calculate AROONOSC"))
    });

    // AROON
    group.bench_function(BenchmarkId::new("AROON", 0), |b| {
        let input = AroonInput::with_default_params(&candles);
        b.iter(|| calculate_aroon(black_box(&input)).expect("Failed to calculate AROON"))
    });

    // APO
    group.bench_function(BenchmarkId::new("APO", 0), |b| {
        let input = ApoInput::with_default_params(&candles);
        b.iter(|| calculate_apo(black_box(&input)).expect("Failed to calculate APO"))
    });

    // AO
    group.bench_function(BenchmarkId::new("AO", 0), |b| {
        let input = AoInput::with_default_params(&candles);
        b.iter(|| calculate_ao(black_box(&input)).expect("Failed to calculate AO"))
    });
    
    // ALMA
    group.bench_function(BenchmarkId::new("ALMA", 0), |b| {
        let input = AlmaInput::with_default_params(&close_prices);
        b.iter(|| calculate_alma(black_box(&input)).expect("Failed to calculate ALMA"))
    });

    // ADOSC
    group.bench_function(BenchmarkId::new("ADOSC", 0), |b| {
        let input = AdoscInput::with_default_params(&candles);
        b.iter(|| calculate_adosc(black_box(&input)).expect("Failed to calculate ADOSC"))
    });


    // ZLEMA
    group.bench_function(BenchmarkId::new("ZLEMA", 0), |b| {
        let input = ZlemaInput::with_default_params(&close_prices);
        b.iter(|| calculate_zlema(black_box(&input)).expect("Failed to calculate ZLEMA"))
    });

    // Alligator
    group.bench_function(BenchmarkId::new("ALLIGATOR", 0), |b| {
        let input = AlligatorInput::with_default_params(&hl2_prices);
        b.iter(|| calculate_alligator(black_box(&input)).expect("Failed to calculate alligator"))
    });

    // ADXR
    group.bench_function(BenchmarkId::new("ADXR", 0), |b| {
        let input = AdxrInput::with_default_params(&candles);
        b.iter(|| calculate_adxr(black_box(&input)).expect("Failed to calculate ADXR"))
    });

    // ADX
    group.bench_function(BenchmarkId::new("ADX", 0), |b| {
        let input = AdxInput::with_default_params(&candles);
        b.iter(|| calculate_adx(black_box(&input)).expect("Failed to calculate ADX"))
    });

    // SMA
    group.bench_function(BenchmarkId::new("SMA", 0), |b| {
        let input = SmaInput::with_default_params(&close_prices);
        b.iter(|| calculate_sma(black_box(&input)).expect("Failed to calculate SMA"))
    });

    // EMA
    group.bench_function(BenchmarkId::new("EMA", 0), |b| {
        let input = EmaInput::with_default_params(&close_prices);
        b.iter(|| calculate_ema(black_box(&input)).expect("Failed to calculate EMA"))
    });

    // RSI
    group.bench_function(BenchmarkId::new("RSI", 0), |b| {
        let input = RsiInput::with_default_params(&close_prices);
        b.iter(|| calculate_rsi(black_box(&input)).expect("Failed to calculate RSI"))
    });

    // ACOSC
    group.bench_function(BenchmarkId::new("ACOSC", 0), |b| {
        let input = AcoscInput::with_default_params(&candles);
        b.iter(|| calculate_acosc(black_box(&input)).expect("Failed to calculate ACOSC"))
    });

    // AD
    group.bench_function(BenchmarkId::new("AD", 0), |b| {
        let input = AdInput::with_default_params(&candles);
        b.iter(|| calculate_ad(black_box(&input)).expect("Failed to calculate AD"))
    });

    group.finish();
}

criterion_group!(benches, benchmark_indicators);
criterion_main!(benches);
