#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use seventh_core::backtrader::{Actions, Backtrader};
use std::sync::Mutex;

struct LockedTrader {
    trader: Mutex<Backtrader>,
}

#[get("/")]
fn index(trader: State<LockedTrader>) -> String {
    let mut lock = trader.trader.lock().expect("Lock state");
    lock.trade(Actions::Hold);
    serde_json::to_string(&lock.data()).unwrap()
}

fn main() {
    rocket::ignite()
        .manage(LockedTrader {
            trader: Mutex::new(Backtrader::new(1000.0, "BTC")),
        })
        .mount("/", routes![index])
        .launch();
}
