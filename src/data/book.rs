use super::json::Expect;
use std::collections::HashMap;
use std::fmt::Debug;
use json::JsonValue;
use std::convert::TryFrom;
use crate::error::PoloError;
use super::timeseries::{Timeseries, WithTime};
use time::{Timespec, get_time};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TradePairs {
  BtcEth,
  BtcBch,
  BtcLtc,
  BtcZec,
  UsdtBtc,
  UsdtEth,
  UsdtLtc,
  UsdtBch,
  UsdtZec,
  UsdtXrp,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Deal {
  #[serde(with = "serialize_timespec")]
  pub time: Timespec,
  pub id: u64,
  pub rate: f64,
  pub amount: f64 // amount < 0 means sell order reconciled, otherwise buy
}

impl WithTime for Deal {
  fn get_time(&self) -> Timespec {
    self.time
  }
}

mod serialize_timespec {
    use time::Timespec;
    use serde::{self, Serializer};

    pub fn serialize<S>(time: &Timespec, serializer: S) -> Result<S::Ok, S::Error>
      where S: Serializer,
    {
        let s = format!("{}.{}", time.sec, time.nsec);
        serializer.serialize_str(&s)
    }
}


type Records = HashMap<String,f64>;

#[derive(Clone, Debug, PartialEq)]
pub struct Book {
  pub pair: TradePairs,
  pub sell: Records,
  pub buy: Records,
  pub deals: Timeseries<Deal>
}

pub trait BookAccounting: Debug {
  // should return previous amount by the same rate OR None
  fn update_sell_orders(&mut self, rate: String, amount: f64) -> Option<f64>;

  // should return previous amount by the same rate OR None
  fn update_buy_orders(&mut self, rate: String, amount: f64) -> Option<f64>;

  // should return rate parse result to f64, or error wrapped in PoloError
  fn new_deal(&mut self, id: u64, rate: String, amount: f64) -> Result<f64, PoloError>;

  // reference to the actual Book struct (for wrappers)
  fn book_ref(&self) -> &Book;
}




// Book operations

impl Book {
  pub fn new(pair: TradePairs) -> Book {
    Book {
      pair,
      sell: HashMap::with_capacity(1000),
      buy: HashMap::with_capacity(1000),
      deals: Timeseries::new(),
    }
  }
}

impl BookAccounting for Book {
  fn update_sell_orders(&mut self, rate: String, amount: f64) -> Option<f64> {
    if amount == 0.0 {
      self.sell.remove(&rate)
    } else {
      self.sell.insert(rate, amount)
    }
  }
  fn update_buy_orders(&mut self, rate: String, amount: f64) -> Option<f64> {
    if amount == 0.0 {
      self.buy.remove(&rate)
    } else {
      self.buy.insert(rate, amount)
    }
  }
  fn new_deal(&mut self, id: u64, rate: String, amount: f64) -> Result<f64, PoloError> {
    let rate = rate.parse().map_err(PoloError::from)?;
    let time = get_time();
    self.deals.add(Deal { time, id, rate, amount });
    Ok(rate)
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
      buy: v["orderBook"][1].expect("initial book orderBook[1]")?,
      deals:  Timeseries::new(),
    })
  }
}
