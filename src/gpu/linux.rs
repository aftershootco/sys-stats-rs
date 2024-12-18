use crate::gpu::{GPUData, GPUUsage};

use adlx::helper::AdlxHelper;
use nvml_wrapper::Nvml;


impl GPUUsage {
    pub fn get_gpu_info() -> Result<GPUData, Box<dyn std::error::Error>> {

        // get gpu info on linux


        // if we are using nvidia gpu
        {

            let nvml = Nvml::init()?;
            // Get the first `Device` (GPU) in the system
            let device = nvml.device_by_index(0)?;

            let name = device.name()?; // GTX 1080 Ti on my system
            let architecture = device.architecture()?; // Pascal on my system
            let memory_info = device.memory_info()?; // Currently 1.63/6.37 GB used on my system

        }

        let mut result: GPUData = GPUData {
            name: "".to_string(),
            architecture: "".to_string(),
            has_unified_memory: false,
            total_memory: 0,
            used_memory: 0,
            free_memory: 0,
        };

       
        Ok(result)
    }

    pub fn get_gpus_list() -> Result<Vec<GPUData>, Box<dyn std::error::Error>> {
        let mut results: Vec<GPUData> = Vec::new();
        results.push(GPUUsage::get_gpu_info()?);
        Ok(results)
    }

    pub fn total_gpu_memory() -> Result<u64, Box<dyn std::error::Error>> {
        unsafe {
            Ok(0)
        }
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
        unsafe {
            Ok(false)
        }
    }

    fn current_cpu_memory_free() -> Result<u64, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.free * 1024) // convert to bytes
    }
}
