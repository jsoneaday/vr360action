pub mod client;
pub mod host;
pub mod model {
    pub mod method;
    pub mod request;
    pub mod response;
    pub mod result;
}
pub mod error;
pub mod driver {
    pub mod driver;
}

use client::WsConnection;
use dotenv::dotenv;
use driver::driver::Driver;
use std::{collections::BTreeMap, env};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<usize>().unwrap();

    let client = env::var("CLIENT");
    if let Ok(_client) = client {
        let mut conn = WsConnection::new(host.to_string(), port, false);
        _ = conn.connect().await;

        let mut driver = Driver::new(conn);

        let mut messages = BTreeMap::new();
        messages.insert("Hello".to_string(), "World".to_string());
        let result = driver.message(messages).await;
        match result {
            Ok(msg) => {         
                println!("Message response {:?}", msg);    
                driver.disconnect().await;
                println!("Disconnected");
            },
            Err(_err) => ()
        }
    } else {              
        host::listen(host, port).await;
    }         
}
