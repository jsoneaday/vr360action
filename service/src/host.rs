use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use lazy_static::lazy_static;
use crate::{model::request::{RpcParams, RpcRequest}, repo::{base::{ConnGetter, DbRepo, Repository}, pc::insert_pc}};

lazy_static! {
    static ref DATA: Arc<RwLock<HashMap<String, String>>> = {
        let data = Arc::new(RwLock::new(HashMap::new()));

        data
    };
}


pub async fn listen(host: String, port: usize) {
    let addr = format!("{}:{}", host, port);
    println!("addr: {:?}", addr);
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
                        let message: RpcRequest = serde_json::from_str(&msg_text).unwrap();

                        match message.clone().params {
                            RpcParams::Message(msg_param) => {
                                let mut repo = DbRepo::init().await;
                                let conn = repo.get_conn();
                                let hostname = get_hostname(msg_param.clone());
                                for mp in msg_param.iter() {
                                    _ = insert_pc(conn, hostname.to_string(), mp.0.to_string(), mp.1.to_string()).await;
                                    println!("Inserted data: {:?} {:?}", mp.0.to_string(), mp.1.to_string());
                                }

                                repo.disconnect().await;                                
                            },
                            _ => ()
                        }                                       

                        write.send(Message::text(format!("I got your message: {:?}", message))).await.expect("Failed to send replay");
                    }, 
                    _ => {
                        println!("Message given was not text: {:?}", msg);
                    }
                }
            }
        });
    }
}

fn get_hostname(data: HashMap<String, String>) -> String {
    let mut hostname = "".to_string();
    for mp in data.iter() {
        if mp.0.contains("sys:Name") {
            hostname = mp.1.to_string();
        }
    }

    hostname
}