#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Debug)]
pub struct GPUInfo {
    name: String,
    architecture: String,
    total_memory: u64,
    free_memory: u64,
    used_memory: u64,
    has_unified_memory: bool,
}

impl GPUInfo {
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
    
    pub fn new_with_values(name: String, architecture: String, total_memory: u64, free_memory: u64, used_memory: u64, has_unified_memory: bool) -> Self {
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

pub trait MemoryUsage {
    fn get_gpu_info() -> Result<GPUInfo, String>;
    
    fn get_gpus_list() -> Result<Vec<GPUInfo>, String>;
    fn total_gpu_memory() -> u64;
    fn current_gpu_memory_usage() -> u64;
    fn current_gpu_memory_free() -> u64;

    fn has_unified_memory() -> bool;

    fn total_cpu_memory() -> u64;
    fn current_cpu_memory_usage() -> u64;
    fn current_cpu_memory_free() -> u64;
}

#[cfg(target_os = "macos")]
pub use macos::MacMemoryUsage as PlatformMemoryUsage;

#[cfg(target_os = "windows")]
pub use windows::WindowsMemoryUsage as PlatformMemoryUsage;