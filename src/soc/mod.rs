#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

use serde::{Deserialize, Serialize};

pub struct SocDetails;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoreConfig {
    p: u32,
    e: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Core {
    name: Option<String>,
    architecture: Option<String>,
    frequency: u32,
    // cache: Vec<Cache>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CpuInfo {
    heterogeneous: bool,
    num_of_cores: u32,
    pub core_config: CoreConfig,
    p_core_data: Core,
    e_core_data: Core,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GpuInfo {
    name: Option<String>,
    architecture: Option<String>,
    num_of_cores: Option<u32>,
    frequency: Option<u32>,
    performance: Option<f32>, // in TFLOPS
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NpuInfo {
    pub name: Option<String>,
    pub cores: Option<u32>,
    pub performance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PowerInfo {
    max_soc_power: Option<u32>,
    cpu_power: Option<u32>,
    gpu_power: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Soc {
    name: Option<String>,
    vendor: Option<String>,
    model: Option<String>,
    variant: u32,
    year: u32,
    cpu: Option<CpuInfo>,
    gpu: Option<GpuInfo>,
    npu: Option<NpuInfo>,
    power: Option<PowerInfo>,
    /// in bits
    bus_width: u32,
    /// in GB/s
    data_rate: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocCollection(Vec<Soc>);

impl Soc {
    fn new(
        name: Option<String>,
        vendor: Option<String>,
        model: Option<String>,
        year: u32,
        cpu: Option<CpuInfo>,
        gpu: Option<GpuInfo>,
        npu: Option<NpuInfo>,
        power: Option<PowerInfo>,
        bus_width: u32,
    ) -> Soc {
        Soc {
            name,
            vendor,
            model,
            variant: 0,
            year,
            cpu,
            gpu,
            npu,
            power,
            bus_width,
            data_rate: 0.0,
        }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn vendor(&self) -> Option<String> {
        self.vendor.clone()
    }

    pub fn model(&self) -> Option<String> {
        self.model.clone()
    }

    pub fn variant(&self) -> u32 {
        self.variant
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn is_heterogeneous(&self) -> bool {
        self.cpu.as_ref().unwrap().heterogeneous
    }

    pub fn num_of_cores(&self) -> u32 {
        self.cpu.as_ref().unwrap().num_of_cores
    }

    pub fn core_config(&self) -> CoreConfig {
        self.cpu.as_ref().unwrap().core_config.clone()
    }

    pub fn p_core_data(&self) -> Core {
        self.cpu.as_ref().unwrap().p_core_data.clone()
    }

    pub fn e_core_data(&self) -> Core {
        self.cpu.as_ref().unwrap().e_core_data.clone()
    }

    pub fn gpu_name(&self) -> Option<String> {
        self.gpu.as_ref().unwrap().name.clone()
    }

    pub fn gpu_architecture(&self) -> Option<String> {
        self.gpu.as_ref().unwrap().architecture.clone()
    }

    pub fn num_of_gpu_cores(&self) -> Option<u32> {
        self.gpu.as_ref().unwrap().num_of_cores
    }

    pub fn gpu_frequency(&self) -> Option<u32> {
        self.gpu.as_ref().unwrap().frequency
    }

    pub fn gpu_performance(&self) -> Option<f32> {
        self.gpu.as_ref().unwrap().performance
    }

    pub fn npu_name(&self) -> Option<String> {
        self.npu.as_ref().unwrap().name.clone()
    }

    pub fn num_of_npu_cores(&self) -> Option<u32> {
        self.npu.as_ref().unwrap().cores
    }

    pub fn npu_performance(&self) -> Option<f32> {
        self.npu.as_ref().unwrap().performance
    }

    pub fn max_soc_power(&self) -> Option<u32> {
        self.power.as_ref().unwrap().max_soc_power
    }

    pub fn cpu_power(&self) -> Option<u32> {
        self.power.as_ref().unwrap().cpu_power
    }

    pub fn gpu_power(&self) -> Option<u32> {
        self.power.as_ref().unwrap().gpu_power
    }

    /// Get the bus width in bits
    pub fn bus_width(&self) -> u32 {
        self.bus_width
    }

    /// Get the data rate in GB/s
    pub fn data_rate(&self) -> f32 {
        self.data_rate
    }
}
