extern crate criterion;
extern crate csv;
extern crate my_project;
extern crate serde;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use my_project::indicators::{ema::calculate_ema, sma::calculate_sma};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static LOAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Deserialize)]
pub struct Candle {
    pub time: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

fn read_candles_from_csv(file_path: &str) -> Result<Vec<Candle>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut candles = Vec::new();
    for result in rdr.deserialize() {
        let record: Candle = result?;
        candles.push(record);
    }

    LOAD_COUNTER.fetch_add(1, Ordering::SeqCst);

    Ok(candles)
}

lazy_static::lazy_static! {
    static ref CANDLES: Vec<f64> = {
        let candles = read_candles_from_csv("C:/Users/dlisz/Desktop/Rust Projects/First Rust Project/my_project/src/bitfinex btc-usd 100,000 candles ends 09-01-24.csv")
            .expect("Failed to load candles");
        println!("Candles loaded: {}", candles.len());
        println!("Candles loaded {} times.", LOAD_COUNTER.load(Ordering::SeqCst));
        candles.iter().map(|c| c.close).collect()
    };
}

fn benchmark_indicators(c: &mut Criterion) {
    let period = 200;

    let mut group = c.benchmark_group("Indicator Benchmarks");
    group.measurement_time(Duration::new(2, 0));
    group.warm_up_time(Duration::new(1, 0));

    group.bench_function(BenchmarkId::new("SMA", period), |b| {
        b.iter(|| calculate_sma(&CANDLES, period))
    });

    group.bench_function(BenchmarkId::new("EMA", period), |b| {
        b.iter(|| calculate_ema(&CANDLES, period))
    });

    group.finish();
}

criterion_group!(benches, benchmark_indicators);
criterion_main!(benches);