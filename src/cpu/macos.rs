use crate::cpu::{CPUData, CPUUsage};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use std::process::Command;

use super::{CPUArchitecture, CPUVendor};

impl CPUUsage {
    pub fn get_cpu_info() -> Result<CPUData, Box<dyn std::error::Error>> {
        Ok(CPUData {
            name: Self::get_name(),
            vendor: Self::get_cpu_vendor(),
            architecture: Self::get_architecture(),
            num_of_cores: Self::num_of_cores(),
            average_cpu_usage: Self::average_usage(),
        })
    }

    fn get_name() -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg("sysctl -n machdep.cpu.brand_string")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8_lossy(&output.stdout);
        name.trim().to_string()
    }

    fn get_cpu_vendor() -> CPUVendor {
        let s = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        let vendor_id = s.cpus().get(0).unwrap().vendor_id();
        
        // Special handling for Apple Silicon and Rosetta2 on macOS
        if vendor_id.is_empty() || vendor_id == "Apple" {
            // Check if this is Apple Silicon
            if let Ok(output) = Command::new("sysctl")
                .arg("-n")
                .arg("machdep.cpu.brand_string")
                .output()
            {
                let brand = String::from_utf8_lossy(&output.stdout);
                if brand.contains("Apple") {
                    return CPUVendor::Apple;
                }
            }
        }
        
        // Check for Rosetta2 when vendor is Intel on macOS
        if vendor_id == "GenuineIntel" && Self::is_rosetta2() {
            return CPUVendor::Rosetta2;
        }
        
        CPUVendor::from_vendor_id(vendor_id)
    }

    /// Helper function to detect if running under Apple Rosetta 2
    fn is_rosetta2() -> bool {
        if let Ok(output) = Command::new("sysctl")
            .arg("-n")
            .arg("sysctl.proc_translated")
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            return result.trim() == "1";
        }
        false
    }

    pub fn num_of_cores() -> u32 {
        sys_info::cpu_num().unwrap()
    }

    pub fn average_usage() -> f32 {
        sys_info::loadavg().unwrap().one as f32
    }

    fn get_architecture() -> CPUArchitecture {
        let output = Command::new("sh")
            .arg("-c")
            .arg("uname -m")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8_lossy(&output.stdout);
        match name.trim() {
            "i386" => CPUArchitecture::I386,
            "x86_64" => CPUArchitecture::X86_64,
            "arm64" => CPUArchitecture::Arm64,
            _ => CPUArchitecture::Unknown,
        }
    }
}
