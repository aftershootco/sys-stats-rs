use crate::npu::{NPUData, NPUUsage};
use std::string::String;

use anyhow::Result;

//
// #[derive(Debug, Default)]
// pub struct DXCoreAdapterProperties {
//     pub instance_luid: u64,
//     pub driver_version: u32,
//     pub driver_description: u32,
//     pub hardware_id: DXCoreHardwareID,
//     pub kmd_model_version: u32,
//     pub compute_preemption_granularity: u32,
//     pub graphics_preemption_granularity: u32,
//     pub dedicated_adapter_memory: u64,
//     pub dedicated_system_memory: u64,
//     pub shared_system_memory: u64,
//     pub acg_compatible: bool,
//     pub is_hardware: bool,
//     pub is_integrated: bool,
//     pub is_detachable: bool,
//     // pub hardware_id_parts: [u32; 4],
//     pub is_npu: bool,
// }

//
// #[derive(Debug, Default)]
// pub struct PropertyKeyInfo {
//     pub supported_properties: Vec<String>,
//     pub available_devices: u32,
//     pub optimal_number_of_infer_requests: u32,
//     pub range_for_async_infer_requests: (u32, u32),
//     pub range_for_streams: String,
//     pub device_full_name: String,
//     pub device_capabilities: Vec<String>,
//     pub model_name: String,
//     pub optimal_batch_size: u32,
//     pub max_batch_size: u32,
//     pub rw_property_key: RwPropertyKeyInfo,
//     // pub other: Option<Cow<'static, str>>,
// }

// #[derive(Debug, Default)]
// pub struct RwPropertyKeyInfo {
//     pub cache_dir: String,
//     pub cache_mode: String,
//     pub num_streams: u32,
//     pub affinity: String,
//     pub inference_num_threads: u32,
//     pub hint_enable_cpu_pinning: bool,
//     pub hint_enable_hyper_threading: bool,
//     pub hint_performance_mode: String,
//     pub hint_scheduling_core_type: String,
//     pub hint_inference_precision: String,
//     pub hint_num_requests: u32,
//     pub log_level: String,
//     pub hint_model_priority: String,
//     pub enable_profiling: bool,
//     pub device_priorities: String,
//     pub hint_execution_mode: String,
//     pub force_tbb_terminate: bool,
//     pub enable_mmap: bool,
//     pub auto_batch_timeout: u32,
//     // pub other: Cow<'static, str>,
// }

// impl PropertyKeyInfo {
//     #[allow(dead_code)]
//     pub fn new() -> Self {
//         PropertyKeyInfo {
//             supported_properties: Vec::new(),
//             available_devices: 0,
//             optimal_number_of_infer_requests: 0,
//             range_for_async_infer_requests: (0, 0),
//             range_for_streams: String::new(),
//             device_full_name: String::new(),
//             device_capabilities: Vec::new(),
//             model_name: String::new(),
//             optimal_batch_size: 0,
//             max_batch_size: 0,
//             rw_property_key: RwPropertyKeyInfo::default(),
//         }
//     }
// }
//
// impl RwPropertyKeyInfo {
//     pub fn new() -> Self {
//         RwPropertyKeyInfo {
//             cache_dir: String::new(),
//             cache_mode: String::new(),
//             num_streams: 0,
//             affinity: String::new(),
//             inference_num_threads: 0,
//             hint_enable_cpu_pinning: false,
//             hint_enable_hyper_threading: false,
//             hint_performance_mode: String::new(),
//             hint_scheduling_core_type: String::new(),
//             hint_inference_precision: String::new(),
//             hint_num_requests: 0,
//             log_level: String::new(),
//             hint_model_priority: String::new(),
//             enable_profiling: false,
//             device_priorities: String::new(),
//             hint_execution_mode: String::new(),
//             force_tbb_terminate: false,
//             enable_mmap: false,
//             auto_batch_timeout: 0,
//             // other: Cow::Borrowed(""),
//         }
//     }
// }

impl NPUUsage {
    pub fn is_npu_available() -> bool {
        let (arch, vendor) = Self::get_platform_details();
        //
        // if arch == "x64" {
        //     if vendor.contains("Intel") {
        //         match Self::get_intel_npu_info() {
        //             Ok(npu) => true,
        //             Err(e) => false,
        //         };
        //
        //         // let core = openvino::Core::new().unwrap();
        //         // let all_devices: std::result::IntoIter<Vec<openvino::DeviceType<'_>>> =
        //         //     core.available_devices().into_iter();
        //
        //         // // if there is an NPU device, return true
        //         // for dev in all_devices.flatten() {
        //         //     if dev == openvino::DeviceType::NPU {
        //         //         return true;
        //         //     }
        //         // }
        //         false
        //     } else if vendor.contains("AMD") {
        //         // amd
        //         false
        //     } else {
        //         false
        //     }
        //
        //     // intel or amd
        // } else if arch == "ARM64" {
        //     // qualcomm. for now
        //     false
        // } else {
        //     false
        // }
        false
    }

