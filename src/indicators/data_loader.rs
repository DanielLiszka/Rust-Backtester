extern crate csv;
extern crate serde;

use lazy_static::lazy_static;
use std::error::Error;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use csv::ReaderBuilder;
use ndarray::{Array2, Axis};
static LOAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
    pub static ref TEST_CANDLES: Mutex<Array2<f64>> = Mutex::new(load_candles().unwrap());
}

lazy_static! {
    pub static ref BENCH_CANDLES: Array2<f64> = {
        let candles = read_candles_from_csv("src/data/bitfinex btc-usd 100,000 candles ends 09-01-24.csv")
            .expect("Failed to load candles");
        println!("Candles loaded {} times.", LOAD_COUNTER.load(Ordering::SeqCst));
        candles
    };
}

pub fn read_candles_from_csv(file_path: &str) -> Result<Array2<f64>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records: Vec<Vec<f64>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row: Vec<f64> = vec![
            record[0].parse()?,
            record[1].parse()?,
            record[2].parse()?,
            record[3].parse()?,
            record[4].parse()?,
            record[5].parse()?,
        ];
        records.push(row);
    }

    let num_rows = records.len();
    let num_cols = records[0].len();
    let flattened_data: Vec<f64> = records.into_iter().flatten().collect();

    let candles = Array2::from_shape_vec((num_rows, num_cols), flattened_data)?;

    LOAD_COUNTER.fetch_add(1, Ordering::SeqCst);

    Ok(candles)
}

pub fn load_candles() -> Result<Array2<f64>, Box<dyn Error>> {
    let file_path = "src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records: Vec<Vec<f64>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row: Vec<f64> = vec![
            record[1].parse()?,
            record[2].parse()?,
            record[3].parse()?,
            record[4].parse()?,
            record[5].parse()?,
            record[6].parse()?,
        ];
        records.push(row);
    }

    let num_rows = records.len();
    let num_cols = records[0].len();
    let flattened_data: Vec<f64> = records.into_iter().flatten().collect();

    let candles = Array2::from_shape_vec((num_rows, num_cols), flattened_data)?;

    LOAD_COUNTER.fetch_add(1, Ordering::SeqCst);

    Ok(candles)
}

pub fn select_candle_field(candles: &Array2<f64>, field: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let column_idx = match field.to_lowercase().as_str() {
        "timestamp" => 0,
        "open" => 1,
        "high" => 3,
        "low" => 4,
        "close" => 2,
        "volume" => 5,
        "hl2" => return Ok(candles.axis_iter(Axis(0))
                               .map(|row| (row[2] + row[3]) / 2.0)
                               .collect()),
        "hlc3" => return Ok(candles.axis_iter(Axis(0))
                                .map(|row| (row[2] + row[3] + row[4]) / 3.0)
                                .collect()),
        "ohlc4" => return Ok(candles.axis_iter(Axis(0))
                                 .map(|row| (row[1] + row[2] + row[3] + row[4]) / 4.0)
                                 .collect()),
        "hlcc4" => return Ok(candles.axis_iter(Axis(0))
                                 .map(|row| (row[2] + row[3] + (row[4] * 2.0)) / 4.0)
                                 .collect()),
        _ => return Err(format!("Invalid field: {}", field).into()),
    };

    let selected_column = candles.column(column_idx).to_owned().to_vec();
    Ok(selected_column)
}