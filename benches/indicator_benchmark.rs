extern crate criterion;
extern crate lazy_static;
extern crate my_project;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use my_project::indicators::data_loader::{read_candles_from_csv, Candles};
use my_project::indicators::{
    acosc::calculate_acosc, ad::calculate_ad, ema::calculate_ema, rsi::calculate_rsi,
    sma::calculate_sma, adx::calculate_adx, adxr::calculate_adxr, alligator::calculate_alligator,
};
use std::time::Duration;

fn benchmark_indicators(c: &mut Criterion) {
    // Define periods for each indicator
    let period_sma: usize = 9;
    let period_ema: usize = 9;
    let period_rsi: usize = 14;
    let period_adx: usize = 14;
    let period_adxr: usize = 14;

    let candles = read_candles_from_csv("src/data/bitfinex btc-usd 100,000 candles ends 09-01-24.csv")
        .expect("Failed to load candles");

    // Pre-extract the "close" field once before benchmarking
    let close_prices = candles
        .select_candle_field("close")
        .expect("Failed to extract close prices");
    let hl2_prices = candles
        .get_calculated_field("hl2")
        .expect("Failed to extract hl2 prices");

    let mut group = c.benchmark_group("Indicator Benchmarks");
    group.measurement_time(Duration::new(10, 0));
    group.warm_up_time(Duration::new(5, 0));

    // Benchmark Alligator
    group.bench_function(BenchmarkId::new("alligator", 0), |b| {
        b.iter(|| calculate_alligator(black_box(&hl2_prices)).expect("Failed to calculate alligator"))
    });

    // Benchmark ADXR
    group.bench_function(BenchmarkId::new("ADXR", 0), |b| {
        b.iter(|| calculate_adxr(black_box(&candles), black_box(period_adxr)).expect("Failed to calculate ADXR"))
    });

    // Benchmark ADX
    group.bench_function(BenchmarkId::new("ADX", 0), |b| {
        b.iter(|| calculate_adx(black_box(&candles), black_box(period_adx)).expect("Failed to calculate ADX"))
    });

    // Benchmark SMA
    group.bench_function(BenchmarkId::new("SMA_close_9", period_sma), |b| {
        b.iter(|| {
            calculate_sma(black_box(&close_prices), black_box(period_sma))
                .expect("Failed to calculate SMA")
        })
    });

    // Benchmark EMA
    group.bench_function(BenchmarkId::new("EMA_close_9", period_ema), |b| {
        b.iter(|| {
            calculate_ema(black_box(&close_prices), black_box(period_ema))
                .expect("Failed to calculate EMA")
        })
    });

    // Benchmark RSI
    group.bench_function(BenchmarkId::new("RSI_close_14", period_rsi), |b| {
        b.iter(|| {
            calculate_rsi(black_box(&close_prices), black_box(period_rsi))
                .expect("Failed to calculate RSI")
        })
    });

    // Benchmark ACOSC
    group.bench_function(BenchmarkId::new("ACOSC", 0), |b| {
        b.iter(|| calculate_acosc(black_box(&candles)).expect("Failed to calculate ACOSC"))
    });

    // Benchmark AD
    group.bench_function(BenchmarkId::new("AD", 0), |b| {
        b.iter(|| calculate_ad(black_box(&candles)).expect("Failed to calculate AD"))
    });

    group.finish();
}

criterion_group!(benches, benchmark_indicators);
criterion_main!(benches);


