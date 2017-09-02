use super::data::book;

#[test]
fn model_works() {
    let records = vec![book::Record { 
        rate: 0.001,
        amount: 10.1
    }];
    let _b: book::Book = book::Book {
        pairs: book::TradePairs::BtcBch,
        sell: records.clone(),
        buy: records
    };
}
