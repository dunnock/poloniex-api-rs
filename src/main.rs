extern crate poloniex;
extern crate tokio_core;
extern crate futures;
extern crate websocket;

use poloniex::push::subscribe;
use poloniex::actors::book;
use tokio_core::reactor::Core;
use futures::future::{Future, ok};
use futures::Stream;
use websocket::OwnedMessage;

static PAIRS: [&str; 1] = ["BTC_BCH"];

fn main() {
  let mut core = Core::new().unwrap();
  let pairs = PAIRS.iter().map(|p| String::from(*p)).collect();
  let updates = subscribe(pairs, &core.handle());

  let (tx, th1) = book::start_updater();

  let printed_updates = updates
    .and_then(move |(s, _)| {
      let tx = tx.clone();
      s.for_each(move |msg| { 
        if let OwnedMessage::Text(msg) = msg { 
          tx.send(msg).unwrap();
        }; 
        ok(()) 
      })
    });

  core.run(printed_updates).unwrap();
  th1.join().unwrap();
}