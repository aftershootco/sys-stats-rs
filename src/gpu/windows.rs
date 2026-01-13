use crate::gpu::{AdapterData, DriverVersionData, GPUData, GPUUsage};
use anyhow::{Result, anyhow};
use std::ffi::c_void;

use windows::{
    Win32::{
        Foundation::*,
        Graphics::{
            DXCore::*,
            Dxgi::{
                CreateDXGIFactory2, DXGI_ADAPTER_DESC1, DXGI_CREATE_FACTORY_FLAGS,
                DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE, IDXGIAdapter1, IDXGIFactory6,
            },
        },
        System::Performance::*,
    },
    core::*,
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
    let instance: String = luid_to_string(luid);
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

unsafe fn enumerate_dxgi_adapters() -> Vec<(LUID, IDXGIAdapter1, DXGI_ADAPTER_DESC1)> {
    let mut list = Vec::new();
    let factory: IDXGIFactory6 = CreateDXGIFactory2(DXGI_CREATE_FACTORY_FLAGS(0)).unwrap();

    let mut i = 0u32;
    loop {
        match factory
            .EnumAdapterByGpuPreference::<IDXGIAdapter1>(i, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE)
        {
            Ok(adapter) => {
                if let Ok(desc) = adapter.GetDesc1() {
                    list.push((desc.AdapterLuid, adapter, desc));
                } else {
                    list.push((
                        LUID {
                            LowPart: 0,
                            HighPart: 0,
                        },
                        adapter,
                        DXGI_ADAPTER_DESC1::default(),
                    ));
                }
            }
            Err(_) => break,
        }
        i += 1;
    }
    list
}

unsafe fn get_bool_prop(adapter: &IDXCoreAdapter, prop: DXCoreAdapterProperty) -> bool {
    let mut buf = [0u8; 4];
    adapter
        .GetProperty(prop, 4, buf.as_mut_ptr() as *mut c_void)
        .is_ok()
        && u32::from_le_bytes(buf) != 0
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
            let dxgi_list = enumerate_dxgi_adapters();

            let factory: IDXCoreAdapterFactory = DXCoreCreateAdapterFactory()?;
            let attrs = [DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE];
            let list: IDXCoreAdapterList = factory.CreateAdapterList(&attrs)?;
            let count = list.GetAdapterCount();

            for i in 0..count {
                let adapter: IDXCoreAdapter = match list.GetAdapter(i) {
                    Ok(a) => a,
                    Err(_) => continue,
                };

                if !get_bool_prop(&adapter, IsHardware) {
                    continue;
                }

                let is_integrated = get_bool_prop(&adapter, IsIntegrated);

                // Driver Version
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

                let mut luid_buf = [0u8; 8];
                if adapter
                    .GetProperty(InstanceLuid, 8, luid_buf.as_mut_ptr() as *mut c_void)
                    .is_err()
                {
                    continue;
                }

                let adapter_luid: LUID = std::ptr::read(luid_buf.as_ptr() as *const _);

                // Find matching DXGI adapter by LUID
                let dxgi_match = dxgi_list.iter().find(|(luid, _dxgi_adapter, _desc)| {
                    luid.LowPart == adapter_luid.LowPart && luid.HighPart == adapter_luid.HighPart
                });

                if let Some((dxgi_luid, _dxgi_adapter, dxgi_desc)) = dxgi_match {
                    let name = wide_to_string(&dxgi_desc.Description);
                    let vendor = dxgi_desc.VendorId;

                    let dedicated_total = dxgi_desc.DedicatedVideoMemory as u64;
                    let shared_total = dxgi_desc.SharedSystemMemory as u64;

                    let (ded_used, shared_used, _commit) =
                        get_gpu_pdh_memory(*dxgi_luid).unwrap_or((0, 0, 0));

                    let dedicated_free = dedicated_total.saturating_sub(ded_used);
                    let shared_free = shared_total.saturating_sub(shared_used);

                    let total_memory = if is_integrated {
                        dedicated_total + shared_total
                    } else { dedicated_total };
                    let used_memory = if is_integrated {
                        ded_used + shared_used
                    } else { ded_used };
                    let free_memory = total_memory - used_memory;

                    gpus.push(GPUData {
                        name: name.clone(),
                        architecture: vendor_to_arch(vendor).to_string(),
                        vendor_id: vendor,
                        total_memory,
                        free_memory,
                        used_memory,
                        has_unified_memory: is_integrated,
                        is_integrated,
                        adapter_index: i,
                        driver_version: driver,
                    });
                } else {
                    let name: String = format!("DXCore Adapter {}", i);

                    gpus.push(GPUData {
                        name: name.clone(),
                        architecture: "Unknown".to_string(),
                        vendor_id: 0,
                        total_memory: 0,
                        free_memory: 0,
                        used_memory: 0,
                        has_unified_memory: is_integrated,
                        is_integrated,
                        adapter_index: i,
                        driver_version: driver,
                    });

                    println!("GPU (adapter_index {}): {} (no DXGI match)", i, name);
                }
            }
        }

        // Sort the GPUs
        gpus.sort_by(|a, b| {
            match (a.is_high_memory_dedicated(), b.is_high_memory_dedicated()) {
                // If both are high memory dedicated or both are not, keep original order
                (true, true) | (false, false) => std::cmp::Ordering::Equal,
                // If a is high memory dedicated but b is not, a comes first
                (true, false) => std::cmp::Ordering::Less,
                // If b is high memory dedicated but a is not, b comes first
                (false, true) => std::cmp::Ordering::Greater,
            }
        });

        Ok(gpus)
    }

    /// Get list of all adapters in the system (hardware and non-hardware)
    pub fn get_all_adapters_list() -> Result<Vec<AdapterData>, Box<dyn std::error::Error>> {
        let mut adapters_list = Vec::new();

        unsafe {
            // Enumerate DXGI adapters first
            let dxgi_list = enumerate_dxgi_adapters();

            let factory: IDXCoreAdapterFactory = DXCoreCreateAdapterFactory()?;
            let attrs = [DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE];
            let list: IDXCoreAdapterList = factory.CreateAdapterList(&attrs)?;
            let count = list.GetAdapterCount();

            for i in 0..count {
                let adapter: IDXCoreAdapter = match list.GetAdapter(i) {
                    Ok(a) => a,
                    Err(_) => continue,
                };

                let is_hardware = {
                    let mut buf = [0u8; 4];
                    adapter
                        .GetProperty(IsHardware, 4, buf.as_mut_ptr() as *mut _)
                        .is_ok()
                        && u32::from_le_bytes(buf) != 0
                };

                let is_integrated = {
                    let mut buf = [0u8; 4];
                    adapter
                        .GetProperty(IsIntegrated, 4, buf.as_mut_ptr() as *mut _)
                        .is_ok()
                        && u32::from_le_bytes(buf) != 0
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

                // Get AdapterLuid for matching
                let mut luid_buf = [0u8; 8];
                let adapter_luid = if adapter
                    .GetProperty(InstanceLuid, 8, luid_buf.as_mut_ptr() as *mut c_void)
                    .is_ok()
                {
                    std::ptr::read(luid_buf.as_ptr() as *const _)
                } else {
                    LUID {
                        LowPart: 0,
                        HighPart: 0,
                    }
                };

                // Try to find DXGI descriptor for richer info
                let dxgi_match = dxgi_list.iter().find(|(luid, _dxgi_adapter, _desc)| {
                    luid.LowPart == adapter_luid.LowPart && luid.HighPart == adapter_luid.HighPart
                });

                let (name, vendor_id, device_id, total_memory, architecture) =
                    if let Some((_luid, _dxgi_adapter, dxgi_desc)) = dxgi_match {
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
