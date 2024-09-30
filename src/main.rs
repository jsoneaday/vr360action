pub mod client;
pub mod server;
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
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<usize>().unwrap();

    let client = env::var("CLIENT");
    if let Ok(_client) = client {
        let mut conn = WsConnection::new(host.to_string(), port, false);
        _ = conn.connect().await;  
    } else {              
        server::listen(host, port).await;
    }         
}
