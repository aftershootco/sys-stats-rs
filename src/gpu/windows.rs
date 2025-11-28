use crate::gpu::{AdapterData, DriverVersionData, GPUData, GPUUsage};

use adlx::ffi::{ADLX_GPU_TYPE_GPUTYPE_DISCRETE, ADLX_GPU_TYPE_GPUTYPE_INTEGRATED};
use adlx::helper::AdlxHelper;
use anyhow::Result;
use nvml_wrapper::Nvml;
use windows::Win32::Graphics::DXCore::*;

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

    /// Get list of all adapters in the system (hardware and non-hardware)
    pub fn get_all_adapters_list(
    ) -> std::result::Result<Vec<AdapterData>, Box<dyn std::error::Error>> {
        let mut adapters_data: Vec<AdapterData> = Vec::new();

        unsafe {
            let adapter_factory: IDXCoreAdapterFactory = DXCoreCreateAdapterFactory()?;

            let attributes = [DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE];
            let d3d12_core_compute_adapters: IDXCoreAdapterList =
                adapter_factory.CreateAdapterList(&attributes)?;

            let count = d3d12_core_compute_adapters.GetAdapterCount();

            for i in 0..count {
                let adapter: IDXCoreAdapter = d3d12_core_compute_adapters.GetAdapter(i)?;

                let mut is_hardware_buffer = [0u8; std::mem::size_of::<u32>()];

                adapter.GetProperty(
                    IsHardware,
                    std::mem::size_of::<u32>(),
                    is_hardware_buffer.as_mut_ptr() as *mut core::ffi::c_void,
                )?;

                let is_hardware = u32::from_ne_bytes(is_hardware_buffer);
                let is_hardware = is_hardware != 0;

                // Get the description size
                let desc_size = adapter.GetPropertySize(DriverDescription)?;
                let mut desc_buffer = vec![0u8; desc_size];

                adapter.GetProperty(
                    DriverDescription,
                    desc_size,
                    desc_buffer.as_mut_ptr() as *mut core::ffi::c_void,
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
                    version_buffer.as_mut_ptr() as *mut core::ffi::c_void,
                )?;

                let version = u64::from_ne_bytes(version_buffer.try_into().unwrap());

                // Windows format
                let major = (version >> 48) & 0xFFFF;
                let minor = (version >> 32) & 0xFFFF;
                let build = (version >> 16) & 0xFFFF;
                let revision = version & 0xFFFF;

                // get hardware id
                let mut hardware_id_buffer = [0u8; std::mem::size_of::<DXCoreHardwareID>()];

                adapter.GetProperty(
                    HardwareID,
                    std::mem::size_of::<DXCoreHardwareID>(),
                    hardware_id_buffer.as_mut_ptr() as *mut core::ffi::c_void,
                )?;

                let hardware_id: DXCoreHardwareID =
                    std::ptr::read(hardware_id_buffer.as_ptr() as *const _);

                let is_integrated = is_integrated_gpu(&adapter)?;

                let mut current_adapter_data = AdapterData::new();

                current_adapter_data.name = gpu_name;
                current_adapter_data.vendor_id = hardware_id.vendorID;
                current_adapter_data.device_id = hardware_id.deviceID;
                current_adapter_data.is_hardware = is_hardware;
                current_adapter_data.is_integrated = is_integrated;
                current_adapter_data.adapter_index = i;
                current_adapter_data.driver_version = DriverVersionData {
                    major,
                    minor,
                    build,
                    revision,
                };

                // Set architecture based on vendor ID
                if hardware_id.vendorID == 0x10DE {
                    current_adapter_data.architecture = "NVIDIA".to_string();
                } else if hardware_id.vendorID == 0x1002 {
                    current_adapter_data.architecture = "AMD".to_string();
                } else if hardware_id.vendorID == 0x8086 {
                    current_adapter_data.architecture = "Intel".to_string();
                } else if hardware_id.vendorID == 0x14E4 {
                    current_adapter_data.architecture = "Qualcomm".to_string();
                } else {
                    current_adapter_data.architecture = "Unknown".to_string();
                }

                // Get memory info if it's hardware
                if is_hardware {
                    let mut memory_size = [0u8; std::mem::size_of::<usize>()];
                    if let Ok(_) = adapter.GetProperty(
                        DedicatedAdapterMemory,
                        std::mem::size_of::<usize>(),
                        memory_size.as_mut_ptr() as *mut core::ffi::c_void,
                    ) {
                        let memory_size = usize::from_ne_bytes(memory_size);
                        current_adapter_data.total_memory = memory_size as u64;
                    }
                }

                adapters_data.push(current_adapter_data);
            }
        }

        Ok(adapters_data)
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

            for i in 0..count {
                let adapter: IDXCoreAdapter = d3d12_core_compute_adapters.GetAdapter(i)?;

                let mut is_hardware_buffer = [0u8; std::mem::size_of::<u32>()];

                adapter.GetProperty(
                    IsHardware,
                    std::mem::size_of::<u32>(), // Specify the buffer size explicitly
                    is_hardware_buffer.as_mut_ptr() as *mut core::ffi::c_void,
                )?;

                let is_hardware = u32::from_ne_bytes(is_hardware_buffer);
                let is_hardware = is_hardware != 0;

                if is_hardware {
                    // dedicated GPU Memory
                    let mut memory_size = [0u8; std::mem::size_of::<usize>()];
                    let memory_size_result = adapter.GetProperty(
                        DedicatedAdapterMemory,
                        std::mem::size_of::<usize>(), // Specify the buffer size explicitly
                        memory_size.as_mut_ptr() as *mut core::ffi::c_void,
                    );
                    let memory_size = if memory_size_result.is_ok() {
                        usize::from_ne_bytes(memory_size)
                    } else {
                        0 // Integrated GPUs often have 0 dedicated memory
                    };

                    // get AdapterMemoryBudget
                    let node_segment_group = DXCoreAdapterMemoryBudgetNodeSegmentGroup {
                        nodeIndex: 0,
                        segmentGroup: DXCoreSegmentGroup(0),
                    };

                    // Create the memory budget struct to receive the data
                    let mut memory_budget = DXCoreAdapterMemoryBudget::default();

                    // Query the state - this might fail for integrated GPUs
                    let budget_result = adapter.QueryState(
                        AdapterMemoryBudget,
                        std::mem::size_of::<DXCoreAdapterMemoryBudgetNodeSegmentGroup>(),
                        Some(&node_segment_group as *const _ as *const core::ffi::c_void),
                        std::mem::size_of::<DXCoreAdapterMemoryBudget>(),
                        &mut memory_budget as *mut _ as *mut core::ffi::c_void,
                    );

                    let is_integrated = is_integrated_gpu(&adapter)?;

                    // get hardware id
                    let mut hardware_id_buffer = [0u8; std::mem::size_of::<DXCoreHardwareID>()];

                    adapter.GetProperty(
                        HardwareID,
                        std::mem::size_of::<DXCoreHardwareID>(),
                        hardware_id_buffer.as_mut_ptr() as *mut core::ffi::c_void,
                    )?;
                    // Convert the raw buffer to DXCoreHardwareID struct
                    let hardware_id: DXCoreHardwareID =
                        std::ptr::read(hardware_id_buffer.as_ptr() as *const _);

                    // Get the description size
                    let desc_size = adapter.GetPropertySize(DriverDescription)?;
                    let mut desc_buffer = vec![0u8; desc_size];

                    adapter.GetProperty(
                        DriverDescription,
                        desc_size,
                        desc_buffer.as_mut_ptr() as *mut core::ffi::c_void,
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
                        version_buffer.as_mut_ptr() as *mut core::ffi::c_void,
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
                    current_gpu_data.driver_version = DriverVersionData {
                        major,
                        minor,
                        build,
                        revision,
                    };

                    // if nvidia
                    if hardware_id.vendorID == 0x10DE {
                        // Try to use NVML for detailed Nvidia GPU information
                        let nvml_success = match Nvml::init() {
                            Ok(nvml) => {
                                match nvml.device_count() {
                                    Ok(nv_gpu_count) if nv_gpu_count > 0 => {
                                        match nvml.device_by_index(0) {
                                            Ok(device) => {
                                                // Try to get detailed information from NVML
                                                let name_result = device.name();
                                                let arch_result = device.architecture();
                                                let memory_result = device.memory_info();

                                                match (name_result, arch_result, memory_result) {
                                                    (Ok(name), Ok(arch), Ok(memory_info)) => {
                                                        current_gpu_data.name = name;
                                                        current_gpu_data.architecture =
                                                            arch.to_string();
                                                        current_gpu_data.total_memory =
                                                            memory_info.total;
                                                        current_gpu_data.free_memory =
                                                            memory_info.free;
                                                        current_gpu_data.used_memory =
                                                            memory_info.used;
                                                        true // NVML succeeded
                                                    }
                                                    _ => false, // NVML failed to get device info
                                                }
                                            }
                                            Err(_) => false, // Failed to get device
                                        }
                                    }
                                    _ => false, // No devices or failed to get count
                                }
                            }
                            Err(_) => false, // NVML init failed
                        };

                        // If NVML failed, fall back to basic information like other vendors
                        if !nvml_success {
                            current_gpu_data.name = gpu_name;
                            current_gpu_data.architecture = "NVIDIA".to_string();
                            current_gpu_data.total_memory = memory_size as u64;

                            if budget_result.is_ok() {
                                current_gpu_data.free_memory =
                                    memory_budget.availableForReservation;
                                current_gpu_data.used_memory =
                                    memory_budget.budget - memory_budget.availableForReservation;
                            } else {
                                current_gpu_data.free_memory = 0;
                                current_gpu_data.used_memory = 0;
                            }
                        }
                    } else if hardware_id.vendorID == 0x1002 {
                        current_gpu_data.architecture = "Radeon".to_string();

                        let target_gpu_type = if is_integrated {
                            ADLX_GPU_TYPE_GPUTYPE_INTEGRATED
                        } else {
                            ADLX_GPU_TYPE_GPUTYPE_DISCRETE
                        };

                        let adlx_success = match AdlxHelper::new() {
                            Ok(adlx_helper) => {
                                match (
                                    adlx_helper.system().gpus(),
                                    adlx_helper.system().performance_monitoring_services(),
                                ) {
                                    (Ok(adlx_gpus), Ok(pms)) => {
                                        let mut found = false;
                                        for adlx_gpu in adlx_gpus.iter() {
                                            if adlx_gpu.type_().unwrap_or(0) == target_gpu_type {
                                                if let (Ok(name), Ok(total_vram)) =
                                                    (adlx_gpu.name(), adlx_gpu.total_vram())
                                                {
                                                    current_gpu_data.name = name.to_string();
                                                    current_gpu_data.total_memory =
                                                        total_vram as u64;

                                                    if let Ok(metrics) =
                                                        pms.current_gpu_metrics(&adlx_gpu)
                                                    {
                                                        if let Ok(vram_used) = metrics.vram() {
                                                            current_gpu_data.used_memory =
                                                                vram_used as u64;
                                                            current_gpu_data.free_memory =
                                                                total_vram as u64
                                                                    - vram_used as u64;
                                                        }
                                                    }
                                                    found = true;
                                                    break;
                                                }
                                            }
                                        }
                                        found
                                    }
                                    _ => false,
                                }
                            }
                            Err(_) => false,
                        };

                        if !adlx_success {
                            current_gpu_data.name = gpu_name;
                            current_gpu_data.total_memory = memory_size as u64;

                            if budget_result.is_ok() {
                                current_gpu_data.free_memory =
                                    memory_budget.availableForReservation;
                                current_gpu_data.used_memory =
                                    memory_budget.budget - memory_budget.availableForReservation;
                            } else {
                                current_gpu_data.free_memory = 0;
                                current_gpu_data.used_memory = 0;
                            }
                        }
                    } else {
                        if hardware_id.vendorID == 0x8086 {
                            current_gpu_data.architecture = "Intel".to_string();
                        } else if hardware_id.vendorID == 0x14E4 {
                            current_gpu_data.architecture = "Qualcomm".to_string();
                        }

                        current_gpu_data.name = gpu_name;
                        current_gpu_data.total_memory = memory_size as u64;

                        if budget_result.is_ok() {
                            current_gpu_data.free_memory = memory_budget.availableForReservation;
                            current_gpu_data.used_memory =
                                memory_budget.budget - memory_budget.availableForReservation;
                        } else {
                            current_gpu_data.free_memory = 0;
                            current_gpu_data.used_memory = 0;
                        }
                    }

                    // Include all GPUs, even those with 0 memory (integrated GPUs)
                    gpus_data.push(current_gpu_data);
                }
            }
        }

        // Sort the GPUs
        gpus_data.sort_by(|a, b| {
            match (a.is_high_memory_dedicated(), b.is_high_memory_dedicated()) {
                // If both are high memory dedicated or both are not, keep original order
                (true, true) | (false, false) => std::cmp::Ordering::Equal,
                // If a is high memory dedicated but b is not, a comes first
                (true, false) => std::cmp::Ordering::Less,
                // If b is high memory dedicated but a is not, b comes first
                (false, true) => std::cmp::Ordering::Greater,
            }
        });

        Ok(gpus_data)
    }

    // Get the total gpu memory of the system
    pub fn total_gpu_memory(adapter_index: u32) -> u64 {
        Self::get_gpus_list()
            .map(|gpus| {
                gpus.iter()
                    .find(|gpu| gpu.adapter_index == adapter_index)
                    .map_or(0, |gpu| gpu.total_memory)
            })
            .unwrap_or(0)
    }

    // Get the allocated gpu memory
    pub fn gpu_memory_usage(adapter_index: u32) -> u64 {
        Self::get_gpus_list()
            .map(|gpus| {
                gpus.iter()
                    .find(|gpu| gpu.adapter_index == adapter_index)
                    .map_or(0, |gpu| gpu.used_memory)
            })
            .unwrap_or(0)
    }

    pub fn gpu_memory_free(adapter_index: u32) -> u64 {
        Self::get_gpus_list()
            .map(|gpus| {
                gpus.iter()
                    .find(|gpu| gpu.adapter_index == adapter_index)
                    .map_or(0, |gpu| gpu.free_memory)
            })
            .unwrap_or(0)
    }

    pub fn has_unified_memory(adapter_index: u32) -> bool {
        Self::get_gpus_list()
            .map(|gpus| {
                gpus.iter()
                    .find(|gpu| gpu.adapter_index == adapter_index)
                    .map_or(false, |gpu| gpu.has_unified_memory)
            })
            .unwrap_or(false)
    }
}

