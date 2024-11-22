#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct GPUUsage;

#[allow(unused_variables)]
pub trait IGPU {
    fn get_gpu_info() -> Result<GPUData, String>;

    fn get_gpus_list() -> Result<Vec<GPUData>, String>;
    fn total_gpu_memory() -> u64;
    fn current_gpu_memory_usage() -> u64;
    fn current_gpu_memory_free() -> u64;

    fn has_unified_memory() -> bool;
}

#[derive(Debug, Clone)]
pub struct GPUData {
    pub name: String,
    pub architecture: String,
    pub total_memory: u64,
    pub free_memory: u64,
    /// Used memory also includes cpu memory, in unified memory systems
    pub used_memory: u64,
    pub has_unified_memory: bool,
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
        }
    }

    pub fn new_with_values(
        name: String,
        architecture: String,
        total_memory: u64,
        free_memory: u64,
        used_memory: u64,
        has_unified_memory: bool,
    ) -> Self {
        Self {
            name,
            architecture,
            total_memory,
            free_memory,
            used_memory,
            has_unified_memory,
        }
    }
}
