use crate::npu::{NPUData, NPUUsage};
use anyhow::Result;
use winapi::um::sysinfoapi::GetSystemInfo;
use winapi::um::sysinfoapi::SYSTEM_INFO;

use windows::Win32::Graphics::DXCore::DXCoreAdapterProperty;
use windows::Win32::Graphics::DXCore::DXCoreCreateAdapterFactory;
use windows::Win32::Graphics::DXCore::DXCoreHardwareID;
use windows::Win32::Graphics::DXCore::DriverDescription;
use windows::Win32::Graphics::DXCore::DriverVersion;
// use windows::Win32::Graphics::DXCore::HardwareIDParts;
use windows::Win32::Graphics::DXCore::AcgCompatible;
use windows::Win32::Graphics::DXCore::ComputePreemptionGranularity;
use windows::Win32::Graphics::DXCore::DedicatedAdapterMemory;
use windows::Win32::Graphics::DXCore::DedicatedSystemMemory;
use windows::Win32::Graphics::DXCore::GraphicsPreemptionGranularity;
use windows::Win32::Graphics::DXCore::IDXCoreAdapter;
use windows::Win32::Graphics::DXCore::IDXCoreAdapterFactory;
use windows::Win32::Graphics::DXCore::IDXCoreAdapterList;
use windows::Win32::Graphics::DXCore::InstanceLuid;
use windows::Win32::Graphics::DXCore::KmdModelVersion;
use windows::Win32::Graphics::DXCore::SharedSystemMemory;
use windows::Win32::Graphics::DXCore::{
    HardwareID, IsDetachable, IsHardware, IsIntegrated, DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE,
};

#[derive(Debug, Default)]
pub struct DXCoreAdapterProperties {
    pub instance_luid: u64,
    pub driver_version: u32,
    pub driver_description: u32,
    pub hardware_id: DXCoreHardwareID,
    pub kmd_model_version: u32,
    pub compute_preemption_granularity: u32,
    pub graphics_preemption_granularity: u32,
    pub dedicated_adapter_memory: u64,
    pub dedicated_system_memory: u64,
    pub shared_system_memory: u64,
    pub acg_compatible: bool,
    pub is_hardware: bool,
    pub is_integrated: bool,
    pub is_detachable: bool,
    // pub hardware_id_parts: [u32; 4],
    pub is_npu: bool,
}

impl DXCoreAdapterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NPUUsage {
    pub fn is_npu_available() -> bool {
        let (arch, vendor) = Self::get_platform_details();

        if arch == "x64" {
            if vendor.contains("Intel") {
                match Self::get_intel_npu_info() {
                    Ok(npu) => true,
                    Err(e) => false,
                }
            } else if vendor.contains("AMD") {
                // amd
                false
            } else {
                false
            }

            // intel or amd
        } else if arch == "ARM64" {
            // qualcomm. for now
            false
        } else {
            false
        }
    }

    pub fn get_npu_info() -> Result<NPUData, String> {
        let (arch, vendor) = Self::get_platform_details();

        if arch == "x64" {
            if vendor.contains("Intel") {
                match Self::get_intel_npu_info() {
                    Ok(npu) => Ok(NPUData {
                        name: "Intel NPU".to_string(),
                        usage: 0.0,
                        capability: 0.0,
                    }),
                    Err(e) => Err(e),
                }
            } else if vendor.contains("AMD") {
                // amd
                Err("AMD NPU not supported".to_string())
            } else {
                Err("unknown platform".to_string())
            }

            // intel or amd
        } else if arch == "ARM64" {
            // qualcomm. for now
            Err("Qualcomm NPU not supported".to_string())
        } else {
            Err("Unknown platform".to_string())
        }
    }

    pub fn total_npu_capability() -> f32 {
        let (arch, vendor) = Self::get_platform_details();

        if arch == "x64" {
            if vendor.contains("Intel") {
                match Self::get_intel_npu_info() {
                    Ok(npu) => {
                        // now we have to get npu tops

                        println!("{:#?}", npu);
                        return 0.0;
                    }
                    Err(e) => eprintln!("{}", e),
                }
            } else if vendor.contains("AMD") {
                // amd
            } else {
                eprintln!("Unknown platform");
            }

            // intel or amd
        } else if arch == "ARM64" {
            // qualcomm. for now
        } else {
            eprintln!("Unknown platform");
        }
        0.0
    }

    pub fn current_npu_usage() -> f32 {
        0.0
    }

    fn get_platform_details() -> (String, String) {
        // architecture name
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }

        let arch = match unsafe { sys_info.u.s().wProcessorArchitecture } {
            0 => "x86".to_string(),
            5 => "ARM".to_string(),
            9 => "x64".to_string(),
            12 => "ARM64".to_string(),
            14 => "RISC-V".to_string(),
            _ => "Unknown".to_string(),
        };

