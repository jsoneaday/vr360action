use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use lazy_static::lazy_static;

use crate::model::request::{RpcParams, RpcRequest};

lazy_static! {
    static ref DATA: Arc<RwLock<BTreeMap<String, String>>> = {
        let data = Arc::new(RwLock::new(BTreeMap::new()));

        data
    };
}


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
                match msg {
                    Message::Text(msg_text) => {
                        println!("Received text");
                        let data = Arc::clone(&DATA);
                        let mut write_data = data.write().await;

                        let message: RpcRequest = serde_json::from_str(&msg_text).unwrap();
                        println!("Deserialized RpcRequest");

                        match message.params {
                            RpcParams::Message(msg_param) => {
                                for mp in msg_param.iter() {
                                    write_data.insert(mp.0.to_string(), mp.1.to_string());
                                }
                            },
                            _ => ()
                        }                        

                        println!("The inserted data: {:?}", write_data);

                        write.send(Message::text("I got your message.".to_string())).await.expect("Failed to send replay");
                    }, 
                    _ => {
                        println!("Message given was not text: {:?}", msg);
                    }
                }
            }
        });
    }
}
