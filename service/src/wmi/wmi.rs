use wmi::{COMLibrary, WMIConnection};

use super::model::{Pc, Win32_BaseBoard, Win32_ComputerSystem, Win32_OperatingSystem, Win32_Processor};

pub fn get_pc_info() -> Result<Pc, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let sys: Vec<Win32_ComputerSystem> = wmi_con.query()?;
    println!("sys: {:?}", sys);
    let os: Vec<Win32_OperatingSystem> = wmi_con.query()?;
    let mb: Vec<Win32_BaseBoard> = wmi_con.query()?;
    let proc: Vec<Win32_Processor> = wmi_con.query()?;

    let pc= Pc {
        sys,
        os,
        mb,
        proc
    };
    
    Ok(pc)
}