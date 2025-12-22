use sysinfo::{MemoryRefreshKind, RefreshKind};

use crate::memory::{MemoryData, MemoryUsage};

impl MemoryUsage {
    pub fn get_system_memory_info() -> Result<MemoryData, Box<dyn std::error::Error>> {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
        );

        Ok(MemoryData {
            total: system.total_memory(),
            free: system.free_memory(),
            used: system.used_memory(),
        })
    }
    pub fn total_system_memory() -> Result<u64, Box<dyn std::error::Error>> {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
        );

        Ok(system.total_memory())
    }

    pub fn current_system_memory_usage() -> Result<u64, Box<dyn std::error::Error>> {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
        );

        Ok(system.used_memory())
    }

    pub fn current_system_memory_free() -> Result<u64, Box<dyn std::error::Error>> {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
        );

        Ok(system.free_memory())
    }

    pub fn current_system_memory_swap() -> Result<(u64, u64), Box<dyn std::error::Error>> {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_swap()),
        );

        Ok((system.used_swap(), system.total_swap()))
    }
}
