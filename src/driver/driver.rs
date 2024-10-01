use std::collections::BTreeMap;
use crate::client::WsConnection;
use crate::model::request::RpcParams;

pub type TungsteniteResult = Result<tungstenite::Message, tungstenite::Error>;

#[allow(unused)]
pub struct Driver {
    conn: WsConnection
}

#[allow(unused)]
impl Driver {
    pub fn new(conn: WsConnection) -> Self {
        Self {
            conn
        }
    }

    pub async fn disconnect(&mut self) {
        self.conn.disconnect().await;
    }

    pub async fn message(&mut self, messages: BTreeMap<String, String>) -> TungsteniteResult {
        self.conn.rpc(RpcParams::Message(messages)).await
    }

    pub async fn pc_info(&mut self) -> TungsteniteResult {
        // get pc info

        // setup tree
        let info_messages = BTreeMap::new();

        self.conn.rpc(RpcParams::Message(info_messages)).await
    }
}
