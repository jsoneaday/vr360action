use crate::client::WsConnection;
use crate::model::method::Method;
use crate::model::request::RpcParams;

pub type TungsteniteResult = Result<tungstenite::Message, tungstenite::Error>;

#[allow(unused)]
pub struct SurrealDriver {
    conn: WsConnection
}

#[allow(unused)]
impl SurrealDriver {
    pub fn new(conn: WsConnection) -> Self {
        Self {
            conn
        }
    }

    pub async fn disconnect(&mut self) {
        self.conn.disconnect().await;
    }

    pub async fn info(&mut self) -> TungsteniteResult {
        self.conn.rpc(Method::Info, RpcParams::Array(Vec::new())).await
    }
}
