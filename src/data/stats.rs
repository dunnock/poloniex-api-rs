use super::book::{Book, BookAccounting};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::cmp::Ordering;
use crate::error::PoloError;
use super::tradestats::{TradeStats, TimeStats};
use super::book::Deal;
use time;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
  pub rate: f64, 
  pub amount: f64
}

#[derive(Clone, Debug, PartialEq)]
pub struct BookStats {
  pub min_sell: f64,
  pub max_buy: f64,
  pub sum_sell: f64,
  pub sum_buy: f64,
  pub vec_buy: Vec<Record>,
  pub vec_sell: Vec<Record>,
  pub skin_buy: f64,
  pub skin_sell: f64,
  pub surface_buy: f64,
  pub surface_sell: f64,
}


#[derive(Clone, Debug, PartialEq)]
pub struct BookWithStats {
  book: Book,
  pub stats: BookStats,
  pub trade_series_1s: VecDeque<TradeStats>,
  pub trade_stats_1m: TradeStats,
}

// BookStats operations

fn rate_by_amount(vec: &Vec<Record>, amount: f64) -> f64 {
  let mut total = 0.0;
  for rec in vec.iter() {
    total = total + rec.amount;
    if total>amount {
      return rec.rate
    }
  }
  return 0.0
}

impl BookStats {
  pub fn new(book: &Book) -> BookStats {
    let mut vec_buy = hash_to_vec(&book.buy);
    vec_buy.sort_unstable_by(|rec1, rec2| f64cmp(&rec2.rate, &rec1.rate));
    let max_buy = vec_buy.first().map_or(0.0, |rec| rec.rate);
    if let Some(filter_trash) = vec_buy.iter().position(|rec| rec.rate < max_buy/10.0) {
      vec_buy.truncate(filter_trash);
    }
    let sum_buy = vec_buy.iter().fold(0.0, |acc, rec| acc + rec.amount);
    let skin_buy = rate_by_amount(&vec_buy, sum_buy*0.1);
    let surface_buy = rate_by_amount(&vec_buy, sum_buy*0.01);

    let mut vec_sell = hash_to_vec(&book.sell);
    vec_sell.sort_unstable_by(|rec1, rec2| f64cmp(&rec1.rate, &rec2.rate));
    let min_sell = vec_sell.first().map_or(0.0, |rec| rec.rate);
    if let Some(filter_trash) = vec_sell.iter().position(|rec| rec.rate > min_sell*10.0) {
      vec_sell.truncate(filter_trash);
    }
    let sum_sell = vec_sell.iter().fold(0.0, |acc, rec| acc + rec.amount);
    let skin_sell = rate_by_amount(&vec_sell, sum_sell*0.1);
    let surface_sell = rate_by_amount(&vec_sell, sum_sell*0.01);

    BookStats { max_buy, min_sell, skin_buy, skin_sell, sum_sell, sum_buy, vec_buy, vec_sell, surface_buy, surface_sell }
  }

  pub fn update_sell_orders(&mut self, rate: f64, amount: f64, prev_amount: Option<f64>) {
    let idx_r = self.vec_sell.binary_search_by(|rec| f64cmp(&rec.rate, &rate));
    let stat_cmp = self.min_sell > rate;
    update_sorted_vec(idx_r, &mut self.vec_sell, &mut self.min_sell, rate, amount, stat_cmp);
    self.sum_sell = self.sum_sell + amount - prev_amount.unwrap_or(0.0);
    if rate < self.skin_sell {
      self.skin_sell = rate_by_amount(&self.vec_sell, self.sum_sell*0.1)
    }
    if rate < self.surface_sell {
      self.surface_sell = rate_by_amount(&self.vec_sell, self.sum_sell*0.01)
    }
  }

  pub fn update_buy_orders(&mut self, rate: f64, amount: f64, prev_amount: Option<f64>) {
    let idx_r = self.vec_buy.binary_search_by(|rec| f64cmp(&rate, &rec.rate));
    let stat_cmp = self.max_buy < rate;
    update_sorted_vec(idx_r, &mut self.vec_buy, &mut self.max_buy, rate, amount, stat_cmp);
    self.sum_buy = self.sum_buy + amount - prev_amount.unwrap_or(0.0);
    if rate > self.skin_buy {
      self.skin_buy = rate_by_amount(&self.vec_buy, self.sum_buy*0.1)
    }
    if rate > self.surface_buy {
      self.surface_buy = rate_by_amount(&self.vec_buy, self.sum_buy*0.01)
    }
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
      trade_series_1s: VecDeque::new(),
      trade_stats_1m: TradeStats::default(),
      book
    }
  }
}

impl fmt::Display for BookWithStats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "  ** STATS ")?;
    self.stats.fmt(f)?;
    if let Some(last_sec) = self.trade_series_1s.front() {
      write!(f, "\n  >> T1s: ")?;
      last_sec.fmt(f)?;
    }
    write!(f, "\n  >> T1m: ")?;
    self.trade_stats_1m.fmt(f)
  }
}

impl BookAccounting for BookWithStats {
  fn update_sell_orders(&mut self, rate: String, amount: f64) -> Option<f64> {
    let rate_f64 = rate.parse::<f64>();
    let prev_amount = self.book.update_sell_orders(rate, amount);
    if let Ok(rate_f64) = rate_f64 {
      self.stats.update_sell_orders(rate_f64, amount, prev_amount);
    };
    prev_amount
  }

