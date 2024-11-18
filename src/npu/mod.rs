use serde::Serialize;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub struct NPUUsage;

#[derive(Debug, Serialize)]
pub struct NPUData {
    name: String,
    usage: f32,
    capability: f32, // in TFLOPS
}

#[allow(unused_variables)]
pub trait INPU {
    fn get_npu_info() -> Result<NPUData, String>;
    fn total_npu_capability() -> f32;
    fn current_npu_usage() -> f32;
}
