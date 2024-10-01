use std::collections::HashMap;
use tungstenite::Message;

use crate::client::WsConnection;
use crate::model::request::RpcParams;
use crate::wmi::wmi::get_pc_info;

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

    pub async fn message(&mut self, messages: HashMap<String, String>) -> TungsteniteResult {
        self.conn.rpc(RpcParams::Message(messages)).await
    }

    pub async fn pc_info(&mut self) -> Result<Message, Box<dyn std::error::Error>> {
        // get pc info
        let pc = get_pc_info();

        match pc {
            Ok(pc) => {
                let sys = pc.sys.iter().next().unwrap();
                let os = pc.os.iter().next().unwrap();
                let mb = pc.mb.iter().next().unwrap();
                let proc = pc.proc.iter().next().unwrap();

                let mut info_messages = HashMap::new();
                info_messages.insert("sys:Name".to_string(), sys.Name.to_string());
                info_messages.insert("os:TotalVisibleMemorySize".to_string(), os.TotalVisibleMemorySize.to_string());
                info_messages.insert("os:FreePhysicalMemory".to_string(), os.FreePhysicalMemory.to_string());
                info_messages.insert("mb:Manufacturer".to_string(), mb.Manufacturer.to_string());
                info_messages.insert("mb:Product".to_string(), mb.Product.to_string());
                info_messages.insert("proc:Name".to_string(), proc.Name.to_string());
                info_messages.insert("proc:NumberOfCores".to_string(), proc.NumberOfCores.to_string());

                match self.conn.rpc(RpcParams::Message(info_messages)).await {
                    Ok(msg) => Ok(msg),
                    Err(e) => Err(Box::new(e))
                }
            },
            Err(e) => Err(e)
        }        
    }
}
