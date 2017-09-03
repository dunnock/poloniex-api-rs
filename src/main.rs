extern crate poloniex;
extern crate tokio_core;
extern crate futures;
extern crate websocket;
extern crate bus;

use poloniex::push::subscribe;
use poloniex::actors::{logger, book};
use tokio_core::reactor::Core;
use futures::future::{Future, ok};
use futures::Stream;
use websocket::OwnedMessage;
use bus::Bus;

static PAIRS: [&str; 1] = ["BTC_BCH"];

fn main() {
  let mut core = Core::new().unwrap();
  let mut bus = Bus::new(100);
  let pairs = PAIRS.iter().map(|p| String::from(*p)).collect();
  let updates = subscribe(pairs, &core.handle());

  let book = book::start(bus.add_rx());
  let logger = logger::start(bus.add_rx());

  let printed_updates = updates
    .and_then(move |(s, _)| {
      s.for_each(move |msg| { 
        if let OwnedMessage::Text(text) = msg { 
          bus.broadcast(text);
        }; 
        ok(()) 
      })
    });

  core.run(printed_updates).unwrap();
  book.join().unwrap();
  logger.join().unwrap();
}