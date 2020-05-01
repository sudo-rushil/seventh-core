//! Historical backtesting system

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

#[derive(Debug)]
pub struct RawData {
    open: Vec<f32>,
    high: Vec<f32>,
    low: Vec<f32>,
    close: Vec<f32>,
}

impl RawData {
    fn new() -> Self {
        RawData {
            open: vec![],
            high: vec![],
            low: vec![],
            close: vec![],
        }
    }

    fn add_record(&mut self, rec: csv::StringRecord) {
        self.open.push(rec.get(1).unwrap().parse().unwrap());
        self.high.push(rec.get(2).unwrap().parse().unwrap());
        self.low.push(rec.get(3).unwrap().parse().unwrap());
        self.close.push(rec.get(4).unwrap().parse().unwrap());
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

pub fn run() -> Result<RawData, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut out = RawData::new();

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        out.add_record(record);
    }
    Ok(out)
}