fn is_integrated_gpu(
    adapter: &IDXCoreAdapter,
) -> std::result::Result<bool, Box<dyn std::error::Error>> {
    let mut integrated_buffer = [0u8; std::mem::size_of::<u32>()];

    unsafe {
        // Check IsIntegrated property
        if let Ok(_) = adapter.GetProperty(
            IsIntegrated,
            std::mem::size_of::<u32>(),
            integrated_buffer.as_mut_ptr() as *mut core::ffi::c_void,
        ) {
            if integrated_buffer != [0, 0, 0, 0] {
                return Ok(true);
            }
        }

        // Get the description size
        let desc_size = adapter.GetPropertySize(DriverDescription)?;
        let mut desc_buffer = vec![0u8; desc_size];

        adapter.GetProperty(
            DriverDescription,
            desc_size,
            desc_buffer.as_mut_ptr() as *mut core::ffi::c_void,
        )?;

        let gpu_name = String::from_utf8_lossy(&desc_buffer)
            .trim_end_matches('\0')
            .to_string();

        // Check common integrated GPU patterns
        return Ok(gpu_name.contains("amd radeon(tm) graphics")
            || gpu_name.contains("AMD Radeon(TM) Graphics")
            || gpu_name.contains("ryzen")
            || gpu_name.contains("uhd graphics"));
    }
}
