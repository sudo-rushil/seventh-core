//! Livetrading interface

use crate::api::data::DataAPI;
use crate::api::transactions::BrokerAPI;
use crate::backtrader::{Actions, TraderData};

pub struct Livetrader {
    api: DataAPI,
    live: BrokerAPI,
    coin: String,
    history: Vec<(f32, Actions)>,
    account: f32,  // in usd
    holdings: f32, // in coin
}

impl Livetrader {
    pub fn new(
        account: f32,
        coin: &str,
        auth: &str,
        account_name: &str,
        payment_method: &str,
    ) -> Self {
        let mut trader = Livetrader {
            api: DataAPI::new(),
            live: BrokerAPI::new(auth, account_name, payment_method),
            coin: coin.to_owned(),
            history: vec![],
            account,
            holdings: 0.0,
        };
        trader.api.update(coin);
        trader
    }

    pub fn reset(&mut self, account: f32, coin: &str) {
        self.history = vec![];
        self.account = account;
        self.coin = coin.to_owned();
        self.holdings = 0.0;
    }

    pub fn trade(&mut self, action: Actions) {
        println!("Making trade");
        self.api.update(&self.coin);
        self.history.push((self.account, action.clone()));

        match action {
            Actions::Buy(usd) => self.buy(usd),
            Actions::Sell(coin) => self.sell(coin),
            Actions::Hold => return,
        };
    }

    fn buy(&mut self, mut usd: f32) {
        if usd > self.account {
            usd = self.account;
        }
        let price = self.api.last().buyprice();

        self.account -= usd;
        self.holdings += usd / price;
        self.live.buy(usd, &self.coin);
    }

    fn sell(&mut self, coin: f32) {
        let price = self.api.last().sellprice();

        self.account += coin * price;
        self.holdings -= coin;
        self.live.sell(coin, &self.coin);
    }

    pub fn data(&self) -> TraderData {
        let coins = self.api.last();

        TraderData {
            historical: coins.historical(),
            buy: coins.buyprice(),
            sell: coins.sellprice(),
            account: self.account,
            holding: self.holdings,
        }
    }

    pub fn history(&self) -> Vec<(f32, Actions)> {
        self.history.clone()
    }
}
