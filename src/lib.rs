/*!
The `seventh-core` crate provides a low level cryptocurrency data API and platforms for backtesting, live testing, and live trading.

# Overview

`seventh-core` uses the CoinGecko and CoinBase APIs for retrieving data. It makes a live
trading connection using CoinBase, and can backtest over provided data or live data.

The primary types in this crate are `DataAPI`, which maintains a connection to retrieve
price data from, and `Backtrader`, which provides an interface for writing testing
servers.

Other data types include `CoinData` and `TraderData`, which are serializable formats for
passing trading data in between other applications. Finally, the `Action` enum represents
all the possible actions in a trade.

# Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
seventh_core = "0.1"
```

# Example

```no_run
use seventh_core::api::DataAPI;

fn main() {
    let mut data_api = DataAPI::new();
    data_api.update("BTC");
    println!("{:?}", data_api.last())
}
```

*/
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate rocket;

pub mod api;
pub mod backtrader;
pub mod historical;
