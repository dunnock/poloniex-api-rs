use super::book::{Book, BookAccounting};
use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;
use ::error::PoloError;
use super::timeseries::Timeseries;
use super::tradestats::{TradeStats, TimeStats};
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
}


#[derive(Clone, Debug, PartialEq)]
pub struct BookWithStats {
  book: Book,
  pub stats: BookStats,
  pub trade_series_1s: Timeseries<TradeStats>,
  trade_stats_1m: TradeStats,
}

// BookStats operations

impl BookStats {
  pub fn new(book: &Book) -> BookStats {
    let (sum_buy, mut vec_buy) = hash_to_vec(&book.buy);
    vec_buy.sort_unstable_by(|rec1, rec2| f64cmp(&rec2.rate, &rec1.rate));
    let (sum_sell, mut vec_sell) = hash_to_vec(&book.sell);
    vec_sell.sort_unstable_by(|rec1, rec2| f64cmp(&rec1.rate, &rec2.rate));

    BookStats {
      min_sell: vec_sell.first().map_or(0.0, |rec| rec.rate),
      max_buy: vec_buy.first().map_or(0.0, |rec| rec.rate),
      sum_sell,
      sum_buy,
      vec_buy,
      vec_sell,
    }
  }

  pub fn update_sell(&mut self, rate: f64, amount: f64, prev_amount: Option<f64>) {
    let idx_r = self.vec_sell.binary_search_by(|rec| f64cmp(&rec.rate, &rate));
    let stat_cmp = self.min_sell > rate;
    update_sorted_vec(idx_r, &mut self.vec_sell, &mut self.min_sell, rate, amount, stat_cmp);
    self.sum_sell = self.sum_sell + amount - prev_amount.unwrap_or(0.0);
  }

  pub fn update_buy(&mut self, rate: f64, amount: f64, prev_amount: Option<f64>) {
    let idx_r = self.vec_buy.binary_search_by(|rec| f64cmp(&rate, &rec.rate));
    let stat_cmp = self.max_buy < rate;
    update_sorted_vec(idx_r, &mut self.vec_buy, &mut self.max_buy, rate, amount, stat_cmp);
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
      trade_series_1s: Timeseries::new(),
      trade_stats_1m: TradeStats::default(),
      book
    }
  }
}

impl fmt::Display for BookWithStats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "  ** STATS ")?;
    self.stats.fmt(f)?;
    if let Some(last_sec) = self.trade_series_1s.data.front() {
      write!(f, "\n  >> T1s: ")?;
      last_sec.fmt(f)?;
    }
    write!(f, "\n  >> T1m: ")?;
    self.trade_stats_1m.fmt(f)
  }
}

impl BookAccounting for BookWithStats {
  fn update_sell(&mut self, rate: String, amount: f64) -> Option<f64> {
    let rate_f64 = rate.parse::<f64>();
    let prev_amount = self.book.update_sell(rate, amount);
    if let Ok(rate_f64) = rate_f64 {
      self.stats.update_sell(rate_f64, amount, prev_amount);
    };
    prev_amount
  }

  fn update_buy(&mut self, rate: String, amount: f64) -> Option<f64> {
    let rate_f64 = rate.parse::<f64>();
    let prev_amount = self.book.update_buy(rate, amount);
    if let Ok(rate_f64) = rate_f64 {
      self.stats.update_buy(rate_f64, amount, prev_amount);
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
  fn update_stats_1s(&mut self) {
    let timestamp = time::get_time();
    // cleanup deals time series
    self.book.deals.drain_until(time::Timespec { sec: timestamp.sec - 600, nsec: 0 });
    // update stats with last second of data
    let last_second = self.book.deals.vec_after(time::Timespec { sec: timestamp.sec - 1, nsec: timestamp.nsec });
    let stats = TradeStats::new(last_second);
    self.trade_stats_1m = self.trade_stats_1m + &stats;
    self.trade_series_1s.add(stats);
    if self.trade_series_1s.data.len() > 60 {
      let stats_1m_ago = &self.trade_series_1s.data[self.trade_series_1s.data.len() - 60];
      self.trade_stats_1m = self.trade_stats_1m - stats_1m_ago;
    }
  }
}

/**
 ** Library functions
 **/

fn f64cmp(f1: &f64, f2: &f64) -> Ordering {
  f1.partial_cmp(f2).unwrap_or(Ordering::Equal)
}

fn hash_to_vec(hash: &HashMap<String, f64>) -> (f64, Vec<Record>) {
  let mut sum = 0.0;
  let vec = hash.iter().filter_map(|(rate_s, amount)| {
    rate_s.parse::<f64>()
      .and_then(|rate| { 
        sum += *amount; 
        Ok(Record { rate, amount: *amount }) 
      }).ok()
  }).collect();
  (sum, vec)
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
  fn stats_update_sell_shift_zero() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_sell(0.13161901, 0.0, Some(0.23709568));
    assert_eq!(book_stats.min_sell, 0.13164313);
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
  fn stats_update_buy_shift_zero() {
    let book_init = r#"{"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13109621": 0.2331, "0.13069621": 0.2331}]}"#;
    let book = Book::try_from(&json::parse(book_init).unwrap()).unwrap();
    let mut book_stats = BookWithStats::new(book).stats;
    book_stats.update_buy(0.13109621, 0.0, Some(0.2331));
    assert_eq!(book_stats.max_buy, 0.13069621);
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