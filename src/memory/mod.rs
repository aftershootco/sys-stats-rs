#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct MemoryUsage;

#[allow(unused_variables)]
pub trait IMemory {
    fn get_system_memory_info() -> Result<MemoryData, String>;

    fn total_system_memory() -> u64;
    fn current_system_memory_usage() -> u64;
    fn current_system_memory_free() -> u64;

    fn current_system_memory_swap() -> bool;

    fn has_unified_memory() -> bool;
}

#[derive(Debug, Clone)]
pub struct MemoryData {
    pub total: u64,
    pub free: u64,
    /// Used memory also includes gpu memory, in unified memory systems
    pub used: u64,
}

impl MemoryData {
    pub fn new() -> Self {
        Self {
            total: 0,
            free: 0,
            used: 0,
        }
    }

    pub fn new_with_values(total: u64, free: u64, used: u64) -> Self {
        Self { total, free, used }
    }
}
