use std::ops::{Index, IndexMut};
use ::error::PoloError;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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
  pub amount: f64,
  _rate_f: f64, 
}

impl Record {
  pub fn new(rate: String, amount: f64) -> Record {
    Record { rate, amount, _rate_f: 0.0 }
  }

  pub fn rate_f64(&mut self) -> Result<f64, PoloError> {
    if self._rate_f == 0.0 {
      self._rate_f = self.rate.parse::<f64>()?;
    };
    Ok(self._rate_f)
  }
}

type Records = HashMap<String,f64>;

#[derive(Clone, Debug, PartialEq)]
pub struct Book {
  pub pairs: TradePairs,
  pub sell: Records,
  pub buy: Records
}

#[derive(Clone, Debug, PartialEq)]
pub struct TradeBook {
  pub btcbch: Book,
  pub btceth: Book
}

// Book constructor
impl Book {
  pub fn new(pairs: TradePairs) -> Book {
    Book {
      pairs,
      sell: HashMap::new(),
      buy: HashMap::new()
    }
  }
}

// TradeBook constructor
impl TradeBook {
  pub fn new() -> TradeBook {
    TradeBook {
      btcbch: Book::new(TradePairs::BtcBch),
      btceth: Book::new(TradePairs::BtcEth),
    }
  }
}

// TradeBook items accessible by TradePairs enum:
// let tb = TradeBook::new();
// tb[TradePairs::BtcBch]
impl<'a> Index<&'a TradePairs> for TradeBook {
  type Output = Book;

  fn index(&self, pairs: &'a TradePairs) -> &Book {
    match *pairs {
      TradePairs::BtcBch => &self.btcbch,
      TradePairs::BtcEth => &self.btceth,
    }
  }
}

impl<'a> IndexMut<&'a TradePairs> for TradeBook {
  fn index_mut(&mut self, pairs: &'a TradePairs) -> &mut Book {
    match *pairs {
      TradePairs::BtcBch => &mut self.btcbch,
      TradePairs::BtcEth => &mut self.btceth,
    }
  }
}