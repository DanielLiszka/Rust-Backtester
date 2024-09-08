extern crate criterion;
extern crate my_project;
extern crate csv;
extern crate serde;

use criterion::{Criterion, criterion_group, criterion_main};
use my_project::indicators::{sma::calculate_sma, ema::calculate_ema};  // Assuming you have both in their respective modules
use serde::Deserialize;
use std::fs::File;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};

// Global counter to track how many times data is loaded
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

    // Increment the counter each time the function is called
    LOAD_COUNTER.fetch_add(1, Ordering::SeqCst);

    Ok(candles)
}

// Preload the candles globally before any benchmarks are run
lazy_static::lazy_static! {
    static ref CANDLES: Vec<f64> = {
        let candles = read_candles_from_csv("C:/Users/dlisz/Desktop/Rust Projects/First Rust Project/my_project/src/bitfinex btc-usd 100,000 candles ends 09-01-24.csv")
            .expect("Failed to load candles");
        println!("Candles loaded: {}", candles.len()); // Print to confirm candles loaded
        println!("Candles loaded {} times.", LOAD_COUNTER.load(Ordering::SeqCst)); // Confirm loading count
        candles.iter().map(|c| c.close).collect()
    };
}
// Benchmark function for SMA
fn benchmark_sma(c: &mut Criterion) {
    let period = 200;

    // Use the preloaded global CANDLES data
    c.bench_function("SMA", |b| {
        b.iter(|| calculate_sma(&CANDLES, period))
    });
}
// Benchmark function for EMA
fn benchmark_ema(c: &mut Criterion) {
    let period = 200;

    // Use the preloaded global CANDLES data
    c.bench_function("EMA", |b| {
        b.iter(|| calculate_ema(&CANDLES, period))
    });
}
// Create the benchmark group
criterion_group!(benches, benchmark_sma, benchmark_ema);
// Register the benchmark group for execution
criterion_main!(benches);
