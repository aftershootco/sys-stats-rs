use winapi::um::sysinfoapi::{ MEMORYSTATUSEX};
use winapi::shared::winerror::FAILED;
use std::ptr;
use nvml_wrapper::Nvml;

use winapi::shared::dxgi::*;
use crate::GPUInfo;

pub struct WindowsMemoryUsage;


impl WindowsMemoryUsage {

    fn get_gpu_info() -> Result<GPUInfo, String> {
        
        let mut result =GPUInfo {
            name: "".to_string(),
            architecture: "".to_string(),
            total_video_memory: 0,
            used_video_memory: 0,
            free_video_memory: 0,
        };
        
        unsafe {

            let nvml = Nvml::init().expect("Failed to initialize NVML");
            let nv_gpu_count = nvml.device_count().expect("Failed to get device count");

            // if we have nvidia gpu
            if  nv_gpu_count > 0 {
                let device = nvml.device_by_index(0).expect("Failed to get device");
            
                let memory_info = device.memory_info().expect("Failed to get memory info");

                result.name = device.brand()?;
                result.architecture = device.board_id()?;
                result.total_video_memory = memory_info.total;
                result.used_video_memory = memory_info.used;
                result.free_video_memory = memory_info.free;

                // TODO: check if the system has unified memory
                result.has_unified_memory = false;
            
            }
            else {
                let mut factory: *mut IDXGIFactory1 = ptr::null_mut();
                let hr = CreateDXGIFactory1(&IID_IDXGIFactory1, &mut factory as *mut *mut _ as *mut *mut _);
                if FAILED(hr) {
                    return Err("Failed to create DXGIFactory".to_string());
                }
                let mut adapter: *mut IDXGIAdapter = ptr::null_mut();
                let hr = (*factory).EnumAdapters(0, &mut adapter);
                if FAILED(hr) {
                    return Err("Failed to enumerate adapters".to_string());
                }
                
    
                let mut desc: DXGI_ADAPTER_DESC = std::mem::zeroed();
                let hr = (*adapter).GetDesc(&mut desc);
                if FAILED(hr) {
                    return Err("Failed to get adapter description".to_string());
                }

                result.total_video_memory = desc.SharedSystemMemory as u64;
                result.used_video_memory = desc.DedicatedVideoMemory as u64;
                result.free_video_memory = desc.SharedSystemMemory as u64 - result.used_video_memory;
    
            }

            Ok(result)
        }
    }


    // Get the total gpu memory of the system
    pub fn total_gpu_memory() -> u64 {
        let gpu_mem_info = Self::get_gpu_memory_info().unwrap();
        return gpu_mem_info.total_video_memory;
       
    }

    // Get the current allocated gpu memory
    pub fn current_gpu_memory_usage() -> u64 {
        let gpu_mem_info = Self::get_gpu_memory_info().unwrap();
        gpu_mem_info.used_video_memory
    }

    pub fn current_gpu_memory_free() -> u64 {
        let gpu_mem_info = Self::get_gpu_memory_info().unwrap();
        gpu_mem_info.free_video_memory
    }

    pub fn has_unified_memory() -> bool {
        false
    }

    pub fn total_cpu_memory() -> Result<u64, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.total / 1024) // MB
    }

    pub fn current_cpu_memory_usage() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.total - mem_info.free) / 1024) // MB
    }

    pub fn current_cpu_memory_free() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.free) / 1024) // MB
    }

    pub fn current_cpu_memory_swap() -> Result<(u64, u64), Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.swap_total / 1024, mem_info.swap_free / 1024)) // MB
    }
}