#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub struct GPUUsage;

#[derive(Debug, Clone)]
pub struct GPUData {
    pub name: String,
    pub architecture: String,
    pub total_memory: u64,
    pub free_memory: u64,
    /// Used memory also includes cpu memory, in unified memory systems
    pub used_memory: u64,
    pub has_unified_memory: bool,
    pub adapter_index: u32,
}

impl GPUData {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            architecture: "".to_string(),
            total_memory: 0,
            free_memory: 0,
            used_memory: 0,
            has_unified_memory: false,
            adapter_index: 0,
        }
    }

    pub fn new_with_values(
        name: String,
        architecture: String,
        total_memory: u64,
        free_memory: u64,
        used_memory: u64,
        has_unified_memory: bool,
        adapter_index: u32,
    ) -> Self {
        Self {
            name,
            architecture,
            total_memory,
            free_memory,
            used_memory,
            has_unified_memory,
            adapter_index,
        }
    }
}
