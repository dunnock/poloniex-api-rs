
#[derive(Clone, Debug)]
pub enum TradePairs {
  BtcEth,
  BtcBch,
}

// sorted by rate, groupd by price
#[derive(Clone, Debug)]
pub struct Record {
  pub rate: f64, 
  pub amount: f64,
}

#[derive(Clone, Debug)]
pub struct Book {
  pub pairs: TradePairs,
  pub sell: Vec<Record>,
  pub buy: Vec<Record>
}

