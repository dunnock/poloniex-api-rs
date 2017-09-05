use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TradePairs {
  BtcEth,
  BtcBch,
}

type Records = HashMap<String,f32>;

#[derive(Clone, Debug, PartialEq)]
pub struct Book {
  pub pair: TradePairs,
  pub sell: Records,
  pub buy: Records
}

pub trait BookAccounting: Debug {
  fn update_sell(&mut self, rate: String, amount: f32) -> Option<f32>;
  fn update_buy(&mut self, rate: String, amount: f32) -> Option<f32>;
  fn book_ref(&self) -> &Book;
}

// Book operations

impl Book {
  pub fn new(pair: TradePairs) -> Book {
    Book {
      pair,
      sell: HashMap::new(),
      buy: HashMap::new()
    }
  }
}

impl BookAccounting for Book {
  fn update_sell(&mut self, rate: String, amount: f32) -> Option<f32> {
    self.sell.insert(rate, amount)
  }
  fn update_buy(&mut self, rate: String, amount: f32) -> Option<f32> {
    self.buy.insert(rate, amount)
  }
  fn book_ref(&self) -> &Book {
    &self
  }
}
