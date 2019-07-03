use futures::future::Future;
use futures::{Sink};
use websocket::{ClientBuilder, Message, WebSocketError};
use websocket::r#async::{TcpStream, Client};
use websocket::client::r#async::TlsStream;

type ClientTcp = Client<TlsStream<TcpStream>>;

// subscribe to trading pair ticker updates
pub fn subscribe(url: &str, pairs: Vec<String>) -> impl Future<Item=ClientTcp, Error=WebSocketError> {
	ClientBuilder::new(url).unwrap()
		.async_connect_secure(None)
		.and_then(move |(mut client, _)| {
			for pair in pairs.iter() {
				let msg = message_subscribe(pair).into();
				if let Err(status) = client.start_send(msg) {
					 return Err(status);
				};
			};
			match client.poll_complete() {
				Err(status) => Err(status),
				_ => Ok(client)
			}
    })
}

fn message_subscribe(channel: &str) -> Message {
	Message::text(format!("{{ \"command\": \"subscribe\", \"channel\": \"{}\" }}", channel))
}