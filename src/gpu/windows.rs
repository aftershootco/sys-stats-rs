use crate::gpu::{DriverVersionData, GPUData, GPUUsage};

use anyhow::Result;
use nvml_wrapper::Nvml;
use windows::{
    Win32::Graphics::DXCore::*,
};

use windows::Win32::Graphics::DXCore::DXCoreHardwareID;
use windows::Win32::Graphics::DXCore::DedicatedAdapterMemory;
use windows::Win32::Graphics::DXCore::DriverDescription;
use windows::Win32::Graphics::DXCore::DriverVersion;
use windows::Win32::Graphics::DXCore::IDXCoreAdapter;
use windows::Win32::Graphics::DXCore::IDXCoreAdapterFactory;
use windows::Win32::Graphics::DXCore::IDXCoreAdapterList;
use windows::Win32::Graphics::DXCore::{
    HardwareID, IsHardware, IsIntegrated, DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE,
};


impl GPUUsage {

    /// Retrieves information about the primary GPU in the system.
    ///
    /// # Returns
    /// * `Ok(GPUData)` - A structure containing information about the GPU, we choose maximum memory gpu if there are multiple GPUs.
    /// * `Err(Box<dyn std::error::Error>)` - An error if no GPUs are found or if there's an issue retrieving the information.
    ///
    /// # Errors
    /// * Returns an error if no GPU is found in the system.
    pub fn get_gpu_info() -> Result<GPUData, Box<dyn std::error::Error>> {
        let gpus = Self::get_gpus_list()?;
        if gpus.len() == 0 {
            return Err("No GPU found".to_string().into());
        }

        Ok(gpus[0].clone())
    }

