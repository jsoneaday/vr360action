#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Win32_Processor {
    pub Name: String,
    pub NumberOfCores: u32,
    pub NumberOfLogicalProcessors: u32,
    pub MaxClockSpeed: u32,
}

// Struct to hold motherboard info
#[derive(Deserialize, Debug)]
pub struct Win32_BaseBoard {
    pub Manufacturer: String,
    pub Product: String,
}

// Struct to hold memory info
#[derive(Deserialize, Debug)]
pub struct Win32_OperatingSystem {
    pub TotalVisibleMemorySize: u64, // In KB
    pub FreePhysicalMemory: u64,     // In KB
}

#[derive(Deserialize, Debug)]
pub struct Win32_ComputerSystem {
    pub Name: String,
}

#[derive(Deserialize, Debug)]
pub struct Pc {
    pub sys: Vec<Win32_ComputerSystem>,
    pub os: Vec<Win32_OperatingSystem>,
    pub mb: Vec<Win32_BaseBoard>,
    pub proc: Vec<Win32_Processor>
}