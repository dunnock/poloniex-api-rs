use super::data::book;
use crate::data::book::BookAccounting;
use crate::data::timeseries::Timeseries;
use std::collections::HashMap;

#[test]
fn model_works() {
    let mut rec_hash = HashMap::new();
    rec_hash.insert(String::from("0.001"), 10.1);
    let mut _b: book::Book = book::Book {
        pair: book::TradePairs::BtcBch,
        sell: rec_hash.clone(),
        buy: rec_hash,
        deals: Timeseries::new(),
        last_updated: time::get_time()
    };
    _b.update_buy_orders("0.001".to_owned(), 10.0);
    assert_eq!(_b.book_ref().buy["0.001"], 10.0);
}
