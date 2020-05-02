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
struct PriceData {
    data: Price
}

#[derive(Deserialize, Debug)]
struct HistoricalData {
    prices: Vec<Vec<f32>>,
    total_volumes: Vec<Vec<f32>>
}

/// Datatype for representing the instantaneous price data for a given cryptocurrency.
///
/// A `CoinData` cannot be created directly, but is the format by which the DataAPI
/// retrieves price data.
///
/// `CoinData` can be serialized via Serde.
#[derive(Serialize, Debug, PartialEq, Clone)]
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

    /// Get buy price from CoinData
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seventh_core::api::{CoinData, DataAPI};
    ///
    /// let mut data = DataAPI::new();
    /// data.update("BTC");
    ///
    /// let coin: CoinData = data.last();
    /// println!("{}", coin.buyprice());
    /// ```
    pub fn buyprice(&self) -> f32 {
        self.buy
    }

    /// Get sell price from CoinData
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seventh_core::api::{CoinData, DataAPI};
    ///
    /// let mut data = DataAPI::new();
    /// data.update("BTC");
    ///
    /// let coin: CoinData = data.last();
    /// println!("{}", coin.sellprice());
    /// ```
    pub fn sellprice(&self) -> f32 {
        self.sell
    }

    /// Get historical prices from CoinData
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seventh_core::api::{CoinData, DataAPI};
    ///
    /// let mut data = DataAPI::new();
    /// data.update("BTC");
    ///
    /// let coin: CoinData = data.last();
    /// println!("{:?}", coin.historical());
    /// ```
    pub fn historical(&self) -> Vec<f32> {
        self.historical.clone()
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

/// Active API for retrieving cryptocurrency price data.
///
/// Maintains both an active reqwest client as well as a vector of `CoinData` structs.
/// The API is coin-agnostic, but currently supports only BTC, ETC, ETH, and LTC, due to
/// limitations of the CoinGecko API.
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

    /// Create a new `DataAPI`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seventh_core::api::DataAPI;
    ///
    /// let mut data = DataAPI::new();
    /// assert_eq!(data.coins().len(), 0);
    /// ```
    pub fn new() -> Self {
        DataAPI { client: reqwest::Client::new(), coins: vec![] }
    }

    /// Update the DataAPI with the data for `coin` at the current time.
    ///
    /// If the connection to either the Coinbase or CoinGecko fails, the method panics.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seventh_core::api::DataAPI;
    ///
    /// let mut data = DataAPI::new();
    /// assert_eq!(data.coins().len(), 0);
    /// data.update("BTC"); // Gets current data for BTC-USD
    /// assert_eq!(data.coins().len(), 1);
    /// ```
    pub fn update(&mut self, coin: &str) {
        let buy = self.get_coinbase(coin, "buy");
        let sell = self.get_coinbase(coin, "sell");
        let spot = self.get_coinbase(coin, "spot");
        let history = self.get_historical(COIN_ID[coin]);
        self.coins.push(CoinData::new(buy, sell, spot, history))
    }

    /// Get list of all `CoinData` items stored in `DataAPI`
    ///
    /// This method is not reccomended to be used directly.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seventh_core::api::DataAPI;
    ///
    /// let mut data = DataAPI::new();
    /// data.update("BTC")
    /// data.update("ETC")
    /// println!("{:?}", data.coins());
    /// ```
    pub fn coins(&self) -> Vec<CoinData> {
        self.coins.clone()
    }

    /// Get last `CoinData` itemsstored in `DataAPI`
    ///
    /// This method returns the CoinData generated by the most recent call to `update`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seventh_core::api::DataAPI;
    ///
    /// let mut data = DataAPI::new();
    /// data.update("BTC")
    /// data.update("ETC")
    /// println!("{}", data.last());
    /// ```
    pub fn last(&self) -> CoinData {
        self.coins.last().unwrap().clone()
    }
}
