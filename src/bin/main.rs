use seventh_core::api::data::DataAPI;

fn main() {
    let mut api = DataAPI::new();

    loop {
        api.update("BTC");
        println!("{}", api.coins().last().unwrap())
    }
}
