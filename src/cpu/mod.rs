#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct CPUUsage;

#[derive(Debug)]
pub struct CPUData {
    pub name: String,
    pub architecture: String,
    pub num_of_cores: u32,
    pub average_cpu_usage: f32,
}
