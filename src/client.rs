#[allow(unused)]
use futures_util::{SinkExt, StreamExt, future, pin_mut};
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tungstenite::{client::IntoClientRequest, Error, Message, Result};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use super::{error::MyError, model::request::{RpcRequest, RpcParams}};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;


pub struct WsConnection {
    last_request_id: Arc<RwLock<Uuid>>,
    use_tls: bool,
    host: String,
    port: usize,
    writer: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    reader: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>
}

impl WsConnection {
    pub fn new(host: String, port: usize, use_tls: bool) -> Self {
        WsConnection {
            last_request_id: Arc::new(RwLock::new(Uuid::new_v4())),
            use_tls,
            host,
            port,
            writer: None,
            reader: None
        }
    }

    pub async fn connect(&mut self) -> Result<(), MyError> {   
        let conn_result = connect_async(
            format!("{}{}:{}/rpc", if self.use_tls == true { "wss://" } else { "ws://" }, 
            self.host, self.port).as_str().into_client_request().unwrap()
        )
        .await;
        
        if let Some(err) = conn_result.as_ref().err() {
            println!("Failure: {:?}", err);
            return Err(MyError::GeneralError);
        };
        
        let (ws_socket, _) = conn_result.unwrap();
        let (writer, reader) = ws_socket.split();
        self.writer = Some(writer);    
        self.reader = Some(reader);  

        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        self.writer.as_mut().unwrap().close().await
    }

    pub async fn rpc(&mut self, params: RpcParams) -> Result<Message, Error> {  
        let mut last_request_id = self.last_request_id.write().await;
        *last_request_id = Uuid::new_v4();
        let rpc_req: RpcRequest = RpcRequest::new(last_request_id.to_string(), params);
        let json = serde_json::to_string(&rpc_req);
        let json_txt = json.unwrap();
                
        match (&mut self.writer, &mut self.reader) {
            (Some(writer), Some(reader)) => {
                writer.send(Message::Text(json_txt.clone())).await?;
                println!("Message sent: {}", json_txt);
                
                loop {
                    tokio::select! {
                        opt_msg = reader.next() => {
                            match opt_msg {
                                Some(result_msg) => {
                                    return result_msg;
                                },
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                }
            },
            _ => {}
        }        

        Ok(Message::Text("invalid nothing returned".to_string()))
    }
}