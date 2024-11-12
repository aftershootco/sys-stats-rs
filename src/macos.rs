use objc2::{msg_send};
use objc2_metal::{MTLCreateSystemDefaultDevice};

pub struct MacMemoryUsage;

impl MacMemoryUsage {
    pub fn total_gpu_memory() -> u64 {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, recommendedMaxWorkingSetSize]
        }
    }

    pub fn current_gpu_memory_usage() -> u64 {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, currentAllocatedSize]
        }
    }

    pub fn current_gpu_memory_free() -> u64 {
        Self::total_gpu_memory() - Self::current_gpu_memory_usage()
    }

    pub fn has_unified_memory() -> bool {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, hasUnifiedMemory]
        }
    }

    pub fn total_cpu_memory() -> Result<u64, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.total * 1024) // Convert from KB to bytes
    }

    pub fn current_cpu_memory_usage() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.total - mem_info.avail) * 1024) // Convert from KB to bytes
    }

    pub fn current_cpu_memory_free() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.free) * 1024) // Convert from KB to bytes
    }

    pub fn current_cpu_memory_swap() -> Result<(u64, u64), Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.swap_total * 1024, mem_info.swap_free * 1024)) // Convert from KB to bytes
    }
}