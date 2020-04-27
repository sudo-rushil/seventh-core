//! Retrieve data from Coinbase and CoinGecko APIs

use std::fmt;
use chrono::Utc;
use phf::{Map, phf_map};

const UNIX_DAY: i64 = 86400;

static COIN_ID: Map<&'static str, &'static str> = phf_map! {
    "BTC" => "bitcoin",
    "ETH" => "ethereum",
    "ETC" => "ethereum-classic",
    "LTC" => "litecoin",
};

#[derive(Deserialize, Debug)]
struct Price {
    base: String,
    currency: String,
    amount: String,
}

#[derive(Deserialize, Debug)]
pub struct PriceData {
    data: Price
}

#[derive(Deserialize, Debug)]
pub struct HistoricalData {
    prices: Vec<Vec<f32>>,
    total_volumes: Vec<Vec<f32>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct CoinData {
    ticker: String,
    historical: Vec<f32>,
    buy: f32,
    sell: f32,
    spot: f32,
}

impl CoinData {
    fn new(buyprice: Price, sellprice: Price, spotprice: Price, history: HistoricalData) -> Self {
        CoinData {
            ticker: spotprice.base,
            historical: history.prices.into_iter().map(|v| v[1]).collect(),
            buy: buyprice.amount.parse().unwrap(),
            sell: sellprice.amount.parse().unwrap(),
            spot: spotprice.amount.parse().unwrap()
        }
    }
}

impl fmt::Display for CoinData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CoinData: ticker {}, buy {}, sell {}, spot {}",
            self.ticker,
            self.buy,
            self.sell,
            self.spot
        )
    }
}

pub struct DataAPI {
    client: reqwest::Client,
    coins: Vec<CoinData>
}

impl DataAPI {
    #[tokio::main]
    async fn get<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: String) -> Result<T, reqwest::Error> {
        let response = self.client.get(&endpoint).send().await?;
        let result: T = response.json().await?;

        Ok(result)
    }

    fn get_coinbase(&self, coin: &str, query: &str) -> Price {
        let endpoint = format!("https://api.coinbase.com/v2/prices/{}-USD/{}", coin, query);
        let result = self.get::<PriceData>(endpoint.clone());

        match result {
            Ok(price_data) => price_data.data,
            Err(e) => panic!("Error calling Coinbase API: {}", e)
        }
    }

    fn get_historical(&self, coin: &str) -> HistoricalData {
        let curr_time = Utc::now().timestamp();
        let endpoint = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart/range?vs_currency=usd&from={}&to={}", coin, curr_time - UNIX_DAY, curr_time);
        let result = self.get::<HistoricalData>(endpoint);

        match result {
            Ok(historical_data) => historical_data,
            Err(e) => panic!("Error calling CoinGecko API: {}", e)
        }
    }

    pub fn update(&mut self, coin: &str) {
        let buy = self.get_coinbase(coin, "buy");
        let sell = self.get_coinbase(coin, "sell");
        let spot = self.get_coinbase(coin, "spot");
        let history = self.get_historical(COIN_ID[coin]);
        self.coins.push(CoinData::new(buy, sell, spot, history))
    }

    pub fn new() -> Self {
        DataAPI { client: reqwest::Client::new(), coins: vec![] }
    }

    pub fn coins(&self) -> Vec<CoinData> {
        self.coins.clone()
    }

}
