# poloniex-api-rs

Poloniex orders/trades feed subscriptions via webscoket interface for Rust. It does not consume API calls limit as well as provides faster access to market events.

# Usage

It's still not published, hence access via git:

```Cargo.toml
[depencencies]
poloniex-api = { git = "https://github.com/dunnock/poloniex-api-rs" }
tokio = "0.2"
```

```main.rs
use poloniex::data::book::{Book, TradePairs, Deal};
use poloniex::data::messages::BookUpdate;
use poloniex::push::{subscribe, Message};

const URL: &str = "wss://api2.poloniex.com:443";

/// Process book RecordUpdate returning if update is a deal. Return:
///  - None - if update is order change
///  - Some(Trade) - when update is actual deal
fn process_book_update(
    book: &mut Book,
    record: RecordUpdate,
    datetime: DateTime<Utc>,
) -> Option<&Deal> {
    match record {
        RecordUpdate::SellTotal(BookRecord { rate, amount }) => {
            book.update_sell_orders(rate, amount);
            None
        }
        RecordUpdate::BuyTotal(BookRecord { rate, amount }) => {
            book.update_buy_orders(rate, amount);
            None
        }
        RecordUpdate::Sell(TradeRecord {
            id: _,
            tid: _,
            rate,
            amount,
        }) => {
            if book.new_deal(rate, amount).is_ok() {
              book.deals.data.back()
            } else {
              None
            }
        }
        RecordUpdate::Buy(TradeRecord {
            id: _,
            tid: _,
            rate,
            amount,
        }) => {
            if book.new_deal(rate, amount).is_ok() {
              book.deals.data.back()
            } else {
              None
            }
        }
        RecordUpdate::Initial(b) => {
            book.reset_orders();
            b.sell.iter().for_each(|(rate, amount)| {
                book.update_sell_orders(rate.to_owned(), *amount);
            });
            b.buy.iter().for_each(|(rate, amount)| {
                book.update_buy_orders(rate.to_owned(), *amount);
            });
            None
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let book = Book::new(TradePairs::UsdtBtc);
    match subscribe(URL, vec!["USDT_BTC".to_owned()]).await {
        Ok(mut stream) => {
            while let Some(Ok(Message::Text(text))) = stream.next().await {
                // Received update message from poloniex via websocket
                debug!("{}", text);
                if let Ok(update) = BookUpdate::from_str(&text) {
                    //parse order record from exchange
                    update.records.into_iter()
                        // update book records with market info and return Trade deal data if any
                        .filter_map(|record| {
                            process_book_update(book, record, datetime)
                        })
                        //for trade deals calculate and execute strategy
                        .map(|deal| {
                          debug!("{:?}", deal);
                        });
                };
            }
        }
        Err(err) => {
            error!("Failed to connect: {:?}", err);
        }
    }
}

```
