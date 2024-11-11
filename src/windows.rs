use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, LPMEMORYSTATUSEX};
use std::mem::zeroed;

pub struct WindowsMemoryUsage;

impl WindowsMemoryUsage {
    // Get the total gpu memory of the system
    pub fn total_gpu_memory() -> u64 {
        let mut memory_status: LPMEMORYSTATUSEX = unsafe { zeroed() };
        memory_status.dwLength = std::mem::size_of::<LPMEMORYSTATUSEX>() as u32;
        unsafe {
            GlobalMemoryStatusEx(&mut memory_status);
        }
        memory_status.ullTotalPhys
    }

    // Get the available gpu memory of the system
    pub fn max_gpu_memory() -> u64 {
        let mut memory_status: LPMEMORYSTATUSEX = unsafe { zeroed() };
        memory_status.dwLength = std::mem::size_of::<LPMEMORYSTATUSEX>() as u32;
        unsafe {
            GlobalMemoryStatusEx(&mut memory_status);
        }
        memory_status.ullAvailPhys
    }

    // Get the current allocated gpu memory
    pub fn current_gpu_memory_usage() -> u64 {
        let mut memory_status: LPMEMORYSTATUSEX = unsafe { zeroed() };
        memory_status.dwLength = std::mem::size_of::<LPMEMORYSTATUSEX>() as u32;
        unsafe {
            GlobalMemoryStatusEx(&mut memory_status);
        }
        memory_status.ullTotalPhys - memory_status.ullAvailPhys
    }

    pub fn has_unified_memory() -> bool {
        false
    }
}