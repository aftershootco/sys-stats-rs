#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub struct GPUUsage;

#[derive(Debug, Clone)]
pub struct DriverVersionData {
    pub major: u64,
    pub minor: u64,
    pub build: u64,
    pub revision: u64,
}

#[derive(Debug, Clone)]
pub struct GPUData {
    pub name: String,
    pub architecture: String,
    pub vendor_id: u32,
    pub total_memory: u64,
    pub free_memory: u64,
    /// Used memory also includes cpu memory, in unified memory systems
    pub used_memory: u64,
    pub has_unified_memory: bool,
    pub is_integrated: bool,
    pub adapter_index: u32,
    pub driver_version: DriverVersionData,
}

impl GPUData {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            architecture: "".to_string(),
            vendor_id: 0,
            total_memory: 0,
            free_memory: 0,
            used_memory: 0,
            has_unified_memory: false,
            is_integrated:false,
            adapter_index: 0,
            driver_version: DriverVersionData {
                major: 0,
                minor: 0,
                build: 0,
                revision: 0,
            },
        }
    }

    pub fn new_with_values(
        name: String,
        architecture: String,
        vendor_id: u32,
        total_memory: u64,
        free_memory: u64,
        used_memory: u64,
        has_unified_memory: bool,
        is_integrated: bool,
        adapter_index: u32,
        driver_version: DriverVersionData,
    ) -> Self {
        Self {
            name,
            architecture,
            vendor_id,
            total_memory,
            free_memory,
            used_memory,
            has_unified_memory,
            is_integrated,
            adapter_index,
            driver_version,
        }
    }
    pub fn is_high_memory_dedicated(&self) -> bool {
        // Consider GPU as high memory if it has 4GB (4 * 1024 * 1024 * 1024 bytes) or more
        let four_gb = 4 * 1024 * 1024 * 1024;
        !self.is_integrated && self.total_memory >= four_gb
    }
}