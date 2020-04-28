use crate::api::data::*;

pub struct Backtrader {
    api: DataAPI,
    coin: String,
    history: Vec<(f32, Actions)>,
    account: f32,  // in usd
    holdings: f32, // in coin
}

#[derive(Clone)]
pub enum Actions {
    Buy(f32),  // amount in usd
    Sell(f32), // amount in btc
    Hold,
}

impl Backtrader {
    pub fn new(account: f32, coin: &str) -> Self {
        Backtrader {
            api: DataAPI::new(),
            coin: coin.to_owned(),
            history: vec![],
            account,
            holdings: 0.0,
        }
    }

    pub fn trade(&mut self, action: Actions) {
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

        self.account -= price;
        self.holdings += usd / price;
    }

    fn sell(&mut self, mut coin: f32) {
        if coin > self.holdings {
            coin = self.holdings;
        }
        let price = self.api.last().sellprice();

        self.account += coin * price;
        self.holdings -= coin;
    }

    pub fn data(&self) -> CoinData {
        self.api.last()
    }

    pub fn history(&self) -> Vec<(f32, Actions)> {
        self.history.clone()
    }
}