    /// Get list of all GPUs in the system, sorted by available memory
    pub fn get_gpus_list() -> std::result::Result<Vec<GPUData>, Box<dyn std::error::Error>> {

        let mut gpus_data: Vec<GPUData> = Vec::new();

        unsafe {
            let adapter_factory: IDXCoreAdapterFactory = DXCoreCreateAdapterFactory()?;

            let attributes = [DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE];
            let d3d12_core_compute_adapters: IDXCoreAdapterList =
                adapter_factory.CreateAdapterList(&attributes)?;


            let count = d3d12_core_compute_adapters.GetAdapterCount();

            println!("{:?}", count);

            for i in 0..count {
                let adapter: IDXCoreAdapter = d3d12_core_compute_adapters.GetAdapter(i)?;

                let mut is_hardware_buffer = [0u8; std::mem::size_of::<u32>()];

                adapter.GetProperty(
                    IsHardware,
                    std::mem::size_of::<u32>(),  // Specify the buffer size explicitly
                    is_hardware_buffer.as_mut_ptr() as *mut core::ffi::c_void
                )?;


                let is_hardware = u32::from_ne_bytes(is_hardware_buffer);
                let is_hardware = is_hardware != 0;

                if is_hardware {

                    // dedicated GPU Memory
                    let mut memory_size = [0u8; std::mem::size_of::<usize>()];
                    adapter.GetProperty(
                        DedicatedAdapterMemory,
                        std::mem::size_of::<usize>(),  // Specify the buffer size explicitly
                        memory_size.as_mut_ptr() as *mut core::ffi::c_void
                    )?;
                    let memory_size = usize::from_ne_bytes(memory_size);


                    // get AdapterMemoryBudget
                    let node_segment_group = DXCoreAdapterMemoryBudgetNodeSegmentGroup {
                        nodeIndex: 0,
                        segmentGroup: DXCoreSegmentGroup(0),
                    };

                    // Create the memory budget struct to receive the data
                    let mut memory_budget = DXCoreAdapterMemoryBudget::default();

                    // Query the state
                    adapter.QueryState(
                        AdapterMemoryBudget,
                        std::mem::size_of::<DXCoreAdapterMemoryBudgetNodeSegmentGroup>(),
                        Some(&node_segment_group as *const _ as *const core::ffi::c_void),
                        std::mem::size_of::<DXCoreAdapterMemoryBudget>(),
                        &mut memory_budget as *mut _ as *mut core::ffi::c_void
                    )?;

                    // Is this Integrated GPU?
                    let mut integrated_buffer = [0u8; std::mem::size_of::<u32>()];

                    adapter.GetProperty(
                        IsIntegrated,
                        std::mem::size_of::<u32>(),  // Specify the buffer size explicitly
                        integrated_buffer.as_mut_ptr() as *mut core::ffi::c_void
                    )?;

                    let is_integrated = u32::from_ne_bytes(integrated_buffer);
                    let is_integrated = is_integrated != 0;


                    // get hardware id
                    let mut hardware_id_buffer = [0u8; std::mem::size_of::<DXCoreHardwareID>()];

                    adapter.GetProperty(
                        HardwareID,
                        std::mem::size_of::<DXCoreHardwareID>(),
                        hardware_id_buffer.as_mut_ptr() as *mut core::ffi::c_void
                    )?;
                    // Convert the raw buffer to DXCoreHardwareID struct
                    let hardware_id: DXCoreHardwareID =  std::ptr::read(hardware_id_buffer.as_ptr() as *const _);


                    // Get the description size
                    let desc_size = adapter.GetPropertySize(DriverDescription)?;
                    let mut desc_buffer = vec![0u8; desc_size];

                    adapter.GetProperty(
                        DriverDescription,
                        desc_size,
                        desc_buffer.as_mut_ptr() as *mut core::ffi::c_void
                    )?;

                    let gpu_name = String::from_utf8_lossy(&desc_buffer)
                        .trim_end_matches('\0')
                        .to_string();


                    // Get driver version
                    let version_size = adapter.GetPropertySize(DriverVersion)?;
                    let mut version_buffer = vec![0u8; version_size];

                    adapter.GetProperty(
                        DriverVersion,
                        version_size,
                        version_buffer.as_mut_ptr() as *mut core::ffi::c_void
                    )?;

                    let version = u64::from_ne_bytes(version_buffer.try_into().unwrap());

                    // Windows format
                    let major = (version >> 48) & 0xFFFF;
                    let minor = (version >> 32) & 0xFFFF;
                    let build = (version >> 16) & 0xFFFF;
                    let revision = version & 0xFFFF;

                    let mut current_gpu_data = GPUData::new();
                    
                    current_gpu_data.vendor_id = hardware_id.vendorID;
                    current_gpu_data.has_unified_memory = is_integrated;
                    current_gpu_data.is_integrated = is_integrated;
                    current_gpu_data.adapter_index = i;
                    current_gpu_data.driver_version = DriverVersionData{
                        major,
                        minor,
                        build,
                        revision,
                    };

                    // if nvidia
                    if hardware_id.vendorID == 0x10DE {
                        let nvml = Nvml::init()?;
                        let nv_gpu_count = nvml.device_count()?;

                        if nv_gpu_count > 0 {
                            let device = nvml.device_by_index(0)?;
                            let memory_info = device.memory_info()?;

                            current_gpu_data.name = device.name()?;
                            current_gpu_data.architecture = device.architecture()?.to_string();
                            current_gpu_data.total_memory = memory_info.total;
                            current_gpu_data.free_memory = memory_info.free;
                            current_gpu_data.used_memory = memory_info.used;
                        }
                    }else {

                        if hardware_id.vendorID == 0x1002{
                            current_gpu_data.architecture = "Radeon".to_string();
                        } else if hardware_id.vendorID == 0x8086{
                            current_gpu_data.architecture = "Intel".to_string();
                        }else if hardware_id.vendorID == 0x14E4{
                            current_gpu_data.architecture = "Qualcomm".to_string();
                        }

                        current_gpu_data.name = gpu_name;
                        current_gpu_data.total_memory = memory_size as u64;
                        current_gpu_data.free_memory = memory_budget.availableForReservation;
                        current_gpu_data.used_memory = memory_budget.budget - memory_budget.availableForReservation;
                    }

                    if current_gpu_data.total_memory > 0 {
                        gpus_data.push(current_gpu_data);
                    }

                }
            }
        }

        gpus_data.sort_by(|a, b| b.total_memory.cmp(&a.total_memory));
        Ok(gpus_data)
    }


    // Get the total gpu memory of the system
    pub fn total_gpu_memory(adapter_index: u32) -> u64 {
        Self::get_gpus_list()
            .map(|gpus| gpus
                .iter()
                .find(|gpu| gpu.adapter_index == adapter_index)
                .map_or(0, |gpu| gpu.total_memory))
            .unwrap_or(0)
    }

    // Get the allocated gpu memory
    pub fn gpu_memory_usage(adapter_index: u32) -> u64 {
        Self::get_gpus_list()
            .map(|gpus| gpus
                .iter()
                .find(|gpu| gpu.adapter_index == adapter_index)
                .map_or(0, |gpu| gpu.used_memory))
            .unwrap_or(0)
    }


    pub fn gpu_memory_free(adapter_index: u32) -> u64 {
        Self::get_gpus_list()
            .map(|gpus| gpus
                .iter()
                .find(|gpu| gpu.adapter_index == adapter_index)
                .map_or(0, |gpu| gpu.free_memory))
            .unwrap_or(0)
    }

    pub fn has_unified_memory(adapter_index: u32) -> bool {
        Self::get_gpus_list()
            .map(|gpus| gpus
                .iter()
                .find(|gpu| gpu.adapter_index == adapter_index)
                .map_or(false, |gpu| gpu.has_unified_memory))
            .unwrap_or(false)
    }
}