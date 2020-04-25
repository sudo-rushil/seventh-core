//! Retrieve data from Coinbase and CoinGecko APIs

use std::fmt;

/// Response types

/// endpoints:
/// coin/marketchart/range
/// prices - buy, sell, spot
/// time

#[derive(Deserialize, Debug)]
struct Price {
    base: String,
    currency: String,
    amount: String,
}

#[derive(Deserialize, Debug)]
struct PriceData {
    data: Price
}

pub struct CoinData {
    ticker: String,
    historical: Vec<f32>,
    buy: f32,
    sell: f32,
    spot: f32,
}

pub struct DataAPI {
    client: reqwest::Client,
    coins: Vec<CoinData>
}

impl DataAPI {
    #[tokio::main]
    async fn get<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str) -> Result<T, reqwest::Error> {
        let response = self.client.get(endpoint).send().await?;
        let result: T = response.json().await?;

        Ok(result)
    }

    pub fn new() -> Self {
        DataAPI { client: reqwest::Client::new(), coins: vec![CoinData { ticker: "BTC".to_owned(), historical: vec![2.0], buy: 2.0, sell: 2.0, spot: 2.0 }] }
    }
}

impl fmt::Display for CoinData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataAPI: buy {}, sell {}, spot {}", self.buy, self.sell, self.spot)
    }
}
