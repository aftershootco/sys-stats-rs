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

        // println!("dwLength {:?}", memory_status.dwLength);
        // println!("dwMemoryLoad {:?}", memory_status.dwMemoryLoad);
        // println!("ullTotalPhys {:?} MB", memory_status.ullTotalPhys / 1024 / 1024);
        // println!("ullAvailPhys {:?} MB", memory_status.ullAvailPhys / 1024 / 1024);
        // println!("ullTotalPageFile {:?} MB", memory_status.ullTotalPageFile / 1024 / 1024);
        // println!("ullAvailPageFile {:?} MB", memory_status.ullAvailPageFile / 1024 / 1024);
        // println!("ullTotalVirtual {:?} MB", memory_status.ullTotalVirtual / 1024 / 1024);
        // println!("ullAvailVirtual {:?} MB", memory_status.ullAvailVirtual / 1024 / 1024);
        //println!("{:?}", memory_status.ullAvailExtendedVirtual);


        memory_status.ullTotalPhys

    }

    // Get the available gpu memory of the system
    pub fn max_gpu_memory() -> u64 {
        let mut memory_status: MEMORYSTATUSEX = unsafe { zeroed() };
        memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        unsafe {
            GlobalMemoryStatusEx(&mut memory_status);
        }
        memory_status.ullAvailPhys
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

    pub fn has_unified_memory() -> bool {
        false
    }
}