use ::data::messages::{BookUpdate, RecordUpdate, BookRecord};
use ::data::trade::TradeBook;
use std::str::FromStr;
use super::Actor;
use std::sync::{Arc, Mutex};
use ::error::PoloError;

#[derive(Clone)]
pub struct Accountant {
  tb: Arc<Mutex<TradeBook>>,
}

impl Accountant {
  pub fn new(tb: &Arc<Mutex<TradeBook>>) -> Accountant {
    Accountant {
      tb: tb.clone()
    }
  }
}

impl Actor for Accountant {
  fn process_message(&mut self, msg: String) -> Result<(),PoloError> {
    let err = |title| PoloError::wrong_data(format!("{} {:?}", title, msg));

    let update = BookUpdate::from_str(&msg)?;
    for rec in update.records {
      let mut tb = self.tb.lock().unwrap();
      match rec {
        RecordUpdate::Initial(book) => {
          tb.add_book(book, update.book_id);
        },
        RecordUpdate::SellTotal(BookRecord {rate, amount}) => {
          let book = tb.book_by_id(&update.book_id)
            .ok_or_else(|| err("book not initialized"))?;
          book.update_sell(rate, amount);
        },
        RecordUpdate::BuyTotal(BookRecord {rate, amount}) => {
          let book = tb.book_by_id(&update.book_id)
            .ok_or_else(|| err("book not initialized"))?;
          book.update_buy(rate, amount);
        },
        RecordUpdate::Sell(deal) => {
          let book = tb.book_by_id(&update.book_id)
            .ok_or_else(|| err("book not initialized"))?;
          println!("new sell {:?}", deal);
          book.new_deal(deal.id, deal.rate, -deal.amount)?;
        },
        RecordUpdate::Buy(deal) => {
          let book = tb.book_by_id(&update.book_id)
            .ok_or_else(|| err("book not initialized"))?;
          println!("new buy {:?}", deal);
          book.new_deal(deal.id, deal.rate, deal.amount)?;
        },
      }
    };
    Ok(())
  }
}


#[cfg(test)]
mod tests {
  use super::Accountant;
  use bus::Bus;
  use ::actors::Actor;
  use ::data::trade::TradeBook;
  use ::data::messages::{BookUpdate, RecordUpdate};
  use std::str::FromStr;
  use std::sync::{Arc, Mutex};
  use std::thread;


  #[test]
  fn initial_order() {
    let tb = Arc::new(Mutex::new(TradeBook::new()));
    let mut actor = Accountant::new(&tb);
    let order = String::from(r#"[189, 5130995, [["i", {"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13169621": 0.2331}]}]]]"#);

    let mut bus = Bus::new(1);
    let ch1 = bus.add_rx();
    let th = thread::spawn(move || actor.listen(ch1));
    bus.broadcast(Some(order.clone()));
    bus.broadcast(None);
    println!("{:?}", th.join());

    let mut tb_mut = tb.lock().unwrap();
    let actor_book = tb_mut.book_by_id(&189).unwrap().book_ref();
    match BookUpdate::from_str(&order).unwrap().records[0] {
      RecordUpdate::Initial(ref book) => assert_eq!(*book, *actor_book),
      _ => panic!("BookUpdate::from_str were not able to parse RecordUpdate::Initial")
    }
  }
}