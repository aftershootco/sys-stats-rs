use crate::gpu::{AdapterData, DriverVersionData, GPUData, GPUUsage};
use anyhow::{anyhow, Result};
use std::ffi::c_void;

use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::{
            DXCore::*,
            Dxgi::{
                CreateDXGIFactory2, IDXGIAdapter1, IDXGIFactory6, DXGI_ADAPTER_DESC1,
                DXGI_CREATE_FACTORY_FLAGS, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
            },
        },
        System::Performance::*,
    },
};

fn to_utf16(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

fn luid_to_string(luid: LUID) -> String {
    format!("luid_0x{:08X}_0x{:08X}", luid.HighPart as u32, luid.LowPart)
}

unsafe fn pdh_read_double(counter: PDH_HCOUNTER) -> f64 {
    let mut val = PDH_FMT_COUNTERVALUE::default();
    let mut typ = 0u32;
    if PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, Some(&mut typ), &mut val)
        == ERROR_SUCCESS.0
    {
        val.Anonymous.doubleValue
    } else {
        0.0
    }
}

unsafe fn get_gpu_pdh_memory(luid: LUID) -> Option<(u64, u64, u64)> {
    let instance = luid_to_string(luid);
    let mut query = PDH_HQUERY::default();
    if PdhOpenQueryW(None, 0, &mut query) != ERROR_SUCCESS.0 {
        return None;
    }

    unsafe fn add_counter(query: PDH_HQUERY, path: &str, counter: &mut PDH_HCOUNTER) -> bool {
        let utf = to_utf16(path);
        PdhAddCounterW(query, PCWSTR(utf.as_ptr()), 0, counter) == ERROR_SUCCESS.0
    }

    let mut c_ded = PDH_HCOUNTER::default();
    let mut c_sha = PDH_HCOUNTER::default();
    let mut c_com = PDH_HCOUNTER::default();

    add_counter(
        query,
        &format!("\\GPU Adapter Memory({}*)\\Dedicated Usage", instance),
        &mut c_ded,
    );
    add_counter(
        query,
        &format!("\\GPU Adapter Memory({}*)\\Shared Usage", instance),
        &mut c_sha,
    );
    add_counter(
        query,
        &format!("\\GPU Adapter Memory({}*)\\Total Committed", instance),
        &mut c_com,
    );

    PdhCollectQueryData(query);
    PdhCollectQueryData(query);

    let ded = pdh_read_double(c_ded) as u64;
    let sha = pdh_read_double(c_sha) as u64;
    let com = pdh_read_double(c_com) as u64;

    PdhCloseQuery(query);

    Some((ded, sha, com))
}

fn wide_to_string(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}

unsafe fn dxgi_get_adapter_desc(index: u32) -> Option<(IDXGIAdapter1, DXGI_ADAPTER_DESC1)> {
    let factory: IDXGIFactory6 = CreateDXGIFactory2(DXGI_CREATE_FACTORY_FLAGS(0)).ok()?;
    let adapter = factory
        .EnumAdapterByGpuPreference::<IDXGIAdapter1>(index, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE)
        .ok()?;
    let desc = adapter.GetDesc1().ok()?;
    Some((adapter, desc))
}

unsafe fn get_bool_prop(adapter: &IDXCoreAdapter, prop: DXCoreAdapterProperty) -> bool {
    let mut buf = [0u8; 4];
    adapter
        .GetProperty(prop, 4, buf.as_mut_ptr() as *mut c_void)
        .is_ok()
        && u32::from_ne_bytes(buf) != 0
}

fn vendor_to_arch(vendor: u32) -> &'static str {
    match vendor {
        0x1002 => "Radeon",
        0x10DE => "NVIDIA",
        0x8086 => "Intel",
        0x14E4 => "Qualcomm",
        _ => "Unknown",
    }
}

impl GPUUsage {
    pub fn get_gpu_info() -> Result<GPUData> {
        Self::get_gpus_list()?
            .into_iter()
            .next()
            .ok_or(anyhow!("No GPU found"))
    }

