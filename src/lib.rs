pub mod model;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let rec = [model::Record { 
            kind: model::OrderType::Ask,
            rate: 0.001,
            amount: 10
        }];
        let b: model::Book = model::Book {
            pairs: model::TradePairs::BtcBch,
            records
        };
    }
}
