use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;

pub async fn listen(host: String, port: usize) {
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(addr).await.expect("Listener failed to bind to addr");
    println!("Listener started");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Failed websocket handshake");  
            println!("Connection accepted");

            let (mut write, mut read) = ws_stream.split();            

            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let reply = Message::text(format!("You said: {}. Hello world! From the server", msg.to_text().unwrap()));
                    write.send(reply).await.expect("Failed to send replay");
                }
            }
        });
    }
}
