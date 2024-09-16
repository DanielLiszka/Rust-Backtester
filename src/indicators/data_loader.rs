use lazy_static::lazy_static;  // Import lazy_static macro
use std::sync::Mutex;          // Import Mutex for safe concurrent access to shared data
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

lazy_static! {
    pub static ref TEST_CLOSE_PRICES: Mutex<Vec<f64>> = Mutex::new(load_close_prices().unwrap());
}

fn load_close_prices() -> Result<Vec<f64>, Box<dyn Error>> {
    let file_path = "src/2018-09-01-2024-Bitfinex_Spot-4h.csv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut close_prices = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let close_price: f64 = record[3].parse()?;  // Parse close price from string to f64
        close_prices.push(close_price);
    }
    Ok(close_prices)
}