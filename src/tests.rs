use super::data::book;
use std::collections::HashMap;
use data::book::BookAccounting;
use data::timeseries::Timeseries;

#[test]
fn model_works() {
  let mut rec_hash = HashMap::new();
  rec_hash.insert(String::from("0.001"), 10.1);
  let mut _b: book::Book = book::Book {
    pair: book::TradePairs::BtcBch,
    sell: rec_hash.clone(),
    buy: rec_hash,
    deals: Timeseries::new()
  };
  _b.update_buy("0.001".to_owned(), 10.0);
  assert_eq!(_b.book_ref().buy["0.001"], 10.0);
}
