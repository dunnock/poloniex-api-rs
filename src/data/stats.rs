use super::book::{Book, BookAccounting};
use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;
 
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
  pub rate: f32, 
  pub amount: f32
}

#[derive(Clone, Debug, PartialEq)]
pub struct BookStats {
  pub min_sell: f32,
  pub max_buy: f32,
  pub sum_sell: f32,
  pub sum_buy: f32,
  pub vec_buy: Vec<Record>,
  pub vec_sell: Vec<Record>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BookWithStats {
  book: Book,
  pub stats: BookStats
}

// BookStats operations
impl BookStats {
  pub fn new(book: &Book) -> BookStats {
    let hash_to_vec = |hash: &HashMap<String, f32>| -> (f32, Vec<Record>) {
      let mut sum = 0.0;
      let vec = hash.iter().filter_map(|(rate_s, amount)| {
        rate_s.parse::<f32>()
          .and_then(|rate| { 
            sum += *amount; 
            Ok(Record { rate, amount: *amount }) 
          }).ok()
      }).collect();
      (sum, vec)
    };

    let (sum_buy, mut vec_buy) = hash_to_vec(&book.buy);
    vec_buy.sort_unstable_by(|rec1, rec2| rec2.rate.partial_cmp(&rec1.rate).unwrap_or(Ordering::Equal));
    let (sum_sell, mut vec_sell) = hash_to_vec(&book.sell);
    vec_sell.sort_unstable_by(|rec1, rec2| rec1.rate.partial_cmp(&rec2.rate).unwrap_or(Ordering::Equal));

    BookStats {
      min_sell: vec_sell.first().map_or(0.0, |rec| rec.rate),
      max_buy: vec_buy.first().map_or(0.0, |rec| rec.rate),
      sum_sell,
      sum_buy,
      vec_buy,
      vec_sell,
    }
  }

  pub fn update_sell(&mut self, rate: f32, amount: f32, prev_amount: Option<f32>) {
    if amount > 0.0 && (self.min_sell > rate || self.min_sell == 0.0) {
      self.min_sell = rate
    };
    self.sum_sell = self.sum_sell + amount - prev_amount.unwrap_or(0.0);
  }

  pub fn update_buy(&mut self, rate: f32, amount: f32, prev_amount: Option<f32>) {
    if amount > 0.0 && self.max_buy < rate {
      self.max_buy = rate
    };
    self.sum_buy = self.sum_buy + amount - prev_amount.unwrap_or(0.0);
  }
}

impl fmt::Display for BookStats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(BUY min {} sum {} | SELL max {} sum {})", self.max_buy, self.sum_buy, self.min_sell, self.sum_sell)
  }
}

// BookWithStats operations
impl BookWithStats {
  pub fn new(book: Book) -> BookWithStats {
    BookWithStats {
      stats: BookStats::new(&book),
      book
    }
  }
}

impl fmt::Display for BookWithStats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.stats.fmt(f)
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


/** 
 ** TESTS TESTS TESTS
 **/

#[cfg(test)]
mod tests {
  use super::BookWithStats;
  use ::data::book::Book;
  use json;
  use std::convert::TryFrom;

  #[test]
  fn stats_init() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let book_stats = BookWithStats::new(book).stats;
    assert_eq!(book_stats.min_sell, 0.13161901);
    assert_eq!(book_stats.max_buy, 0.13109621);
  }

  #[test]
  fn stats_init_wrong_order() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13361901": 0.23709568, "0.13164313": "0.17328089"}, {"0.12909621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let book_stats = BookWithStats::new(book).stats;
    assert_eq!(book_stats.min_sell, 0.13164313);
    assert_eq!(book_stats.max_buy, 0.13069621);
  }

  #[test]
  fn stats_update_sell_zero() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_sell(0.1, 0.0, None);
    assert_eq!(book_stats.min_sell, 0.13161901);
  }


  #[test]
  fn stats_update_sell() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_sell(0.1, 1.0, None);
    assert_eq!(book_stats.min_sell, 0.1);
  }


  #[test]
  fn stats_update_buy_zero() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_buy(100.0, 0.0, None);
    assert_eq!(book_stats.max_buy, 0.13109621);
  }


  #[test]
  fn stats_update_buy() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_buy(100.0, 1.0, None);
    assert_eq!(book_stats.max_buy, 100.0);
  }

}