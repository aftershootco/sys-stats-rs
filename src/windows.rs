use winapi::shared::winerror::FAILED;
use std::ptr;
use nvml_wrapper::Nvml;

use winapi::shared::dxgi::*;
use crate::GPUInfo;

pub struct WindowsMemoryUsage;


impl WindowsMemoryUsage {

    pub fn get_gpu_info() -> Result<GPUInfo, String> {
        
        let mut result =GPUInfo {
            name: "".to_string(),
            architecture: "".to_string(),
            has_unified_memory: false,
            total_memory: 0,
            used_memory: 0,
            free_memory: 0,
        };

        // vendor id nvidia : 4318
        // vendor id amd : 4098
        // vendor id intel : 32902
        // vendor id qualcomm : 23170

        let gpu_desc_list = Self::get_gpu_list();

        // if we have nvidia gpu
        if gpu_desc_list.iter().any(|x| x.VendorId == 4318) {
            let nvml = Nvml::init().expect("Failed to initialize NVML");
            let nv_gpu_count = nvml.device_count().expect("Failed to get device count");

            if nv_gpu_count > 0 {
                let device = nvml.device_by_index(0).expect("Failed to get device");
            
                let memory_info = device.memory_info().expect("Failed to get memory info");

                result.name = device.name().unwrap_or("Unknown".to_string());
                result.architecture = device.architecture().unwrap().to_string();
                result.total_memory = memory_info.total / 1024 / 1024;
                result.used_memory = memory_info.used / 1024 / 1024;
                result.free_memory = memory_info.free / 1024 / 1024;
                result.has_unified_memory = false;
            }
        }
        else if gpu_desc_list.iter().any(|x| x.VendorId == 4098) { // if we have amd gpu
            // todo: get the correct Data using amd api
            let desc = gpu_desc_list.iter().find(|x| x.VendorId == 4098).unwrap();
            result.name = "AMD".to_string();
            result.architecture = "Radeon".to_string();
            result.total_memory =(desc.SharedSystemMemory / 1024 / 1024) as u64;
            result.used_memory = (desc.DedicatedVideoMemory / 1024 / 1024) as u64;
            result.free_memory = result.total_memory - result.used_memory;
            result.has_unified_memory = false;
        } else if gpu_desc_list.iter().any(|x| x.VendorId == 32902) {
            // if we have intel gpu
            // todo: get the correct Data using intel api
            let desc = gpu_desc_list.iter().find(|x| x.VendorId == 32902).unwrap();
            result.name = "Intel".to_string();
            result.architecture = "Integrated or Arc".to_string();
            result.total_memory = (desc.SharedSystemMemory / 1024 /1024) as u64;
            result.used_memory = (desc.DedicatedVideoMemory / 1024 / 1024) as u64;
            result.free_memory = result.total_memory - result.used_memory;
            result.has_unified_memory = true;
        } else if gpu_desc_list.iter().any(|x| x.VendorId == 23170) {
            // if we have qualcomm gpu
            let desc = gpu_desc_list.iter().find(|x| x.VendorId == 23170).unwrap();
            result.name = "Qualcomm".to_string();
            result.architecture = "Adreno".to_string();
            result.total_memory = (desc.SharedSystemMemory / 1024 / 1024) as u64;
            result.used_memory = (desc.DedicatedVideoMemory / 1024 / 1024) as u64;
            result.free_memory = result.total_memory - result.used_memory;
            result.has_unified_memory = true;
        }
            
        Ok(result)

    }
    

    fn get_gpu_list() -> Vec<DXGI_ADAPTER_DESC> {
        
        let mut desc_list: Vec<DXGI_ADAPTER_DESC> = vec![];

        unsafe {
            let mut factory: *mut IDXGIFactory1 = ptr::null_mut();
            let hr = CreateDXGIFactory1(&IID_IDXGIFactory1, &mut factory as *mut *mut _ as *mut *mut _);
            if FAILED(hr) {
                return desc_list;
            }
            let mut i = 0;
            loop {
                let mut adapter: *mut IDXGIAdapter = ptr::null_mut();
                let hr = (*factory).EnumAdapters(i, &mut adapter);
                if FAILED(hr) {
                    break;
                }
                let mut desc: DXGI_ADAPTER_DESC = std::mem::zeroed();
                let hr = (*adapter).GetDesc(&mut desc);
                if FAILED(hr) {
                    break;
                }
    
                desc_list.push(desc);
                i += 1;
            }
        }

        desc_list
    }


    // Get the total gpu memory of the system
    pub fn total_gpu_memory() -> u64 {
        if let Ok(gpu_info) = Self::get_gpu_info() {
            return gpu_info.total_memory
        }
        0
    }

    // Get the current allocated gpu memory
    pub fn current_gpu_memory_usage() -> u64 {
        if let Ok(gpu_info) = Self::get_gpu_info() {
            return gpu_info.used_memory;
        }
        0
    }

    pub fn current_gpu_memory_free() -> u64 {
        if let Ok(gpu_info) = Self::get_gpu_info() {
            return gpu_info.free_memory;
        }
        0
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