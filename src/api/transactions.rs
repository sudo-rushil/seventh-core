//! Place orders using the CoinBase API

use crate::api::orders::{OrderData, Transaction};

pub struct BrokerAPI {
    client: reqwest::Client,
    history: Vec<Transaction>,
    auth: String,
    account: String,
    payment: String,
}

impl BrokerAPI {
    #[tokio::main]
    async fn post<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: String, data: &Transaction) -> Result<T, reqwest::Error> {
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

    pub fn sell(&mut self, amount: f32, currency: &str) {
        let trans = Transaction::new(amount, currency, &self.payment, false);
        let endpoint = format!("https://api.coinbase.com/v2/accounts/{}/sells", self.account);

        match self.post::<OrderData>(endpoint, &trans) {
            Ok(order) => println!("{:?}", order.data),
            Err(e) => panic!("Error placing sell: {}", e)
        };

        self.history.push(trans);
    }

    pub fn buy(&mut self, amount: f32, currency: &str) {
        let trans = Transaction::new(amount, currency, &self.payment, true);
        let endpoint = format!("https://api.coinbase.com/v2/accounts/{}/buys", self.account);

        match self.post::<OrderData>(endpoint, &trans) {
            Ok(order) => println!("{:?}", order.data),
            Err(e) => panic!("Error placing buy: {}", e)
        };

        self.history.push(trans);
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