  fn update_buy_orders(&mut self, rate: String, amount: f64) -> Option<f64> {
    let rate_f64 = rate.parse::<f64>();
    let prev_amount = self.book.update_buy_orders(rate, amount);
    if let Ok(rate_f64) = rate_f64 {
      self.stats.update_buy_orders(rate_f64, amount, prev_amount);
    };
    prev_amount
  }

  fn new_deal(&mut self, id: u64, rate: String, amount: f64) -> Result<f64, PoloError> {
    let rate_f64 = self.book.new_deal(id, rate, amount)?;
    Ok(rate_f64)
  }

  fn book_ref(&self) -> &Book {
    &self.book
  }
}

impl TimeStats for BookWithStats {
  fn update_stats_1s(&mut self) -> Vec<&Deal> {
    let timestamp = time::get_time();
    // cleanup deals time series
    self.book.deals.drain_until(time::Timespec { sec: timestamp.sec - 600, nsec: 0 });
    // update stats with last second of data
    let last_second = self.book.deals.vec_after(time::Timespec { sec: timestamp.sec - 1, nsec: timestamp.nsec });
    let stats = TradeStats::new(&last_second);
    self.trade_stats_1m = self.trade_stats_1m + &stats;
    self.trade_series_1s.push_front(stats);
    if let Some(stats_1m_ago) = self.trade_series_1s.get(60) {
      self.trade_stats_1m = self.trade_stats_1m - stats_1m_ago;
    }
    last_second
  }
}

/**
 ** Library functions
 **/

fn f64cmp(f1: &f64, f2: &f64) -> Ordering {
  f1.partial_cmp(f2).unwrap_or(Ordering::Equal)
}

fn hash_to_vec(hash: &HashMap<String, f64>) -> Vec<Record> {
  hash.iter().filter_map(|(rate_s, amount)| {
    rate_s.parse::<f64>()
      .and_then(|rate| { 
        Ok(Record { rate, amount: *amount }) 
      }).ok()
  }).collect()
}

fn update_sorted_vec(idx_r: Result<usize, usize>, vec: &mut Vec<Record>, stat: &mut f64, rate: f64, amount: f64, stat_cmp: bool) {
  if amount == 0.0 {
    if let Ok(idx) = idx_r {
      vec.remove(idx);
    };
    if *stat == rate || *stat == 0.0 {
      *stat = vec.first().map_or(*stat, |rec| rec.rate);
    }
  } else if amount > 0.0  {
    match idx_r {
      Ok(idx) => vec[idx].amount = amount,
      Err(idx) => vec.insert(idx, Record { rate, amount })
    };
    if stat_cmp || *stat == 0.0 {
      *stat = rate
    }
  };
}


/** 
 ** TESTS TESTS TESTS
 **/

#[cfg(test)]
mod tests {
  use super::BookWithStats;
  use crate::data::book::Book;
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
    book_stats.update_sell_orders(0.1, 0.0, None);
    assert_eq!(book_stats.min_sell, 0.13161901);
  }


  #[test]
  fn stats_update_sell_shift_zero() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_sell_orders(0.13161901, 0.0, Some(0.23709568));
    assert_eq!(book_stats.min_sell, 0.13164313);
  }


  #[test]
  fn stats_update_sell() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_sell_orders(0.1, 1.0, None);
    assert_eq!(book_stats.min_sell, 0.1);
  }


  #[test]
  fn stats_update_buy_zero() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_buy_orders(100.0, 0.0, None);
    assert_eq!(book_stats.max_buy, 0.13109621);
  }

  #[test]
  fn stats_update_buy_shift_zero() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_buy_orders(0.13109621, 0.0, Some(0.2331));
    assert_eq!(book_stats.max_buy, 0.13069621);
  }

  #[test]
  fn stats_update_buy() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_buy_orders(100.0, 1.0, None);
    assert_eq!(book_stats.max_buy, 100.0);
  }

  #[test]
  fn stats_skin() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.1111": 100.0, "0.1112": 100.0, "0.1113": 1000.0}, {"0.1003": 1.0, "0.1002": 1.0, "0.1001": 10.0}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let book_stats = BookWithStats::new(book).stats;
    assert_eq!(book_stats.skin_sell, 0.1112);
    assert_eq!(book_stats.skin_buy, 0.1002);
  }

  #[test]
  fn stats_surface() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.1110": 10.0, "0.1111": 100.0, "0.1112": 100.0, "0.1113": 1000.0}, {"0.1004": 0.1, "0.1003": 1.0, "0.1002": 1.0, "0.1001": 10.0}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let book_stats = BookWithStats::new(book).stats;
    assert_eq!(book_stats.surface_sell, 0.1111);
    assert_eq!(book_stats.surface_buy, 0.1003);
  }

  #[test]
  fn stats_surface_update() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.1110": 10.0, "0.1111": 100.0, "0.1112": 100.0, "0.1113": 1000.0}, {"0.1004": 0.1, "0.1003": 1.0, "0.1002": 1.0, "0.1001": 10.0}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_sell_orders(0.1109, 10.0, None);
    book_stats.update_buy_orders(0.1005, 0.1, None);
    assert_eq!(book_stats.surface_sell, 0.1110);
    assert_eq!(book_stats.surface_buy, 0.1004);
  }

}