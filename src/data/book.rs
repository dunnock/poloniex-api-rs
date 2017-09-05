use ::error::PoloError;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TradePairs {
  BtcEth,
  BtcBch,
}

/**
 * Record struct
 **/
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
  pub rate: String, 
  pub amount: f32,
  _rate_f: f32, 
}

impl Record {
  pub fn new(rate: String, amount: f32) -> Record {
    Record { rate, amount, _rate_f: 0.0 }
  }

  pub fn rate_f32(&mut self) -> Result<f32, PoloError> {
    if self._rate_f == 0.0 {
      self._rate_f = self.rate.parse::<f32>()?;
    };
    Ok(self._rate_f)
  }
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
