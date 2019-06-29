use futures::future::{ok, err, Future};
use futures::Sink;
use websocket::{ClientBuilder, Message};
use websocket::client::r#async;

type Client = r#async::ClientNew<r#async::TlsStream<r#async::TcpStream>>;

// subscribe to trading pair ticker updates
pub fn subscribe(url: &str, pairs: Vec<String>) -> Client {
	let client_future = ClientBuilder::new(url)
		.unwrap()
		.async_connect_secure(None)
		.and_then(move |(mut client, hdr)| {
			for pair in pairs.iter() {
				let msg = message_subscribe(pair).into();
				match client.start_send(msg) {
					Err(status) => return err(status),
					_ => ()
				};
			}
			match client.poll_complete() {
				Err(status) => return err(status),
				_ => return ok((client, hdr))
			}
    });
		Box::new(client_future)
}

fn message_subscribe(channel: &str) -> Message {
	Message::text(format!("{{ \"command\": \"subscribe\", \"channel\": \"{}\" }}", channel))
}