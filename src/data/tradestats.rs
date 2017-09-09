use super::book::Deal;
use std::ops;
use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TradeStats {
  pub sum_sell: f64,
  pub sum_buy: f64,
  pub sum_sell_dest: f64,
  pub sum_buy_dest: f64,
  pub num_sell: u16,
  pub num_buy: u16,
}

// TradeStats operations

impl Default for TradeStats {
  fn default() -> TradeStats {
    TradeStats { sum_sell: 0.0, sum_sell_dest: 0.0, sum_buy: 0.0, sum_buy_dest: 0.0, num_sell: 0, num_buy: 0 }
  }
}

impl TradeStats {
  pub fn new(deals: Vec<&Deal>) -> TradeStats {
    deals.into_iter().fold(TradeStats::default(), |acc, deal| acc + deal)
  }
}

impl<'a> ops::Add<&'a TradeStats> for TradeStats {
  type Output = TradeStats;
  fn add(self, other: &TradeStats) -> TradeStats {
    TradeStats {
      sum_sell: self.sum_sell + other.sum_sell,
      sum_sell_dest: self.sum_sell_dest + other.sum_sell_dest,
      sum_buy: self.sum_buy + other.sum_buy,
      sum_buy_dest: self.sum_buy_dest + other.sum_buy_dest,
      num_sell: self.num_sell + other.num_sell,
      num_buy: self.num_buy + other.num_buy,
    }
  }
}

impl<'a> ops::Add<&'a Deal> for TradeStats {
  type Output = TradeStats;
  fn add(self, other: &Deal) -> TradeStats {
    let (buy, num_buy, buy_dest, sell, num_sell, sell_dest) = 
      if other.amount > 0.0 { (other.amount, 1, other.amount*other.rate, 0.0, 0, 0.0) } 
      else                  { (0.0, 0, 0.0, other.amount, 0, other.amount*other.rate) };
    TradeStats {
      sum_sell: self.sum_sell + sell,
      sum_sell_dest: self.sum_sell_dest + sell_dest,
      sum_buy: self.sum_buy + buy,
      sum_buy_dest: self.sum_buy_dest + buy_dest,
      num_sell: self.num_sell + num_sell,
      num_buy: self.num_buy + num_buy,
    }
  }
}

impl<'a> ops::Sub<&'a TradeStats> for TradeStats {
  type Output = TradeStats;
  fn sub(self, other: &TradeStats) -> TradeStats {
    TradeStats {
      sum_sell: self.sum_sell - other.sum_sell,
      sum_sell_dest: self.sum_sell_dest - other.sum_sell_dest,
      sum_buy: self.sum_buy - other.sum_buy,
      sum_buy_dest: self.sum_buy_dest - other.sum_buy_dest,
      num_sell: self.num_sell - other.num_sell,
      num_buy: self.num_buy - other.num_buy,
    }
  }
}

impl fmt::Display for TradeStats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SELL {} to {} rate {} num {} | BUY {} to {} rate {} num {}", 
      self.sum_sell, self.sum_sell_dest, self.sum_sell/self.sum_sell_dest, self.num_sell, 
      self.sum_buy, self.sum_buy_dest, self.sum_buy/self.sum_buy_dest, self.num_buy)
  }
}

pub trait TimeStats {
  fn update_stats_1s(&mut self);
}
