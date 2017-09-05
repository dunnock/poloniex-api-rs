use super::data::book;
use std::collections::HashMap;

#[test]
fn model_works() {
    let _records = vec![book::Record::new(String::from("0.001"), 10.1)];
    let mut rec_hash = HashMap::new();
    rec_hash.insert(String::from("0.001"), 10.1);
    let _b: book::Book = book::Book {
        pair: book::TradePairs::BtcBch,
        sell: rec_hash.clone(),
        buy: rec_hash
    };
}
