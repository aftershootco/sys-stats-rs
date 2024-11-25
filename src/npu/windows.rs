use crate::npu::{NPUData, NPUUsage};
use openvino;
use std::borrow::Cow;
use std::string::String;
use winapi::um::sysinfoapi::GetSystemInfo;
use winapi::um::sysinfoapi::SYSTEM_INFO;

#[derive(Debug, Default)]
pub struct PropertyKeyInfo {
    pub supported_properties: String,
    pub available_devices: String,
    pub optimal_number_of_infer_requests: String,
    pub range_for_async_infer_requests: String,
    pub range_for_streams: String,
    pub device_full_name: String,
    pub device_capabilities: String,
    pub model_name: String,
    pub optimal_batch_size: String,
    pub max_batch_size: String,
    // pub rw_property_key: Option<RwPropertyKeyInfo>,
    // pub other: Option<Cow<'static, str>>,
}

#[derive(Debug, Default)]
pub struct RwPropertyKeyInfo {
    pub cache_dir: Option<String>,
    pub cache_mode: Option<String>,
    pub num_streams: Option<u32>,
    pub affinity: Option<String>,
    pub inference_num_threads: Option<u32>,
    pub hint_enable_cpu_pinning: Option<bool>,
    pub hint_enable_hyper_threading: Option<bool>,
    pub hint_performance_mode: Option<String>,
    pub hint_scheduling_core_type: Option<String>,
    pub hint_inference_precision: Option<String>,
    pub hint_num_requests: Option<u32>,
    pub log_level: Option<String>,
    pub hint_model_priority: Option<String>,
    pub enable_profiling: Option<bool>,
    pub device_priorities: Option<String>,
    pub hint_execution_mode: Option<String>,
    pub force_tbb_terminate: Option<bool>,
    pub enable_mmap: Option<bool>,
    pub auto_batch_timeout: Option<u32>,
    pub other: Option<Cow<'static, str>>,
}

impl PropertyKeyInfo {
    pub fn new() -> Self {
        PropertyKeyInfo {
            supported_properties: String::new(),
            available_devices: String::new(),
            optimal_number_of_infer_requests: String::new(),
            range_for_async_infer_requests: String::new(),
            range_for_streams: String::new(),
            device_full_name: String::new(),
            device_capabilities: String::new(),
            model_name: String::new(),
            optimal_batch_size: String::new(),
            max_batch_size: String::new(),
        }
    }
}

impl NPUUsage {
    pub fn is_npu_available() -> bool {
        let (arch, vendor) = Self::get_platform_details();

        if arch == "x64" {
            if vendor.contains("Intel") {
                let core = openvino::Core::new().unwrap();
                let all_devices: std::result::IntoIter<Vec<openvino::DeviceType<'_>>> =
                    core.available_devices().into_iter();

                // if there is an NPU device, return true
                for dev in all_devices.flatten() {
                    if dev == openvino::DeviceType::NPU {
                        return true;
                    }
                }
                false
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
                let npu_data = Self::get_intel_npu_info();
                Ok(NPUData {
                    name: npu_data.device_full_name,
                    usage: 0.0,
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

    fn get_intel_npu_info() -> PropertyKeyInfo {
        let mut result = PropertyKeyInfo::default();
        let core = openvino::Core::new().unwrap();
        let dev = openvino::DeviceType::NPU;

        result.supported_properties = core
            .get_property(&dev, &openvino::PropertyKey::SupportedProperties)
            .unwrap();
        result.available_devices = core
            .get_property(&dev, &openvino::PropertyKey::AvailableDevices)
            .unwrap();
        result.optimal_number_of_infer_requests = core
            .get_property(&dev, &openvino::PropertyKey::OptimalNumberOfInferRequests)
            .unwrap();
        result.range_for_async_infer_requests = core
            .get_property(&dev, &openvino::PropertyKey::RangeForAsyncInferRequests)
            .unwrap();
        result.range_for_streams = core
            .get_property(&dev, &openvino::PropertyKey::RangeForStreams)
            .unwrap();
        result.device_full_name = core
            .get_property(&dev, &openvino::PropertyKey::DeviceFullName)
            .unwrap();
        result.device_capabilities = core
            .get_property(&dev, &openvino::PropertyKey::DeviceCapabilities)
            .unwrap();
        result.model_name = core
            .get_property(&dev, &openvino::PropertyKey::ModelName)
            .unwrap_or("N/A".to_string());
        result.optimal_batch_size = core
            .get_property(&dev, &openvino::PropertyKey::OptimalBatchSize)
            .unwrap_or("N/A".to_string());
        result.max_batch_size = core
            .get_property(&dev, &openvino::PropertyKey::MaxBatchSize)
            .unwrap_or("N/A".to_string());
        // Add more properties as needed

        result
    }
}
