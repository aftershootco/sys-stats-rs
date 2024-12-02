use crate::npu::{NPUData, NPUUsage};
use crate::soc::SocDetails;
use std::process::Command;

impl NPUUsage {
    fn get_architecture() -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg("uname -m")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8_lossy(&output.stdout);
        name.trim().to_string()
    }

    pub fn is_npu_available() -> bool {
        // if we are on arm64, we can assume that the device has an NPU
        if Self::get_architecture() == "arm64" {
            true
        } else {
            false
        }
    }

    pub fn get_npu_info() -> Result<NPUData, String> {
        if Self::get_architecture() == "arm64" {
            return Ok(NPUData {
                name: "NPU".to_string(),
                capability: Self::total_npu_capability(),
            });
        }
        Err("NPU not available".to_string())
    }
    pub fn total_npu_capability() -> f32 {
        // get the soc details and return the NPU performance
        let soc = SocDetails::get_current_soc_info();
        // use if let to avoid panics
        if let Some(npu) = soc.npu_performance() {
            npu
        } else {
            0.0
        }
    }
}
