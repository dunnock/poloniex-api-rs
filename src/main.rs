extern crate poloniex;
extern crate tokio_core;
extern crate futures;

use poloniex::push::subscribe;
use tokio_core::reactor::Core;
use futures::future::{ok, Future};
use futures::Stream;

static PAIRS: [&str; 1] = ["BTC_BCH"];

fn main() {
  let mut core = Core::new().unwrap();
  let pairs = PAIRS.iter().map(|p| String::from(*p)).collect();
  let updates = subscribe(pairs, &core.handle());
  let printed_updates = updates
    .and_then(|(s, _)| s.into_future().map_err(|e| e.0))
    .and_then(|(msg, _)| {
      println!("{:?}", msg);
      ok(msg)
    } );
  core.run(printed_updates).unwrap();
}