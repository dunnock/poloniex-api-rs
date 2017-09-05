use super::book::{Book, BookAccounting};
use std::collections::HashMap;
use std::fmt;

 
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
  pub rate: f32, 
  pub amount: f32
}

#[derive(Clone, Debug, PartialEq)]
pub struct BookStats {
  pub max_sell: f32,
  pub min_buy: f32,
  pub sum_sell: f32,
  pub sum_buy: f32,
  pub vec_buy: Vec<Record>,
  pub vec_sell: Vec<Record>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BookWithStats {
  book: Book,
  stats: BookStats
}

// BookStats operations
impl BookStats {
  pub fn new() -> BookStats {
    BookStats {
      max_sell: 0.0,
      min_buy: 0.0,
      sum_sell: 0.0,
      sum_buy: 0.0,
      vec_buy: Vec::new(),
      vec_sell: Vec::new(),
    }
  }

  pub fn init(&mut self, book: &Book) {
    let hash_to_vec = |hash: &HashMap<String, f32>| -> Vec<Record> {
      hash.iter().filter_map(|(rate_s, amount)| {
        rate_s.parse::<f32>()
          .and_then(|rate| Ok(Record { rate, amount: *amount })).ok()
      }).collect()
    };
    self.vec_buy = hash_to_vec(&book.buy);
    self.vec_sell = hash_to_vec(&book.sell);
  }

  pub fn update_sell(&mut self, rate: f32, amount: f32, prev_amount: Option<f32>) {
    if amount > 0.0 && self.max_sell < rate {
      self.max_sell = rate
    };
    self.sum_sell = self.sum_sell + amount - prev_amount.unwrap_or(0.0);
  }

  pub fn update_buy(&mut self, rate: f32, amount: f32, prev_amount: Option<f32>) {
    if amount > 0.0 && self.min_buy > rate {
      self.min_buy = rate
    };
    self.sum_buy = self.sum_buy + amount - prev_amount.unwrap_or(0.0);
  }
}

impl fmt::Display for BookStats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(BUY min {} sum {} | SELL max {} sum {})", self.min_buy, self.sum_buy, self.max_sell, self.sum_sell)
  }
}

// BookWithStats operations
impl BookWithStats {
  pub fn new(book: Book) -> BookWithStats {
    BookWithStats {
      book,
      stats: BookStats::new()
    }
  }
}

impl BookAccounting for BookWithStats {
  fn update_sell(&mut self, rate: String, amount: f32) -> Option<f32> {
    let rate_f32 = rate.parse::<f32>();
    let prev_amount = self.book.update_sell(rate, amount);
    if let Ok(rate_f32) = rate_f32 {
      self.stats.update_sell(rate_f32, amount, prev_amount);
    };
    prev_amount
  }

  fn update_buy(&mut self, rate: String, amount: f32) -> Option<f32> {
    let rate_f32 = rate.parse::<f32>();
    let prev_amount = self.book.update_buy(rate, amount);
    if let Ok(rate_f32) = rate_f32 {
      self.stats.update_buy(rate_f32, amount, prev_amount);
    };
    prev_amount
  }

  fn book_ref(&self) -> &Book {
    &self.book
  }
}
