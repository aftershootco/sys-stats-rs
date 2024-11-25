use serde::Serialize;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub struct NPUUsage;

#[derive(Debug, Serialize)]
pub struct NPUData {
    pub name: String,
    pub capability: f32, // in TFLOPS
}

impl NPUData {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            capability: 0.0,
        }
    }

    pub fn new_with_values(name: String, capability: f32) -> Self {
        Self { name, capability }
    }
}
