pub fn add() -> u32 {
    2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use]
extern crate serde;
extern crate reqwest;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Thing {
    base: String,
    currency: String,
    amount: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    data: Thing
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request_url = format!(
        "https://api.coinbase.com/v2/prices/BTC-USD/sell"
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Data = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
