use seventh_core::api::data::*;

#[test]
fn test_initialization() {
    let data_api = DataAPI::new();
    assert_eq!(data_api.coins(), vec![]);
}

#[test]
fn test_update() {
    let mut data_api = DataAPI::new();
    assert_eq!(data_api.coins(), vec![]);
    data_api.update("BTC");
    assert_eq!(data_api.coins().len(), 1);
}
