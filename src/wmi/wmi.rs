use serde::Deserialize;
use uuid::Variant;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};
use std::collections::HashMap;
use futures::executor::block_on;

pub fn get_pc_info() -> Result<HashMap<String, String>, std::error::Error> {
    let com_con = COMLibrary::new()?;
    // Create a new WMI connection
    let wmi_con = WMIConnection::new(com_con.into())?;

    let cpu = wmi_con.query()?;

}