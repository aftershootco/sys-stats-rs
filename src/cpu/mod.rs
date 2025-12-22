use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub struct CPUUsage;

#[derive(Debug, Serialize, Deserialize)]
pub struct CPUData {
    pub name: String,
    pub vendor: CPUVendor,
    pub architecture: CPUArchitecture,
    pub num_of_cores: u32,
    pub logical_processors: u32,
    pub average_cpu_usage: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CPUVendor {
    Intel,
    AMD,
    Apple,
    Qualcomm,
    Nvidia,
    // Virtual Machines
    VirtualPC,
    Rosetta2,
    Other,
}

impl CPUVendor {
    /// Maps vendor ID strings to CPUVendor enum variants
    pub fn from_vendor_id(vendor_id: &str) -> Self {
        match vendor_id {
            "GenuineIntel" => CPUVendor::Intel,
            "AuthenticAMD" => CPUVendor::AMD,
            "Apple" => CPUVendor::Apple, // For Apple Silicon
            "Qualcomm" => CPUVendor::Qualcomm,
            "Nvidia" => CPUVendor::Nvidia,
            // Virtual Machines
            "ConnectixCPU" | "Virtual CPU " | "Microsoft Hv" => CPUVendor::VirtualPC,
            _ => CPUVendor::Other,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CPUArchitecture {
    Arm,
    Arm64,
    I386,
    X86_64,
    RiscV32,
    RiscV64,
    Unknown,
}
