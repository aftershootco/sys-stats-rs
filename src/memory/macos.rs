use crate::memory::{MemoryData, MemoryUsage};

impl MemoryUsage{

    pub fn get_system_memory_info() -> Result<MemoryData, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(MemoryData::new_with_values(mem_info.total, mem_info.free, (mem_info.total - mem_info.free)).to_bytes())
    }
    pub fn total_system_memory() -> Result<u64, Box<dyn std::error::Error>> {
        let mem_info = sys_info::mem_info()?;
        Ok(mem_info.total * 1024 )
    }

    pub fn current_system_memory_usage() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.total - mem_info.free) * 1024 ) // convert to bytes
    }

    pub fn current_system_memory_free() -> Result<u64, Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.free) * 1024) // convert to bytes
    }

    pub fn current_system_memory_swap() -> Result<(u64, u64), Box<dyn std::error::Error>>  {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.swap_total * 1024 , mem_info.swap_free * 1024) ) // convert to bytes
    }
}