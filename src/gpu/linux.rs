use crate::gpu::{AdapterData, DriverVersionData, GPUData, GPUUsage};
use std::process::Command;

use nvml_wrapper::Nvml;

impl GPUUsage {
    pub fn get_gpu_info() -> Result<GPUData, Box<dyn std::error::Error>> {
        // check avaliable gpus using lspci command

        let mut result: GPUData = GPUData {
            name: "".to_string(),
            architecture: "".to_string(),
            vendor_id: 0,
            total_memory: 0,
            free_memory: 0,
            used_memory: 0,
            has_unified_memory: false,
            is_integrated: false,
            adapter_index: 0,
            driver_version: DriverVersionData {
                major: 0,
                minor: 0,
                build: 0,
                revision: 0,
            },
        };

        let gpus: Vec<(String, String)> = dbg!(Self::get_gpu_from_lspci());

        gpus.iter().for_each(|gpu| {
            if gpu.1.contains("NVIDIA") || gpu.1.contains("nvidia") || gpu.1.contains("Nvidia") {
                result = Self::get_nvidia_details().unwrap();
            } else if gpu.1.contains("AMD") || gpu.1.contains("amd") || gpu.1.contains("AMD") {
                println!("AMD GPU found");
            } else if gpu.1.contains("Intel") || gpu.1.contains("intel") || gpu.1.contains("INTEL")
            {
                result.name = gpu.1.clone();
            }
        });

        Ok(result)
    }

    pub fn get_all_adapters_list() -> Result<Vec<AdapterData>, Box<dyn std::error::Error>> {
        println!("get_all_adapters_list not implemented for Linux");
        Ok(Vec::new())
    }

    pub fn get_gpus_list() -> Result<Vec<GPUData>, Box<dyn std::error::Error>> {
        let mut results: Vec<GPUData> = Vec::new();
        results.push(GPUUsage::get_gpu_info()?);
        Ok(results)
    }

    pub fn total_gpu_memory() -> Result<u64, Box<dyn std::error::Error>> {
        unsafe { Ok(0) }
    }

    pub fn current_gpu_memory_usage() -> Result<u64, Box<dyn std::error::Error>> {
        unsafe {
            // this approach is not accurate, but it's the only way to get the current allocated size
            // as apple does not provide a way to get the free/used gpu memory
            // rough estimate of the current used memory

            let total = Self::total_gpu_memory()?;
            let free = Self::current_gpu_memory_free()?;

            if total < free {
                eprintln!("Free can not be more than total");
                return Ok(0);
            }

            Ok(total - free)
        }
    }

    pub fn current_gpu_memory_free() -> Result<u64, Box<dyn std::error::Error>> {
        let free_memory: u64 = Self::current_cpu_memory_free()?;
        Ok(free_memory)
    }

    pub fn has_unified_memory() -> Result<bool, Box<dyn std::error::Error>> {
        unsafe { Ok(false) }
    }

    fn current_cpu_memory_free() -> Result<u64, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.free * 1024) // convert to bytes
    }

    fn get_nvidia_details() -> Result<GPUData, Box<dyn std::error::Error>> {
        let mut ret: GPUData = GPUData {
            name: "".to_string(),
            architecture: "".to_string(),
            has_unified_memory: false,
            total_memory: 0,
            used_memory: 0,
            free_memory: 0,
            adapter_index: 0,
            vendor_id: 0,
            is_integrated: false,
            driver_version: DriverVersionData {
                major: 0,
                minor: 0,
                build: 0,
                revision: 0,
            },
        };

        let nvml = Nvml::init()?;

        let version_info = nvml
            .sys_driver_version()?
            .split('.')
            .map(|x| x.parse::<u64>().expect("Cannot parse version info"))
            .take(2)
            .collect::<Vec<u64>>();

        let device = nvml.device_by_index(0)?;

        ret.name = device.name()?;
        ret.architecture = device.architecture().unwrap().to_string();
        ret.has_unified_memory = false;
        ret.total_memory = device.memory_info()?.total;
        ret.used_memory = device.memory_info()?.used;
        ret.free_memory = device.memory_info()?.free;
        ret.driver_version.major = version_info[0];
        ret.driver_version.minor = version_info[1];

        Ok(ret)
    }

    fn get_gpu_from_lspci() -> Vec<(String, String)> {
        let mut gpus: Vec<(String, String)> = Vec::new();

        let output = Command::new("sh")
            .arg("-c")
            .arg("lspci | grep -i VGA ")
            .output()
            .expect("Failed to execute command");

        let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 3 {
                continue;
            }

            let mut name = String::new();
            for i in 2..parts.len() {
                name.push_str(parts[i]);
                name.push_str(" ");
            }

            gpus.push((parts[1].to_string(), name.trim().to_string()));
        }

        gpus
    }
}
