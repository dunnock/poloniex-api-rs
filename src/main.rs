extern crate poloniex;
extern crate tokio_core;
extern crate websocket;

use poloniex::push;
use tokio_core::reactor::Core;
use websocket::WebSocketError::{ResponseError};

fn main() {
  let mut core = Core::new().unwrap();
  let client = push::connect(&core.handle());
  core.run(client).unwrap();
}