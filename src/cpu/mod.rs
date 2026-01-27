use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub struct CPUUsage;

#[derive(Debug, Serialize, Deserialize)]
pub struct CPUData {
    pub name: String,
    pub vendor: CPUVendor,
    pub architecture: CPUArchitecture,
    pub num_of_cores: u32,
    pub logical_processors: u32,
    pub instruction_sets: Vec<CpuFeatureSet>,
    pub average_cpu_usage: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CpuFeatureSet {
    // ===== SSE FAMILY =====
    Sse,
    Sse2,
    Sse3,
    Ssse3,
    Sse41,
    Sse42,
    Sse4a,

    // ===== AVX FAMILY =====
    Avx,
    Avx2,

    // ===== AVX-512 CORE =====
    Avx512F,
    Avx512Cd,
    Avx512Pf,
    Avx512Er,
    Avx512Bw,
    Avx512Dq,
    Avx512Vl,

    // ===== AVX-512 EXTENSIONS =====
    Avx512Ifma,
    Avx512Vbmi,
    Avx512Vpopcntdq,
    Avx512Vbmi2,
    Avx512Vnni,
    Avx512Bitalg,
    Avx512Bf16,
    Avx512Vp2Intersect,
    Avx512Fp16,

    // ===== AVX EXTENSIONS (NON-512) =====
    AvxVnni,
    AvxIfma,
    AvxNeConvert,
    AvxVnniInt8,
    AvxVnniInt16,
}

impl CpuFeatureSet {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn is_supported_x86(&self) -> bool {
        use std::is_x86_feature_detected;

        match self {
            CpuFeatureSet::Sse => is_x86_feature_detected!("sse"),
            CpuFeatureSet::Sse2 => is_x86_feature_detected!("sse2"),
            CpuFeatureSet::Sse3 => is_x86_feature_detected!("sse3"),
            CpuFeatureSet::Ssse3 => is_x86_feature_detected!("ssse3"),
            CpuFeatureSet::Sse41 => is_x86_feature_detected!("sse4.1"),
            CpuFeatureSet::Sse42 => is_x86_feature_detected!("sse4.2"),
            CpuFeatureSet::Sse4a => is_x86_feature_detected!("sse4a"),

            CpuFeatureSet::Avx => is_x86_feature_detected!("avx"),
            CpuFeatureSet::Avx2 => is_x86_feature_detected!("avx2"),

            CpuFeatureSet::Avx512F => is_x86_feature_detected!("avx512f"),
            CpuFeatureSet::Avx512Cd => is_x86_feature_detected!("avx512cd"),
            CpuFeatureSet::Avx512Pf => is_x86_feature_detected!("avx512pf"),
            CpuFeatureSet::Avx512Er => is_x86_feature_detected!("avx512er"),
            CpuFeatureSet::Avx512Bw => is_x86_feature_detected!("avx512bw"),
            CpuFeatureSet::Avx512Dq => is_x86_feature_detected!("avx512dq"),
            CpuFeatureSet::Avx512Vl => is_x86_feature_detected!("avx512vl"),

            CpuFeatureSet::Avx512Ifma => is_x86_feature_detected!("avx512ifma"),
            CpuFeatureSet::Avx512Vbmi => is_x86_feature_detected!("avx512vbmi"),
            CpuFeatureSet::Avx512Vpopcntdq => {
                is_x86_feature_detected!("avx512vpopcntdq")
            }
            CpuFeatureSet::Avx512Vbmi2 => is_x86_feature_detected!("avx512vbmi2"),
            CpuFeatureSet::Avx512Vnni => is_x86_feature_detected!("avx512vnni"),
            CpuFeatureSet::Avx512Bitalg => is_x86_feature_detected!("avx512bitalg"),
            CpuFeatureSet::Avx512Bf16 => is_x86_feature_detected!("avx512bf16"),
            CpuFeatureSet::Avx512Vp2Intersect => {
                is_x86_feature_detected!("avx512vp2intersect")
            }
            CpuFeatureSet::Avx512Fp16 => is_x86_feature_detected!("avx512fp16"),

            CpuFeatureSet::AvxVnni => is_x86_feature_detected!("avxvnni"),
            CpuFeatureSet::AvxIfma => is_x86_feature_detected!("avxifma"),
            CpuFeatureSet::AvxNeConvert => is_x86_feature_detected!("avxneconvert"),
            CpuFeatureSet::AvxVnniInt8 => is_x86_feature_detected!("avxvnniint8"),
            CpuFeatureSet::AvxVnniInt16 => is_x86_feature_detected!("avxvnniint16"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CPUVendor {
    Intel,
    AMD,
    Apple,
    Qualcomm,
    Nvidia,
    // Virtual Machines
    VirtualPC,
    Rosetta2,
    Other,
}

impl CPUVendor {
    /// Maps vendor ID strings to CPUVendor enum variants
    pub fn from_vendor_id(vendor_id: &str) -> Self {
        match vendor_id {
            "GenuineIntel" => CPUVendor::Intel,
            "AuthenticAMD" => CPUVendor::AMD,
            "Apple" => CPUVendor::Apple, // For Apple Silicon
            "Qualcomm" => CPUVendor::Qualcomm,
            "Nvidia" => CPUVendor::Nvidia,
            // Virtual Machines
            "ConnectixCPU" | "Virtual CPU " | "Microsoft Hv" => CPUVendor::VirtualPC,
            _ => CPUVendor::Other,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CPUArchitecture {
    Arm,
    Arm64,
    I386,
    X86_64,
    RiscV32,
    RiscV64,
    Unknown,
}
