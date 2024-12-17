use crate::cpu::{CPUData, CPUUsage};
use std::process::Command;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

use super::CPUArchitecture;

impl CPUUsage {
    pub fn get_cpu_info() -> Result<CPUData, Box<dyn std::error::Error>> {
        Ok(CPUData {
            name: Self::get_name(),
            architecture: Self::get_architecture(),
            num_of_cores: Self::num_of_cores(),
            average_cpu_usage: Self::average_usage(),
        })
    }

    fn get_name() -> String {
        let s = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        s.cpus().iter().next().unwrap().name().to_string()
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
