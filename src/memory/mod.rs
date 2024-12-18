#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub struct MemoryUsage;

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
