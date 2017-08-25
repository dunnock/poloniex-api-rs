use std::thread;
use std::io::stdin;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};
use websocket::async::client::{ClientNew};
use websocket::client::async::{TlsStream, TcpStream};

const URL: &'static str = "wss://api.poloniex.com";

pub type PushClientFuture = ClientNew<TlsStream<TcpStream>>;

pub fn connect(handle: &Handle) -> PushClientFuture {
	ClientBuilder::new(URL)
		.unwrap()
		.async_connect_secure(None, handle)
}

/*
pub fn subscribe(pair: &str, connection: ) {
  connect
		.and_then(|(duplex, _)| {
			let (sink, stream) = duplex.split();
			stream.filter_map(|message| {
				                  println!("Received Message: {:?}", message);
				                  match message {
				                      OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
				                      OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
				                      _ => None,
				                  }
				                 })
			      .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
			      .forward(sink)
		});
	core.run(runner).unwrap();

}*/