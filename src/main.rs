#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod indicators; 

//use indicators::sma::simple_moving_average;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}