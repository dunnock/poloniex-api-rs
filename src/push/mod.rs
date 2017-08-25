use tokio_core::reactor::Handle;
use futures::future::Future;
use futures::Stream;
use futures::stream::StreamFuture;
use websocket::{ClientBuilder};
use websocket::client::async::{TlsStream,TcpStream};

const URL: &'static str = "wss://api.poloniex.com";

pub fn subscribe(pairs: &Vec<String>, handle: &Handle) -> StreamFuture<TlsStream<TcpStream>> {
	ClientBuilder::new(URL)
		.unwrap()
		.async_connect_secure(None, handle)
		.and_then(|(duplex, _)| {
			let (sink, stream) = duplex.split();
      stream.into_future()
    })
}