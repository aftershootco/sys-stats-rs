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
    pub architecture: CPUArchitecture,
    pub num_of_cores: u32,
    pub average_cpu_usage: f32,
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
