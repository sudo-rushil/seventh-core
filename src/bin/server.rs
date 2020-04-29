#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{request::Form, State};
use std::sync::Mutex;

use seventh_core::backtrader::{Actions, Backtrader};

struct LockedTrader {
    trader: Mutex<Backtrader>,
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

fn main() {
    rocket::ignite()
        .manage(LockedTrader {
            trader: Mutex::new(Backtrader::new(1000.0, "BTC")),
        })
        .mount("/", routes![index, data, trade])
        .launch();
}
