use tungstenite::{protocol::Message, error::Error};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use url;

// subscribe to trading pair ticker updates
pub async fn subscribe(
    connect_addr: &str,
    pairs: Vec<String>,
) -> Result<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Error>
 {

    let url = url::Url::parse(&connect_addr).map_err(|err| Error::Url(err.to_string().into()))?;
    let (mut ws_stream, _) = connect_async(url).await?;
    for pair in pairs.iter() {
        let msg = message_subscribe(pair).into();
        ws_stream.send(msg).await?;
    }
    Ok(ws_stream)
}

fn message_subscribe(channel: &str) -> Message {
    Message::text(format!(
        "{{ \"command\": \"subscribe\", \"channel\": \"{}\" }}",
        channel
    ))
}
