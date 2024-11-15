// Module: lib
mod cpu;
pub mod gpu;
mod memory;
mod npu;

pub use crate::gpu::GPUUsage as GPUStats;
pub use crate::memory::MemoryUsage as MemoryStats;
pub use crate::cpu::CPUUsage as CPUStats;

