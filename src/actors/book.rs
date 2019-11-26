use super::Processor;
use crate::data::messages::{BookRecord, BookUpdate, RecordUpdate};
use crate::data::trade::TradeBook;
use crate::error::PoloError;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Accountant {
    tb: Arc<Mutex<TradeBook>>,
}

impl Accountant {
    pub fn new(tb: Arc<Mutex<TradeBook>>) -> Accountant {
        Accountant { tb }
    }
}

impl Processor for Accountant {
    fn process_message(&mut self, msg: String) -> Result<(), PoloError> {
        let err = |title| PoloError::wrong_data(format!("{} {:?}", title, msg));

        let update = BookUpdate::from_str(&msg)?;
        for rec in update.records {
            let mut tb = self.tb.lock().unwrap();
            match rec {
                RecordUpdate::Initial(book) => {
                    tb.add_book(book, update.book_id);
                }
                RecordUpdate::SellTotal(BookRecord { rate, amount }) => {
                    let book = tb
                        .book_by_id(update.book_id)
                        .ok_or_else(|| err("book not initialized"))?;
                    book.update_sell_orders(rate, amount);
                }
                RecordUpdate::BuyTotal(BookRecord { rate, amount }) => {
                    let book = tb
                        .book_by_id(update.book_id)
                        .ok_or_else(|| err("book not initialized"))?;
                    book.update_buy_orders(rate, amount);
                }
                RecordUpdate::Sell(deal) => {
                    let book = tb
                        .book_by_id(update.book_id)
                        .ok_or_else(|| err("book not initialized"))?;
                    book.new_deal(deal.id, deal.rate, -deal.amount)?;
                }
                RecordUpdate::Buy(deal) => {
                    let book = tb
                        .book_by_id(update.book_id)
                        .ok_or_else(|| err("book not initialized"))?;
                    book.new_deal(deal.id, deal.rate, deal.amount)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Accountant;
    use crate::actors::Processor;
    use crate::data::messages::{BookUpdate, RecordUpdate};
    use crate::data::trade::TradeBook;
    use std::str::FromStr;
    use std::sync::{Arc, Mutex};

    #[test]
    fn initial_order() {
        let tb = Arc::new(Mutex::new(TradeBook::new()));
        let mut accountant = Accountant::new(tb.clone());
        let order = String::from(r#"[189, 5130995, [["i", {"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13169621": 0.2331}]}]]]"#);

        accountant.process_message(order.clone()).unwrap();

        let mut tb_mut = tb.lock().unwrap();
        let actor_book = tb_mut.book_by_id(189).unwrap().book_ref();
        match BookUpdate::from_str(&order).unwrap().records[0] {
            RecordUpdate::Initial(ref book) => assert_eq!((&book.sell, &book.buy), (&actor_book.sell, &actor_book.buy)),
            _ => panic!("BookUpdate::from_str were not able to parse RecordUpdate::Initial"),
        }
    }
}
