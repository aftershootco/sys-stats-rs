use crate::npu::{NPUData, NPUUsage};
use std::process::Command;

impl NPUUsage {
    fn get_architecture() -> String {
        // get the architecture of the device on linux
        let output = Command::new("sh")
            .arg("-c")
            .arg("uname -m")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8_lossy(&output.stdout);
        name.trim().to_string()
    }

    pub fn is_npu_available() -> bool {
        false
    }

    pub fn get_npu_info() -> Result<NPUData, String> {
        Err("NPU not available".to_string())
    }
    pub fn total_npu_capability() -> f32 {
        0.0
    }
}
