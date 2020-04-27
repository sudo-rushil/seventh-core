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
