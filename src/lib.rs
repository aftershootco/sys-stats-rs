#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub trait MemoryUsage {
    fn total_gpu_memory() -> u64;
    fn max_gpu_memory() -> u64;
    fn current_gpu_memory_usage() -> u64;
    fn has_unified_memory() -> bool;
}

#[cfg(target_os = "macos")]
pub use macos::MacMemoryUsage as PlatformMemoryUsage;

#[cfg(target_os = "windows")]
pub use windows::WindowsMemoryUsage as PlatformMemoryUsage;