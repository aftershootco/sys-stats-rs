use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use std::mem::zeroed;

pub struct WindowsMemoryUsage;

impl WindowsMemoryUsage {
    // Get the total gpu memory of the system
    pub fn total_gpu_memory() -> u64 {
        let mut memory_status: MEMORYSTATUSEX = unsafe { zeroed() };
        memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        unsafe {
            GlobalMemoryStatusEx(&mut memory_status);
        }
        memory_status.ullTotalPhys
    }

    // Get the current allocated gpu memory
    pub fn current_gpu_memory_usage() -> u64 {
        let mut memory_status: MEMORYSTATUSEX = unsafe { zeroed() };
        memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        unsafe {
            GlobalMemoryStatusEx(&mut memory_status);
        }
        memory_status.ullTotalPhys - memory_status.ullAvailPhys
    }

    pub fn current_gpu_memory_free() -> u64 {
        total_gpu_memory() - current_gpu_memory_usage()
    }

    pub fn has_unified_memory() -> bool {
        false
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