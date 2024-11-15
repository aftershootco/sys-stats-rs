use objc2::msg_send;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};
use crate::gpu::{GPUData, GPUUsage};

// Import CoreGraphics as it's required for Metal on Intel macs
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

impl GPUUsage{

    pub fn get_gpu_info() -> Result<GPUData, String>{
        let mut result: GPUData = GPUData {
            name: "".to_string(),
            architecture: "".to_string(),
            has_unified_memory: false,
            total_memory: 0,
            used_memory: 0,
            free_memory: 0,
        };

        unsafe {
            let mtl_device =  {MTLCreateSystemDefaultDevice()};
            let mtl_device = mtl_device.as_ref().unwrap();
            result.name = mtl_device.name().to_string();
            result.architecture = mtl_device.architecture().name().to_string();

            // handling memory calculations separately, because apple does not provide a direct way to get the free/used gpu memory
            result.total_memory = Self::total_gpu_memory();
            result.used_memory = Self::current_gpu_memory_usage();
            result.free_memory = Self::current_gpu_memory_free();

            result.has_unified_memory = mtl_device.hasUnifiedMemory();
        }
        Ok(result)
    }
    

    pub fn get_gpus_list() -> Result<Vec<GPUData>, String> {
        let mut results: Vec<GPUData> = Vec::new();
        results.push(GPUUsage::get_gpu_info()?);
        Ok(results)
    }

    pub fn total_gpu_memory() -> u64 {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            let recommended_max_working_set_size: u64 = msg_send![mtl_device, recommendedMaxWorkingSetSize];
            recommended_max_working_set_size
        }
    }

    pub fn current_gpu_memory_usage() -> u64 {
        unsafe {
            // this approach is not accurate, but it's the only way to get the current allocated size
            // as apple does not provide a way to get the free/used gpu memory
            
            // let mtl_device = MTLCreateSystemDefaultDevice();
            // let current_allocated_size: u64 = msg_send![mtl_device, currentAllocatedSize];
            // current_allocated_size

            // rough estimate of the current used memory
            Self::total_gpu_memory() - Self::current_cpu_memory_free().unwrap() 
        }
    }

    pub fn current_gpu_memory_free() -> u64 {

        let mut free_memory: u64 = 0;

        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            let is_unified :bool =  msg_send![mtl_device, hasUnifiedMemory];

            // If the memory is unified, we can use the CPU memory to get the free memory
            // also apple does not provide a way to get the free/used gpu memory
            if is_unified {
                free_memory = (Self::current_cpu_memory_free().unwrap()); // convert to bytes
            }else {
                let mtl_device =  {MTLCreateSystemDefaultDevice()};
                let mtl_device = mtl_device.as_ref().unwrap();

                let total_memory = mtl_device.recommendedMaxWorkingSetSize();
                let used_memory = mtl_device.currentAllocatedSize() as u64;
                free_memory = total_memory - used_memory;
            }
        }
        free_memory
    }

    pub fn has_unified_memory() -> bool {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, hasUnifiedMemory]
        }
    }

    fn current_cpu_memory_free() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.free * 1024) // convert to bytes
    }
}