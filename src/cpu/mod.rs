#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct CPUUsage;

#[derive(Debug)]
pub struct CPUData {
    name: String,
    architecture: String,
    num_of_cores: u32,
    average_cpu_usage: f32,
}

#[allow(unused_variables)]
pub trait ICPU {
    fn get_cpu_info() -> Result<CPUData, String>;
    fn num_of_cores() -> u32;
    fn average_usage() -> f32;
}