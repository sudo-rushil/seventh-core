//! Historical backtesting system

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

use crate::backtrader::{Actions, TraderData};

#[derive(Debug, Clone)]
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

    fn take_slice(&self, size: usize) -> RawData {
        let len = self.open.len();

        RawData {
            open: self.open[len - size..len].to_vec(),
            high: self.high[len - size..len].to_vec(),
            low: self.low[len - size..len].to_vec(),
            close: self.close[len - size..len].to_vec(),
        }
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
    Ok(out.take_slice(1000))
}

pub struct Histtrader {
    data: RawData,
    range: usize,
    current: (usize, Vec<f32>, f32, f32), // (position, hist, buy, sell)
    ticker: String,
    history: Vec<(f32, Actions)>,
    account: f32,  // in usd
    holdings: f32, // in stock units
}

impl Histtrader {
    pub fn new(data: RawData, ticker: &str, account: f32, start: usize) -> Self {
        println!(
            "Initializing Histtrader: Max iterations {}",
            data.close.len() - start
        );
        Histtrader {
            data: data.clone(),
            range: start,
            current: (
                start,
                data.close[..=start].to_vec(),
                data.low[start],
                data.high[start],
            ),
            ticker: ticker.to_owned(),
            history: vec![],
            account,
            holdings: 0.0,
        }
    }

    fn update(&mut self) {
        let new_start = self.current.0 + 1;
        self.current = (
            new_start,
            self.data.close[new_start - self.range..=new_start].to_vec(),
            self.data.low[new_start],
            self.data.high[new_start],
        )
    }

    pub fn reset(&mut self, account: f32, ticker: &str) {
        self.history = vec![];
        self.account = account;
        self.ticker = ticker.to_owned();
        self.holdings = 0.0;
    }

    pub fn trade(&mut self, action: Actions) {
        println!("Making trade");
        self.update();
        self.history.push((self.account, action.clone()));

        match action {
            Actions::Buy(usd) => self.buy(usd),
            Actions::Sell(coin) => self.sell(coin),
            Actions::Hold => return,
        };
    }

    fn buy(&mut self, mut usd: f32) {
        if usd > self.account {
            usd = self.account;
        }
        let price = self.current.2;

        self.account -= usd;
        self.holdings += usd / price;
    }

    fn sell(&mut self, item: f32) {
        let price = self.current.3;

        self.account += item * price;
        self.holdings -= item;
    }

    pub fn data(&self) -> TraderData {
        TraderData {
            historical: self.current.1.clone(),
            buy: self.current.2,
            sell: self.current.3,
            account: self.account,
            holding: self.holdings,
        }
    }

    pub fn history(&self) -> Vec<(f32, Actions)> {
        self.history.clone()
    }
}
