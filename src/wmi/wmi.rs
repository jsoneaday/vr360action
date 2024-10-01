use wmi::{COMLibrary, WMIConnection};

use super::model::{Pc, Win32_BaseBoard, Win32_OperatingSystem, Win32_Processor};

pub fn get_pc_info() -> Result<Pc, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    // Create a new WMI connection
    let wmi_con = WMIConnection::new(com_con.into())?;

    let os: Vec<Win32_OperatingSystem> = wmi_con.query()?;
    let mb: Vec<Win32_BaseBoard> = wmi_con.query()?;
    let proc: Vec<Win32_Processor> = wmi_con.query()?;

    let pc= Pc {
        os,
        mb,
        proc
    };
    println!("pc: {:?}", pc);
    Ok(pc)
}