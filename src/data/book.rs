use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub enum TradePairs {
  BtcEth,
  BtcBch,
}

// sorted by rate, groupd by price
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
  pub rate: f64, 
  pub amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Book {
  pub pairs: TradePairs,
  pub sell: Vec<Record>,
  pub buy: Vec<Record>
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
      sell: Vec::new(),
      buy: Vec::new()
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