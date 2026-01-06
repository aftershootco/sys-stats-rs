use crate::cpu::{CPUData, CPUUsage, CpuFeatureSet};
use anyhow::Result;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use winapi::um::sysinfoapi::GetSystemInfo;
use winapi::um::sysinfoapi::SYSTEM_INFO;

use super::{CPUArchitecture, CPUVendor};

impl CPUUsage {
    pub fn get_cpu_info() -> Result<CPUData, String> {
        Ok(CPUData {
            name: Self::get_cpu_name(),
            vendor: Self::get_cpu_vendor(),
            architecture: Self::get_cpu_architecture(),
            num_of_cores: Self::num_of_cores(),
            logical_processors: Self::logical_processors(),
            average_cpu_usage: Self::average_usage(),
            instruction_sets: Self::get_instruction_sets(),
        })
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

    fn get_cpu_vendor() -> CPUVendor {
        let s = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        let vendor_id = s.cpus().get(0).unwrap().vendor_id();
        CPUVendor::from_vendor_id(vendor_id)
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

    fn get_instruction_sets() -> Vec<CpuFeatureSet> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            use strum::IntoEnumIterator;
            CpuFeatureSet::iter()
                .filter(|f| f.is_supported_x86())
                .collect()
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            Vec::new()
        }
    }
}