    pub fn get_npu_info() -> Result<NPUData, String> {
        let (arch, vendor) = Self::get_platform_details();

        if arch == "x64" {
            if vendor.contains("Intel") {
                // let npu_data = Self::get_intel_npu_info();
                Ok(NPUData {
                    // name: npu_data.device_full_name,
                    name: "Intel AI Boost".to_string(),
                    capability: 0.0,
                })
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
                return 0.0;
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
        // let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        // unsafe {
        //     GetSystemInfo(&mut sys_info);
        // }
        //
        // let arch = match unsafe { sys_info.u.s().wProcessorArchitecture } {
        //     0 => "x86".to_string(),
        //     5 => "ARM".to_string(),
        //     9 => "x64".to_string(),
        //     12 => "ARM64".to_string(),
        //     14 => "RISC-V".to_string(),
        //     _ => "Unknown".to_string(),
        // };
        //
        // // platform name
        // // use wmic
        // let output = std::process::Command::new("wmic")
        //     .args(&["cpu", "get", "name"])
        //     .output()
        //     .expect("failed to execute process");
        //
        // let vendor = String::from_utf8_lossy(&output.stdout);
        // let vendor = vendor.split("\n").collect::<Vec<&str>>();
        // let vendor = vendor[1].trim();
        //
        // (arch, vendor.to_string())
        ("x64".to_string(), "Intel".to_string())
    }
    //
    // pub fn get_intel_npu_info() -> Result<DXCoreAdapterProperties, String> {
    //     let factory: IDXCoreAdapterFactory =
    //         unsafe { DXCoreCreateAdapterFactory().map_err(|e| e.message().to_string())? };
    //
    //     let mut adapter_list: Option<IDXCoreAdapterList> = None;
    //     unsafe {
    //         adapter_list = factory
    //             .CreateAdapterList(&[DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE])
    //             .map(|list| Some(list))
    //             .map_err(|e| e.message().to_string())?;
    //
    //         let adapter_list = adapter_list.ok_or("Failed to create DXCore adapter list")?;
    //
    //         let adapter_count = unsafe { adapter_list.GetAdapterCount() };
    //
    //         for i in 0..adapter_count {
    //             let mut adapter: Option<IDXCoreAdapter> = None;
    //             unsafe {
    //                 adapter = adapter_list
    //                     .GetAdapter(i)
    //                     .map(|adapter| Some(adapter))
    //                     .map_err(|e| e.message().to_string())?;
    //             }
    //             let adapter = adapter.ok_or("Failed to get DXCore adapter")?;
    //
    //             let mut hardware_id: DXCoreHardwareID = DXCoreHardwareID::default();
    //             unsafe {
    //                 adapter
    //                     .GetProperty(
    //                         HardwareID,
    //                         size_of::<DXCoreHardwareID>(),
    //                         &mut hardware_id as *mut _ as *mut _,
    //                     )
    //                     .map_err(|e| e.message().to_string())?;
    //             }
    //
    //             // Check if the adapter matches the NPU device ID
    //             // if hardware_id.vendorID == 0x8086 && hardware_id.deviceID == 0x7D1D {
    //             //     let mut adapter_properties = DXCoreAdapterProperties::new();
    //             //     adapter_properties.hardware_id = hardware_id;
    //             //
    //             //     // Retrieve other properties
    //             //     unsafe {
    //             //         adapter
    //             //             .GetProperty(
    //             //                 InstanceLuid,
    //             //                 size_of::<u64>(),
    //             //                 &mut adapter_properties.instance_luid as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         let mut driver_version: DXCoreAdapterProperty =
    //             //             DXCoreAdapterProperty::default();
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 DriverVersion,
    //             //                 size_of::<[u16; 16]>(),
    //             //                 &mut driver_version as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //         adapter_properties.driver_version = driver_version.0;
    //             //
    //             //         let mut driver_description: DXCoreAdapterProperty =
    //             //             DXCoreAdapterProperty::default();
    //             //         adapter
    //             //             .GetProperty(
    //             //                 DriverDescription,
    //             //                 size_of::<[u16; 16]>(),
    //             //                 &mut driver_description as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //         adapter_properties.driver_description = driver_description.0;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 KmdModelVersion,
    //             //                 size_of::<u32>(),
    //             //                 &mut adapter_properties.kmd_model_version as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 ComputePreemptionGranularity,
    //             //                 size_of::<u32>(),
    //             //                 &mut adapter_properties.compute_preemption_granularity as *mut _
    //             //                     as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 GraphicsPreemptionGranularity,
    //             //                 size_of::<u32>(),
    //             //                 &mut adapter_properties.graphics_preemption_granularity as *mut _
    //             //                     as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 DedicatedAdapterMemory,
    //             //                 size_of::<u64>(),
    //             //                 &mut adapter_properties.dedicated_adapter_memory as *mut _
    //             //                     as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 DedicatedSystemMemory,
    //             //                 size_of::<u64>(),
    //             //                 &mut adapter_properties.dedicated_system_memory as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 SharedSystemMemory,
    //             //                 size_of::<u64>(),
    //             //                 &mut adapter_properties.shared_system_memory as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 AcgCompatible,
    //             //                 size_of::<bool>(),
    //             //                 &mut adapter_properties.acg_compatible as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 IsHardware,
    //             //                 size_of::<bool>(),
    //             //                 &mut adapter_properties.is_hardware as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 IsIntegrated,
    //             //                 size_of::<bool>(),
    //             //                 &mut adapter_properties.is_integrated as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         adapter
    //             //             .GetProperty(
    //             //                 IsDetachable,
    //             //                 size_of::<bool>(),
    //             //                 &mut adapter_properties.is_detachable as *mut _ as *mut _,
    //             //             )
    //             //             .map_err(|e| e.message().to_string())?;
    //             //
    //             //         // adapter.GetProperty(HardwareIDParts, size_of::<[u32; 4]>(), &mut adapter_properties.hardware_id_parts as *mut _ as *mut _)
    //             //         //     .map_err(|e| e.message().to_string())?;
    //             //     }
    //             //
    //             //     return Ok(adapter_properties);
    //             // }
    //         }
    //
    //         Err("No Intel NPU found".to_string())
    //     }
    // }

    // fn get_intel_npu_info() -> PropertyKeyInfo {
    //     let mut result = PropertyKeyInfo::default();

    //     let core = match openvino::Core::new() {
    //         Ok(core) => core,
    //         Err(_) => return PropertyKeyInfo::default(),
    //     };

    //     let dev = openvino::DeviceType::NPU;

    //     let supported_properties: String = core
    //         .get_property(&dev, &openvino::PropertyKey::SupportedProperties)
    //         .unwrap_or_default();

    //     // split string by whitespace
    //     result.supported_properties = supported_properties
    //         .split_whitespace()
    //         .map(|s| s.to_string())
    //         .collect();

    //     let available_devices: String = core
    //         .get_property(&dev, &openvino::PropertyKey::AvailableDevices)
    //         .unwrap_or_default();

    //     result.available_devices = available_devices.parse::<u32>().unwrap_or(0);

    //     let optimal_number_of_infer_requests: String = core
    //         .get_property(&dev, &openvino::PropertyKey::OptimalNumberOfInferRequests)
    //         .unwrap_or_default();

    //     result.optimal_number_of_infer_requests =
    //         optimal_number_of_infer_requests.parse::<u32>().unwrap_or(0);

    //     let range_for_async_infer_requests: String = core
    //         .get_property(&dev, &openvino::PropertyKey::RangeForAsyncInferRequests)
    //         .unwrap_or_default();

    //     let range_for_async_infer_requests: Vec<&str> =
    //         range_for_async_infer_requests.split_whitespace().collect();
    //     result.range_for_async_infer_requests = (
    //         range_for_async_infer_requests[0]
    //             .parse::<u32>()
    //             .unwrap_or_default(),
    //         range_for_async_infer_requests[1]
    //             .parse::<u32>()
    //             .unwrap_or_default(),
    //     );

    //     result.range_for_streams = core
    //         .get_property(&dev, &openvino::PropertyKey::RangeForStreams)
    //         .unwrap_or_default();

    //     result.device_full_name = core
    //         .get_property(&dev, &openvino::PropertyKey::DeviceFullName)
    //         .unwrap_or_default();

    //     let device_capabilities: String = core
    //         .get_property(&dev, &openvino::PropertyKey::DeviceCapabilities)
    //         .unwrap_or_default();

    //     let device_capabilities: Vec<&str> = device_capabilities.split_whitespace().collect();
    //     result.device_capabilities = device_capabilities.iter().map(|s| s.to_string()).collect();

    //     result.model_name = core
    //         .get_property(&dev, &openvino::PropertyKey::ModelName)
    //         .unwrap_or_default();

    //     let optimal_batch_size: String = core
    //         .get_property(&dev, &openvino::PropertyKey::OptimalBatchSize)
    //         .unwrap_or_default();

    //     result.optimal_batch_size = optimal_batch_size.parse::<u32>().unwrap_or(0);

    //     let max_batch_size: String = core
    //         .get_property(&dev, &openvino::PropertyKey::MaxBatchSize)
    //         .unwrap_or_default();

    //     result.max_batch_size = max_batch_size.parse::<u32>().unwrap_or_default();

    //     result.rw_property_key = Self::get_rw_property_key(core, dev);

    //     result
    // }

    // fn get_rw_property_key(core: openvino::Core, dev: openvino::DeviceType) -> RwPropertyKeyInfo {
    //     let mut rw_property_key = RwPropertyKeyInfo::new();

    //     rw_property_key.cache_dir = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::CacheDir),
    //         )
    //         .unwrap_or_default();

    //     rw_property_key.cache_mode = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::CacheMode),
    //         )
    //         .unwrap_or_default();

    //     let num_stream = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::NumStreams),
    //         )
    //         .unwrap_or_default();
    //     rw_property_key.num_streams = num_stream.parse::<u32>().unwrap_or(0);

    //     rw_property_key.affinity = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::Affinity),
    //         )
    //         .unwrap_or_default();

    //     let inference_num_threads = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::InferenceNumThreads),
    //         )
    //         .unwrap_or_default();

    //     rw_property_key.inference_num_threads = inference_num_threads.parse::<u32>().unwrap_or(0);

    //     let hint_enable_cpu_pinning = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintEnableCpuPinning),
    //         )
    //         .unwrap_or("false".to_string());

    //     rw_property_key.hint_enable_cpu_pinning =
    //         hint_enable_cpu_pinning.parse::<bool>().unwrap_or(false);

    //     let hint_enable_hyper_threading = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintEnableHyperThreading),
    //         )
    //         .unwrap_or("false".to_string());
    //     rw_property_key.hint_enable_hyper_threading =
    //         hint_enable_hyper_threading.parse::<bool>().unwrap_or(false);

    //     rw_property_key.hint_performance_mode = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintPerformanceMode),
    //         )
    //         .unwrap_or("".to_string());

    //     rw_property_key.hint_scheduling_core_type = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintSchedulingCoreType),
    //         )
    //         .unwrap_or("".to_string());

    //     rw_property_key.hint_inference_precision = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintInferencePrecision),
    //         )
    //         .unwrap_or("".to_string());

    //     let hint_num_requests = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintNumRequests),
    //         )
    //         .unwrap_or("0".to_string());

    //     rw_property_key.hint_num_requests = hint_num_requests.parse::<u32>().unwrap_or(0);

    //     rw_property_key.log_level = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::LogLevel),
    //         )
    //         .unwrap_or("".to_string());

    //     rw_property_key.hint_model_priority = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintModelPriority),
    //         )
    //         .unwrap_or("".to_string());

    //     let enable_profiling = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::EnableProfiling),
    //         )
    //         .unwrap_or("false".to_string());

    //     rw_property_key.enable_profiling = enable_profiling.parse::<bool>().unwrap_or(false);

    //     rw_property_key.device_priorities = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::DevicePriorities),
    //         )
    //         .unwrap_or("".to_string());

    //     rw_property_key.hint_execution_mode = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::HintExecutionMode),
    //         )
    //         .unwrap_or("".to_string());

    //     let force_tbb_terminate = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::ForceTbbTerminate),
    //         )
    //         .unwrap_or("false".to_string());

    //     rw_property_key.force_tbb_terminate = force_tbb_terminate.parse::<bool>().unwrap_or(false);

    //     let enable_mmap = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::EnableMmap),
    //         )
    //         .unwrap_or("false".to_string());

    //     rw_property_key.enable_mmap = enable_mmap.parse::<bool>().unwrap_or(false);

    //     let auto_batch_timeout = core
    //         .get_property(
    //             &dev,
    //             &openvino::PropertyKey::Rw(openvino::RwPropertyKey::AutoBatchTimeout),
    //         )
    //         .unwrap_or("0".to_string());

    //     rw_property_key.auto_batch_timeout = auto_batch_timeout.parse::<u32>().unwrap_or(0);

    //     rw_property_key
    // }
}
