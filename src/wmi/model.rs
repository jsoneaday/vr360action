#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Win32_Processor {
    Name: String,
    NumberOfCores: u32,
    NumberOfLogicalProcessors: u32,
    MaxClockSpeed: u32,
}

// Struct to hold motherboard info
#[derive(Deserialize, Debug)]
struct Win32_BaseBoard {
    Manufacturer: String,
    Product: String,
}

// Struct to hold memory info
#[derive(Deserialize, Debug)]
struct Win32_OperatingSystem {
    TotalVisibleMemorySize: u64, // In KB
    FreePhysicalMemory: u64,     // In KB
}