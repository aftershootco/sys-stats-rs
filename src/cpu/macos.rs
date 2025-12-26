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

    /// Helper function to get sysctl values
    fn get_sysctl_value(key: &str) -> Option<String> {
        Command::new("sysctl")
            .arg("-n")
            .arg(key)
            .output()
            .ok()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_name() -> String {
        Self::get_sysctl_value("machdep.cpu.brand_string")
            .unwrap_or_else(|| "Unknown CPU".to_string())
    }

    fn get_cpu_vendor() -> CPUVendor {
        // Check if running under Rosetta2 first (Intel code on Apple Silicon)
        if Self::is_rosetta2() {
            return CPUVendor::Rosetta2;
        }

        // Get CPU brand string to check for Apple Silicon
        let cpu_brand = Self::get_sysctl_value("machdep.cpu.brand_string").unwrap_or_default();

        if cpu_brand.contains("Apple") {
            return CPUVendor::Apple;
        }

        // Get vendor ID from sysinfo for other cases
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        if let Some(cpu) = system.cpus().first() {
            let vendor_id = cpu.vendor_id();

            // For native Intel hardware, return Intel
            if vendor_id == "GenuineIntel" {
                return CPUVendor::Intel;
            }

            // Fallback to generic vendor mapping
            CPUVendor::from_vendor_id(vendor_id)
        } else {
            CPUVendor::Other
        }
    }

    /// Helper function to detect if running under Apple Rosetta 2
    fn is_rosetta2() -> bool {
        Self::get_sysctl_value("sysctl.proc_translated")
            .map(|value| value == "1")
            .unwrap_or(false)
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
        sys_info::loadavg().unwrap().one as f32
    }

    fn get_architecture() -> CPUArchitecture {
        Command::new("uname")
            .arg("-m")
            .output()
            .ok()
            .and_then(|output| {
                let arch = String::from_utf8_lossy(&output.stdout);
                match arch.trim() {
                    "i386" => Some(CPUArchitecture::I386),
                    "x86_64" => Some(CPUArchitecture::X86_64),
                    "arm64" => Some(CPUArchitecture::Arm64),
                    _ => None,
                }
            })
            .unwrap_or(CPUArchitecture::Unknown)
    }
}