    pub fn get_gpus_list() -> Result<Vec<GPUData>> {
        let mut gpus = Vec::new();

        unsafe {
            let factory: IDXCoreAdapterFactory = DXCoreCreateAdapterFactory()?;
            let attrs = [DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE];
            let list: IDXCoreAdapterList = factory.CreateAdapterList(&attrs)?;
            let count = list.GetAdapterCount();

            for i in 0..count {
                let adapter = list.GetAdapter(i)?;
                if !get_bool_prop(&adapter, IsHardware) {
                    continue;
                }

                let is_integrated = get_bool_prop(&adapter, IsIntegrated);
                let ver_size = adapter.GetPropertySize(DriverVersion)?;
                let mut ver_buf = vec![0u8; ver_size];
                adapter.GetProperty(
                    DriverVersion,
                    ver_size,
                    ver_buf.as_mut_ptr() as *mut c_void,
                )?;
                let ver = u64::from_ne_bytes(ver_buf.try_into().unwrap());
                let driver = DriverVersionData {
                    major: ((ver >> 48) & 0xFFFF) as u64,
                    minor: ((ver >> 32) & 0xFFFF) as u64,
                    build: ((ver >> 16) & 0xFFFF) as u64,
                    revision: (ver & 0xFFFF) as u64,
                };

                if let Some((_, dxgi_desc)) = dxgi_get_adapter_desc(i) {
                    let name = wide_to_string(&dxgi_desc.Description);
                    let vendor = dxgi_desc.VendorId;

                    let dedicated = dxgi_desc.DedicatedVideoMemory as u64;
                    let shared = dxgi_desc.SharedSystemMemory as u64;
                    let total = if is_integrated {
                        dedicated + shared
                    } else {
                        dedicated
                    };

                    let (ded_used, sha_used, _) =
                        get_gpu_pdh_memory(dxgi_desc.AdapterLuid).unwrap_or((0, 0, 0));
                    let used = if is_integrated {
                        ded_used + sha_used
                    } else {
                        ded_used
                    };
                    let free = total.saturating_sub(used);

                    gpus.push(GPUData {
                        name,
                        architecture: vendor_to_arch(vendor).to_string(),
                        vendor_id: vendor,
                        total_memory: total,
                        free_memory: free,
                        used_memory: used,
                        has_unified_memory: is_integrated,
                        is_integrated,
                        adapter_index: i,
                        driver_version: driver,
                    });
                }
            }
        }

        gpus.sort_by(|a, b| b.total_memory.cmp(&a.total_memory));
        Ok(gpus)
    }

    /// Get list of all adapters in the system (hardware and non-hardware)
    pub fn get_all_adapters_list() -> Result<Vec<AdapterData>, Box<dyn std::error::Error>> {
        let mut adapters_list = Vec::new();

        unsafe {
            let factory: IDXCoreAdapterFactory = DXCoreCreateAdapterFactory()?;
            let attrs = [DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE];
            let list: IDXCoreAdapterList = factory.CreateAdapterList(&attrs)?;
            let count = list.GetAdapterCount();

            for i in 0..count {
                let adapter: IDXCoreAdapter = list.GetAdapter(i)?;

                let is_hardware = {
                    let mut buf = [0u8; 4];
                    adapter
                        .GetProperty(IsHardware, 4, buf.as_mut_ptr() as *mut _)
                        .is_ok()
                        && u32::from_ne_bytes(buf) != 0
                };

                let is_integrated = {
                    let mut buf = [0u8; 4];
                    adapter
                        .GetProperty(IsIntegrated, 4, buf.as_mut_ptr() as *mut _)
                        .is_ok()
                        && u32::from_ne_bytes(buf) != 0
                };

                let ver_size = adapter.GetPropertySize(DriverVersion)?;
                let mut ver_buf = vec![0u8; ver_size];
                adapter.GetProperty(DriverVersion, ver_size, ver_buf.as_mut_ptr() as *mut _)?;
                let ver = u64::from_ne_bytes(ver_buf.try_into().unwrap());

                let driver_version = DriverVersionData {
                    major: ((ver >> 48) & 0xFFFF) as u64,
                    minor: ((ver >> 32) & 0xFFFF) as u64,
                    build: ((ver >> 16) & 0xFFFF) as u64,
                    revision: (ver & 0xFFFF) as u64,
                };

                let (name, vendor_id, device_id, total_memory, architecture) =
                    if let Some((_, dxgi_desc)) = dxgi_get_adapter_desc(i) {
                        let name = wide_to_string(&dxgi_desc.Description);
                        let vendor_id = dxgi_desc.VendorId;
                        let device_id = dxgi_desc.DeviceId;
                        let total_memory = dxgi_desc.DedicatedVideoMemory as u64
                            + if is_integrated {
                                dxgi_desc.SharedSystemMemory as u64
                            } else {
                                0
                            };
                        let architecture = vendor_to_arch(vendor_id).to_string();
                        (name, vendor_id, device_id, total_memory, architecture)
                    } else {
                        (format!("Adapter {}", i), 0, 0, 0, "Unknown".to_string())
                    };

                adapters_list.push(AdapterData {
                    name,
                    vendor_id,
                    device_id,
                    is_hardware,
                    is_integrated,
                    adapter_index: i,
                    driver_version,
                    total_memory,
                    architecture,
                });
            }
        }

        Ok(adapters_list)
    }
}
