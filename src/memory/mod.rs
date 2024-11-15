
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct MemoryUsage;

#[derive(Debug)]
pub struct MemoryData {
    total: u64,
    free: u64,
    /// Used memory also includes cpu memory, in unified memory systems
    used: u64,
}

impl MemoryData {
    pub(crate) fn to_bytes(&self) -> MemoryData {
        MemoryData {
            total: self.total * 1024,
            free: self.free * 1024,
            used: self.used * 1024,
        }
    }
}

pub trait IMemory {
    fn get_system_memory_info() -> Result<MemoryData, String>;

    fn total_system_memory() -> u64;
    fn current_system_memory_usage() -> u64;
    fn current_system_memory_free() -> u64;

    fn current_system_memory_swap() -> bool;

    fn has_unified_memory() -> bool;
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
        Self {
            total,
            free,
            used,
        }
    }
}