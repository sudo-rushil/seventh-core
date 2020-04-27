//! Place orders using the CoinBase API

use crate::api::orders::{Order, Transaction};

pub struct BrokerAPI {
    client: reqwest::Client,
    history: Vec<Transaction>,
    auth: String,
    account: String,
    payment: String,
}

impl BrokerAPI {
    #[tokio::main]
    async fn post<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: String, data: Transaction) -> Result<T, reqwest::Error> {
        let response = self.client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(self.auth.clone())
            .body(data.json())
            .send()
            .await?;

        let result: T = response.json().await?;

        Ok(result)
    }

    pub fn new(auth: &str, account: &str, payment_method: &str) -> Self {
        BrokerAPI {
            client: reqwest::Client::new(),
            history: vec![],
            auth: auth.to_owned(),
            account: account.to_owned(),
            payment: payment_method.to_owned()
        }
    }
}
