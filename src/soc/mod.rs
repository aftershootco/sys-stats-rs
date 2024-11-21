use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
struct CoreConfig {
    p: u32,
    e: u32,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Cache {
//     size: u32,
//     level: u32,
// }

#[derive(Serialize, Deserialize, Debug)]
struct Core {
    name: Option<String>,
    architecture: Option<String>,
    frequency: u32,
    // cache: Vec<Cache>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CpuInfo {
    heterogeneous: bool,
    num_of_cores: u32,
    core_config: CoreConfig,
    p_core_data: Core,
    e_core_data: Core,
}

#[derive(Serialize, Deserialize, Debug)]
struct GpuInfo {
    name: Option<String>,
    architecture: Option<String>,
    num_of_cores: Option<u32>,
    frequency: Option<u32>,
    performance: Option<f32>, // in TFLOPS
}

#[derive(Serialize, Deserialize, Debug)]
struct NpuInfo {
    name: Option<String>,
    cores: Option<u32>,
    performance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PowerInfo {
    max_soc_power: Option<u32>,
    cpu_power: Option<u32>,
    gpu_power: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn new(
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
}

pub struct SocDetails;

impl SocDetails {
    pub fn get_soc_info_by_name(name: &str) -> Soc {
        // parse the soc.json file and return the Soc struct

        let s = include_str!("db/apple/soc.json");
        let soc_details: Result<SocCollection, Error> = serde_json::from_str(s);

        match soc_details {
            Ok(soc) => {
                for s in soc.0 {
                    if s.name == Some(name.to_string()) {
                        return s;
                    }
                }
                Soc::new(None, None, None, 0, None, None, None, None, 0)
            }
            Err(_) => Soc::new(None, None, None, 0, None, None, None, None, 0),
        }
    }
}
