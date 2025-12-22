use crate::cpu::{CPUData, CPUUsage};
use std::process::Command;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

use super::{CPUArchitecture, CPUVendor};

impl CPUUsage {
    pub fn get_cpu_info() -> Result<CPUData, Box<dyn std::error::Error>> {
        Ok(CPUData {
            name: Self::get_name(),
            vendor: Self::get_cpu_vendor(),
            architecture: Self::get_architecture(),
            num_of_cores: Self::num_of_cores(),
            logical_processors: Self::logical_processors(),
            average_cpu_usage: Self::average_usage(),
        })
    }

    fn get_name() -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lscpu | grep 'Model name' | awk -F: '{print $2}'")
            .output()
            .expect("Failed to execute command");

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn get_cpu_vendor() -> CPUVendor {
        let s = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        let vendor_id = s.cpus().get(0).unwrap().vendor_id();
        CPUVendor::from_vendor_id(vendor_id)
    }

    pub fn num_of_cores() -> u32 {
        sysinfo::System::physical_core_count().unwrap_or(0) as u32
    }

    pub fn logical_processors() -> u32 {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        sys.cpus().len() as u32
    }

    pub fn average_usage() -> f32 {
        match sys_info::loadavg() {
            Ok(load) => load.one as f32,
            Err(_) => 0.0,
        }
    }

    fn get_architecture() -> CPUArchitecture {
        let arch = std::env::consts::ARCH.to_string();

        match arch.trim() {
            "i386" => CPUArchitecture::I386,
            "x86_64" => CPUArchitecture::X86_64,
            "riscv32" => CPUArchitecture::RiscV32,
            "riscv64" => CPUArchitecture::RiscV64,
            "arm" => CPUArchitecture::Arm,
            "arm64" => CPUArchitecture::Arm64,
            "aarch64" => CPUArchitecture::Arm64, // aarch64 is shown on linux systems
            _ => CPUArchitecture::Unknown,
        }
    }
}
