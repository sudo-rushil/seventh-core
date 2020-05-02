//! Place orders using the CoinBase API

#[derive(Deserialize, Debug)]
struct Amount {
    amount: String,
    currency: String,
}

#[derive(Deserialize, Debug)]
struct TransactionMethod {
    id: String,
    resource: String,
    resource_path: String,
}

#[derive(Deserialize, Debug)]
struct Order {
    id: String,
    status: String,
    payment_method: TransactionMethod,
    transaction: TransactionMethod,
    amount: Amount,
    total: Amount,
    subtotal: Amount,
    created_at: String,
    updated_at: String,
    resource: String,
    resource_path: String,
    committed: bool,
    instant: bool,
    fee: Amount,
    payout_at: String,
}

#[derive(Deserialize, Debug)]
struct OrderData {
    pub data: Order,
}

#[derive(Debug)]
struct Transaction {
    amount: f32,
    currency: String,
    payment_method: String,
    isbuy: bool,
}

impl Transaction {
    fn new(amount: f32, currency: &str, payment_method: &str, isbuy: bool) -> Self {
        Transaction {
            amount: amount,
            currency: currency.to_string(),
            payment_method: payment_method.to_string(),
            isbuy,
        }
    }

    fn json(&self) -> String {
        serde_json::json!({
            "amount": self.amount,
            "currency": self.currency,
            "payment_method": self.payment_method
        })
        .to_string()
    }
}

/// Active API for placing trades using the Coinbase trading API.
///
/// Requires a valid authentication token, account token, and payment method token. See
/// the Coinbase API documentation for details.
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

    pub fn new(auth: &str, account: &str, payment_method: &str) -> Self {
        BrokerAPI {
            client: reqwest::Client::new(),
            history: vec![],
            auth: auth.to_owned(),
            account: account.to_owned(),
            payment: payment_method.to_owned()
        }
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
}
