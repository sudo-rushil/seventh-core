use seventh_core::api::data::*;

#[test]
fn get_response() {
    DataAPI::new().get::<PriceData>("https://api.coinbase.com/v2/prices/BTC-USD/spot");
    assert!(true);
}
