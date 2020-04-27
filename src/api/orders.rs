//! Order datatypes for placing buys and sells using the CoinBase API

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
pub struct Order {
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

#[derive(Debug)]
pub struct Transaction {
    amount: f32,
    currency: String,
    payment_method: String,
    isbuy: bool,
}

impl Transaction {
    pub fn new(amount: f32, currency: &str, payment_method: &str, isbuy: bool) -> Self {
        Transaction {
            amount: amount,
            currency: currency.to_string(),
            payment_method: payment_method.to_string(),
            isbuy,
        }
    }

    pub fn json(&self) -> String {
        serde_json::json!({
            "amount": self.amount,
            "currency": self.currency,
            "payment_method": self.payment_method
        })
        .to_string()
    }
}
