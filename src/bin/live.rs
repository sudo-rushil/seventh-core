#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// extern crate config;

use rocket::{request::Form, State};
use std::sync::Mutex;

use seventh_core::backtrader::Actions;
use seventh_core::livetrader::Livetrader;

struct LockedTrader {
    trader: Mutex<Livetrader>,
}

#[derive(FromForm)]
struct Transaction {
    amount: f32,
}

#[get("/")]
fn index(trader: State<LockedTrader>) -> String {
    let mut lock = trader.trader.lock().expect("Lock state");
    lock.reset(1000.0, "BTC");
    "LIVE".to_owned()
}

#[get("/data")]
fn data(trader: State<LockedTrader>) -> String {
    let lock = trader.trader.lock().expect("Lock state");
    serde_json::to_string(&lock.data()).unwrap()
}

#[post("/trade/<action>", data = "<trans>")]
fn trade(action: String, trans: Form<Transaction>, trader: State<LockedTrader>) -> String {
    let mut lock = trader.trader.lock().expect("Lock state");
    let amount = trans.amount;
    println!("{}, {}", amount, action);

    match action.as_str() {
        "buy" => lock.trade(Actions::Buy(amount)),
        "sell" => lock.trade(Actions::Sell(amount)),
        _ => lock.trade(Actions::Hold),
    };

    serde_json::to_string(&lock.data()).unwrap()
}

fn read_settings() -> Result<(String, String, String), config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Keys")).unwrap();
    let auth = settings.get_str("auth")?;
    let account = settings.get_str("account")?;
    let payment = settings.get_str("payment")?;

    Ok((auth, account, payment))
}

fn main() {
    let (auth, account, payment) = match read_settings() {
        Ok(credentials) => credentials,
        Err(_) => panic!("Credentials unparsable"),
    };
    rocket::ignite()
        .manage(LockedTrader {
            trader: Mutex::new(Livetrader::new(1000.0, "BTC", &auth, &account, &payment)),
        })
        .mount("/", routes![index, data, trade])
        .launch();
}
