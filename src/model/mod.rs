pub enum TradePairs {
    BtcEth,
    BtcBch,
}

pub enum OrderType {
    Ask,
    Bid,
}

pub struct Book {
    pub pairs: TradePairs,
    pub records: Vec<Record>
}

// sorted by rate, groupd by price
pub struct Record {
    pub kind: OrderType, 
    pub rate: f64, 
    pub amount: f64,
}
