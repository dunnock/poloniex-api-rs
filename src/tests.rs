use super::model;

#[test]
fn model_works() {
    let records = vec![model::Record { 
        kind: model::OrderType::Ask,
        rate: 0.001,
        amount: 10.1
    }];
    let _b: model::Book = model::Book {
        pairs: model::TradePairs::BtcBch,
        records
    };
}
