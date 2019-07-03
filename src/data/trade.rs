use super::book::{TradePairs, Book, BookAccounting};
use super::stats::BookWithStats;
use std::collections::HashMap;

pub enum TradeOp {
  Sell,
  Buy,
}

#[derive(Default, Debug, PartialEq)]
pub struct TradeBook {
  pub books: Vec<BookWithStats>,
  pub by_id: HashMap<u16, usize>,
  pub by_pair: HashMap<TradePairs, usize>,
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
    let idx: usize;
    if let Some(i) = self.by_pair.get(&pair) {
      idx = *i;
      self.books[idx] = BookWithStats::new(book);
    } else {
      self.books.push(BookWithStats::new(book));
      idx = self.books.len()-1;
    }
    self.by_id.insert(id, idx);
    self.by_pair.insert(pair, idx);
  }

  pub fn book_by_id(&mut self, id: u16) -> Option<&mut BookAccounting> {
    if let Some(idx) = self.by_id.get(&id) {
      Some(&mut self.books[*idx])
    } else {
      None
    }
  }
}
