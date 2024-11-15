// Module: lib
mod cpu;
mod gpu;
mod memory;
mod npu;

pub use crate::cpu::CPUUsage as CPUStats;
pub use crate::gpu::GPUUsage as GPUStats;
pub use crate::memory::MemoryUsage as MemoryStats;
