//! Retrieve data from Coinbase and CoinGecko APIs

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
struct Data {
    data: Price
}

struct DataAPI {
    client: reqwest::Client,
    historical: Vec<f32>,
    buy: f32,
    sell: f32,
    spot: f32,
}

#[tokio::main]
pub async fn thing() -> Result<(), reqwest::Error> {
    let request_url = format!(
        "https://api.coinbase.com/v2/prices/BTC-USD/sell"
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Data = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
