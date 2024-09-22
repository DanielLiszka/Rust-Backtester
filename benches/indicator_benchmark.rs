extern crate criterion;
extern crate my_project;
extern crate lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use my_project::indicators::{ema::calculate_ema, sma::calculate_sma, rsi::calculate_rsi, acosc::calculate_acosc};
use my_project::indicators::data_loader::{BENCH_CANDLES, select_candle_field};
use std::time::Duration;

fn benchmark_indicators(c: &mut Criterion) {
    // Define periods for each indicator
    let period_sma = 9;
    let period_ema = 9;
    let period_rsi = 14;

    // Access the loaded candles directly
    let candles = &*BENCH_CANDLES;

    // Pre-extract the "close" field once before benchmarking
    let close_prices = select_candle_field(candles, "close")
        .expect("Failed to extract close prices");

    let mut group = c.benchmark_group("Indicator Benchmarks");
    group.measurement_time(Duration::new(4, 0));
    group.warm_up_time(Duration::new(2, 0));

    // Benchmark SMA
    group.bench_function(BenchmarkId::new("SMA_close_9", period_sma), |b| {
        b.iter(|| {
            calculate_sma(
                black_box(&close_prices),
                black_box(period_sma),
            )
            .expect("Failed to calculate SMA")
        })
    });

    // Benchmark EMA
    group.bench_function(BenchmarkId::new("EMA_close_9", period_ema), |b| {
        b.iter(|| {
            calculate_ema(
                black_box(&close_prices),
                black_box(period_ema),
            )
            .expect("Failed to calculate EMA")
        })
    });

    // Benchmark RSI
    group.bench_function(BenchmarkId::new("RSI_close_14", period_rsi), |b| {
        b.iter(|| {
            calculate_rsi(
                black_box(&close_prices),
                black_box(period_rsi),
            )
            .expect("Failed to calculate RSI")
        })
    });

    // Benchmark ACOSC
    group.bench_function(BenchmarkId::new("ACOSC", 0), |b| {
        b.iter(|| {
            calculate_acosc(
                black_box(&candles),
            )
            .expect("Failed to calculate ACOSC")
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_indicators);
criterion_main!(benches);