        // platform name
        // use wmic
        let output = std::process::Command::new("wmic")
            .args(&["cpu", "get", "name"])
            .output()
            .expect("failed to execute process");

        let vendor = String::from_utf8_lossy(&output.stdout);
        let vendor = vendor.split("\n").collect::<Vec<&str>>();
        let vendor = vendor[1].trim();

        (arch, vendor.to_string())
    }

    pub fn get_intel_npu_info() -> Result<DXCoreAdapterProperties, String> {
        let factory: IDXCoreAdapterFactory =
            unsafe { DXCoreCreateAdapterFactory().map_err(|e| e.message().to_string())? };

        let mut adapter_list: Option<IDXCoreAdapterList> = None;
        unsafe {
            adapter_list = factory
                .CreateAdapterList(&[DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE])
                .map(|list| Some(list))
                .map_err(|e| e.message().to_string())?;

            let adapter_list = adapter_list.ok_or("Failed to create DXCore adapter list")?;

            let adapter_count = unsafe { adapter_list.GetAdapterCount() };

            for i in 0..adapter_count {
                let mut adapter: Option<IDXCoreAdapter> = None;
                unsafe {
                    adapter = adapter_list
                        .GetAdapter(i)
                        .map(|adapter| Some(adapter))
                        .map_err(|e| e.message().to_string())?;
                }
                let adapter = adapter.ok_or("Failed to get DXCore adapter")?;

                let mut hardware_id: DXCoreHardwareID = DXCoreHardwareID::default();
                unsafe {
                    adapter
                        .GetProperty(
                            HardwareID,
                            size_of::<DXCoreHardwareID>(),
                            &mut hardware_id as *mut _ as *mut _,
                        )
                        .map_err(|e| e.message().to_string())?;
                }

                // Check if the adapter matches the NPU device ID
                if hardware_id.vendorID == 0x8086 && hardware_id.deviceID == 0x7D1D {
                    let mut adapter_properties = DXCoreAdapterProperties::new();
                    adapter_properties.hardware_id = hardware_id;

                    // Retrieve other properties
                    unsafe {
                        adapter
                            .GetProperty(
                                InstanceLuid,
                                size_of::<u64>(),
                                &mut adapter_properties.instance_luid as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        let mut driver_version: DXCoreAdapterProperty = DXCoreAdapterProperty::default();

                        adapter
                            .GetProperty(
                                DriverVersion,
                                size_of::<[u16; 16]>(), &mut driver_version as *mut _ as *mut _)
                                .map_err(|e| e.message().to_string())?;
                            adapter_properties.driver_version = driver_version.0;

                        let mut driver_description: DXCoreAdapterProperty = DXCoreAdapterProperty::default();
                        adapter
                            .GetProperty(
                                DriverDescription,
                                size_of::<[u16; 16]>(),
                                &mut driver_description as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;
                        adapter_properties.driver_description = driver_description.0;

                        adapter
                            .GetProperty(
                                KmdModelVersion,
                                size_of::<u32>(),
                                &mut adapter_properties.kmd_model_version as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                ComputePreemptionGranularity,
                                size_of::<u32>(),
                                &mut adapter_properties.compute_preemption_granularity as *mut _
                                    as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                GraphicsPreemptionGranularity,
                                size_of::<u32>(),
                                &mut adapter_properties.graphics_preemption_granularity as *mut _
                                    as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                DedicatedAdapterMemory,
                                size_of::<u64>(),
                                &mut adapter_properties.dedicated_adapter_memory as *mut _
                                    as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                DedicatedSystemMemory,
                                size_of::<u64>(),
                                &mut adapter_properties.dedicated_system_memory as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                SharedSystemMemory,
                                size_of::<u64>(),
                                &mut adapter_properties.shared_system_memory as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                AcgCompatible,
                                size_of::<bool>(),
                                &mut adapter_properties.acg_compatible as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                IsHardware,
                                size_of::<bool>(),
                                &mut adapter_properties.is_hardware as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                IsIntegrated,
                                size_of::<bool>(),
                                &mut adapter_properties.is_integrated as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        adapter
                            .GetProperty(
                                IsDetachable,
                                size_of::<bool>(),
                                &mut adapter_properties.is_detachable as *mut _ as *mut _,
                            )
                            .map_err(|e| e.message().to_string())?;

                        // adapter.GetProperty(HardwareIDParts, size_of::<[u32; 4]>(), &mut adapter_properties.hardware_id_parts as *mut _ as *mut _)
                        //     .map_err(|e| e.message().to_string())?;
                    }

                    return Ok(adapter_properties);
                }
            }

            Err("No Intel NPU found".to_string())
        }
    }
}
