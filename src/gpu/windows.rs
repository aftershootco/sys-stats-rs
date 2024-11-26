use crate::gpu::{GPUData, GPUUsage};

use adlx::helper::AdlxHelper;
use anyhow::Result;
use nvml_wrapper::Nvml;
use std::ptr;
use winapi::shared::winerror::FAILED;

use winapi::shared::dxgi::*;

impl GPUUsage {
    // get the list of gpus in the system, using the windows api
    fn get_dxgi_list() -> Vec<DXGI_ADAPTER_DESC> {
        let mut desc_list: Vec<DXGI_ADAPTER_DESC> = vec![];

        unsafe {
            let mut factory: *mut IDXGIFactory1 = ptr::null_mut();
            let hr = CreateDXGIFactory1(
                &IID_IDXGIFactory1,
                &mut factory as *mut *mut _ as *mut *mut _,
            );
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

    pub fn get_gpu_info() -> Result<GPUData, String> {
        let gpus = Self::get_gpus_list().unwrap();
        if gpus.len() == 0 {
            return Err("No GPU found".to_string());
        }

        Ok(gpus[0].clone())
    }

    pub fn get_gpus_list() -> Result<Vec<GPUData>, String> {
        let mut results = vec![];
        let gpu_desc_list = Self::get_dxgi_list();

        // vendor id nvidia : 4318
        // vendor id amd : 4098
        // vendor id intel : 32902
        // vendor id qualcomm : 23170

        // if we have nvidia gpu
        if gpu_desc_list.iter().any(|x| x.VendorId == 4318) {
            let nvml = Nvml::init().expect("Failed to initialize NVML");
            let nv_gpu_count = nvml.device_count().expect("Failed to get device count");

            if nv_gpu_count > 0 {
                let device = nvml.device_by_index(0).expect("Failed to get device");

                let memory_info = device.memory_info().expect("Failed to get memory info");

                let result = GPUData::new_with_values(
                    device.name().unwrap_or("Unknown".to_string()),
                    device.architecture().unwrap().to_string(),
                    memory_info.total,
                    memory_info.free,
                    memory_info.used,
                    false,
                );

                results.push(result);
            }
        }

        if gpu_desc_list.iter().any(|x| x.VendorId == 4098) {
            // if we have amd gpu
            // use adlx to get the gpu info
            let adlx_helper = AdlxHelper::new().unwrap();
            let gpus = adlx_helper.system().gpus().unwrap();
            let pms = adlx_helper
                .system()
                .performance_monitoring_services()
                .unwrap();

            let gpu1 = gpus.at(0).unwrap();

            let result = GPUData::new_with_values(
                gpu1.name().unwrap().to_string(),
                gpu1.asic_family_type().unwrap().to_string(),
                gpu1.total_vram().unwrap() as u64,
                pms.current_gpu_metrics(&gpu1).unwrap().vram().unwrap() as u64,
                gpu1.total_vram().unwrap() as u64
                    - pms.current_gpu_metrics(&gpu1).unwrap().vram().unwrap() as u64,
                gpu1.type_().unwrap() == 1,
            );

            results.push(result);
        }

        if gpu_desc_list.iter().any(|x| x.VendorId == 32902) {
            // if we have intel gpu
            // todo: get the correct Data using intel api
            let desc = gpu_desc_list.iter().find(|x| x.VendorId == 32902).unwrap();

            let result = GPUData::new_with_values(
                "Intel".to_string(),
                "Integrated or Arc".to_string(),
                (desc.SharedSystemMemory) as u64,
                (desc.DedicatedVideoMemory) as u64,
                (desc.SharedSystemMemory) as u64 - (desc.DedicatedVideoMemory) as u64,
                true,
            );

            results.push(result);
        }

        if gpu_desc_list.iter().any(|x| x.VendorId == 23170) {
            // if we have qualcomm gpu
            let desc = gpu_desc_list.iter().find(|x| x.VendorId == 23170).unwrap();

            let result = GPUData::new_with_values(
                "Qualcomm".to_string(),
                "Adreno".to_string(),
                (desc.SharedSystemMemory) as u64,
                (desc.DedicatedVideoMemory) as u64,
                (desc.SharedSystemMemory) as u64 - (desc.DedicatedVideoMemory) as u64,
                true,
            );

            results.push(result);
        }
        Ok(results)
    }

    // Get the total gpu memory of the system
    pub fn total_gpu_memory() -> u64 {
        if let Ok(gpu_info) = Self::get_gpu_info() {
            return gpu_info.total_memory;
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
        if let Ok(gpu_info) = Self::get_gpu_info() {
            return gpu_info.has_unified_memory;
        }
        false
    }
}
