use std::collections::HashMap;
use std::fmt::Debug;
use json::JsonValue;
use std::convert::TryFrom;
use ::error::PoloError;
use super::json::Expect;

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


/**
 * Book conversion traits
 * use: 
 *  let val = match json::parse(r#"["i", {"currencyPair": "BTC_BCH", "orderBook": [{0.13161901: "0.23709568", 0.13164313: "0.17328089"}, {0.13169621: "0.2331"}]]"#) {
 *     json::Array(vec) => vec,
 *     _ => Err(())
 *  };
 *  let records:: Book = Book::try_from(&val)
 **/

impl<'a> TryFrom<&'a JsonValue> for Book {
  type Error = PoloError;
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    let err = |msg| Err(PoloError::wrong_data(format!("{} {:?}", msg, v)));

    if !v.is_object() {
      return err("initial book is not object");
    }
    if v["orderBook"].len()!=2 {
      return err("initial book orderBook array should contain 2 objects");
    }

    Ok(Self {
      pair: TradePairs::try_from(&v["currencyPair"])?, 
      sell: v["orderBook"][0].expect("initial book orderBook[0]")?, 
      buy: v["orderBook"][1].expect("initial book orderBook[1]")?
    })
  }
}
