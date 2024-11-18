// Module: lib
mod cpu;
mod gpu;
mod memory;
mod npu;
mod soc;

pub use crate::cpu::CPUUsage as CPUStats;
pub use crate::gpu::GPUUsage as GPUStats;
pub use crate::memory::MemoryUsage as MemoryStats;
pub use crate::npu::NPUUsage as NPUStats;
pub use crate::soc::SocDetails;
