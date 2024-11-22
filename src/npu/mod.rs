use serde::Serialize;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub struct NPUUsage;

#[derive(Debug, Serialize)]
pub struct NPUData {
    pub name: String,
    pub usage: f32,
    pub capability: f32, // in TFLOPS
}

#[allow(unused_variables)]
pub trait INPU {
    fn is_npu_available() -> bool;
    fn get_npu_info() -> Result<NPUData, String>;
    fn total_npu_capability() -> f32;
    fn current_npu_usage() -> f32;
}

impl NPUData {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            usage: 0.0,
            capability: 0.0,
        }
    }

    pub fn new_with_values(name: String, usage: f32, capability: f32) -> Self {
        Self {
            name,
            usage,
            capability,
        }
    }
}
