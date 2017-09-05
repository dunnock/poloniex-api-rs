use ::error::PoloError;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TradePairs {
  BtcEth,
  BtcBch,
}

pub enum TradeOp {
  Sell,
  Buy,
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

#[derive(Debug, PartialEq)]
pub struct TradeBook {
  pub books: Vec<Book>,
  pub by_id: HashMap<u16, usize>,
  pub by_pair: HashMap<TradePairs, usize>,
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

  pub fn update_sell(&mut self, rate: String, amount: f32) -> Option<f32> {
    self.sell.insert(rate, amount)
  }

  pub fn update_buy(&mut self, rate: String, amount: f32) -> Option<f32> {
    self.buy.insert(rate, amount)
  }
}

// TradeBook operations
impl TradeBook {
  pub fn new() -> TradeBook {
    TradeBook {
      books: Vec::new(),
      by_id: HashMap::new(),
      by_pair: HashMap::new()
    }
  }

  pub fn add_book(&mut self, book: Book, id: u16) {
    let pair = book.pair.clone();
    self.books.push(book);
    let idx = self.books.len()-1;
    self.by_id.insert(id, idx);
    self.by_pair.insert(pair, idx);
  }

  pub fn get_book_by_id(&mut self, id: &u16) -> Option<&mut Book> {
    if let Some(idx) = self.by_id.get(id) {
      Some(&mut self.books[*idx])
    } else {
      None
    }
  }
}
