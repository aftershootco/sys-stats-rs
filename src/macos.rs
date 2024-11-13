use objc2::{msg_send};
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};
use crate::GPUInfo;

// Import CoreGraphics as it's required for Metal on Intel macs
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

pub struct MacMemoryUsage;

impl MacMemoryUsage {

    pub fn get_gpu_info() -> Result<GPUInfo, String>{
        let mut result = GPUInfo::new();
        unsafe {
            let mtl_device =  {MTLCreateSystemDefaultDevice()};
            let mtl_device = mtl_device.as_ref().unwrap();
            result.name = mtl_device.name().to_string();
            result.architecture = mtl_device.architecture().name().to_string();
            result.total_memory = mtl_device.recommendedMaxWorkingSetSize() / 1024 / 1024;
            result.used_memory = (mtl_device.currentAllocatedSize() as u64) / 1024 / 1024;
            result.free_memory = (result.total_memory - result.used_memory);
            result.has_unified_memory = mtl_device.hasUnifiedMemory();
        }
        Ok(result)
    }

    pub fn total_gpu_memory() -> u64 {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            let mut recommended_max_working_set_size: u64 = msg_send![mtl_device, recommendedMaxWorkingSetSize];
            recommended_max_working_set_size =  recommended_max_working_set_size / 1024 / 1024;
            recommended_max_working_set_size
        }
    }

    pub fn current_gpu_memory_usage() -> u64 {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            let mut current_allocated_size: u64 = msg_send![mtl_device, currentAllocatedSize];
            current_allocated_size = current_allocated_size / 1024 / 1024;
            current_allocated_size
        }
    }

    pub fn current_gpu_memory_free() -> u64 {

        let mut free_memory = 0;

        unsafe {
            let mtl_device =  {MTLCreateSystemDefaultDevice()};
            let mtl_device = mtl_device.as_ref().unwrap();

            let total_memory = mtl_device.recommendedMaxWorkingSetSize() / 1024 / 1024;
            let used_memory = (mtl_device.currentAllocatedSize() as u64) / 1024 / 1024;
            free_memory = (total_memory - used_memory);
        }
        free_memory
    }

    pub fn has_unified_memory() -> bool {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, hasUnifiedMemory]
        }
    }

    pub fn total_cpu_memory() -> Result<u64, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.total / 1024 )
    }

    pub fn current_cpu_memory_usage() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.total - mem_info.avail) / 1024) // Convert from KB to MB
    }

    pub fn current_cpu_memory_free() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.avail) / 1024) // Convert from KB to MB
    }

    pub fn current_cpu_memory_swap() -> Result<(u64, u64), Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.swap_total / 1024, mem_info.swap_free / 1024)) // Convert from KB to MB
    }
}