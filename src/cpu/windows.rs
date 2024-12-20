use crate::cpu::{CPUData, CPUUsage};
use anyhow::Result;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use winapi::um::sysinfoapi::GetSystemInfo;
use winapi::um::sysinfoapi::SYSTEM_INFO;

use super::CPUArchitecture;

impl CPUUsage {
    pub fn get_cpu_info() -> Result<CPUData, String> {
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }

        let mut cpu_data = CPUData {
            name: Self::get_cpu_name(),
            architecture: Self::get_cpu_architecture(),
            num_of_cores: 0,
            average_cpu_usage: 0.0,
        };
        cpu_data.num_of_cores = Self::num_of_cores();
        cpu_data.average_cpu_usage = Self::average_usage();
        Ok(cpu_data)
    }

    pub fn num_of_cores() -> u32 {
        let s =
            System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing()));
        s.cpus().len() as u32
    }

    pub fn average_usage() -> f32 {
        sys_info::loadavg().unwrap().one as f32
    }

    fn get_cpu_name() -> String {
        let s =
            System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing()));
        let cpu = s.cpus().get(0);

        if cpu.is_none() {
            "".to_string()
        } else {
            cpu.unwrap().brand().to_string()
        }
    }

    fn get_cpu_architecture() -> CPUArchitecture {
        // using winapi to get the architecture
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }
        match unsafe { sys_info.u.s().wProcessorArchitecture } {
            0 => CPUArchitecture::I386,
            5 => CPUArchitecture::Arm,
            9 => CPUArchitecture::X86_64,
            12 => CPUArchitecture::Arm64,
            14 => CPUArchitecture::RiscV64,
            _ => CPUArchitecture::Unknown,
        }
    }
}
