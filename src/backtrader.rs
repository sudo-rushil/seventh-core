use crate::api::data::DataAPI;

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

#[derive(Serialize)]
pub struct TraderData {
    historical: Vec<f32>,
    buy: f32,
    sell: f32,
    account: f32,
    holding: f32,
}

impl Backtrader {
    pub fn new(account: f32, coin: &str) -> Self {
        let mut trader = Backtrader {
            api: DataAPI::new(),
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
    }

    fn sell(&mut self, mut coin: f32) {
        if coin > self.holdings {
            coin = self.holdings;
        }
        let price = self.api.last().sellprice();

        self.account += coin * price;
        self.holdings -= coin;
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
