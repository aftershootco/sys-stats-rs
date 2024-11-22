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

    fn get_npu_info() -> Result<NPUData, String> {
        Ok(NPUData {
            name: "NPU".to_string(),
            capability: 50.0,
            usage: 0.0,
        })
    }
    pub fn total_npu_capability() -> f32 {
        // get the soc details

        let output = Command::new("sh")
            .arg("-c")
            .arg("sysctl -n machdep.cpu.brand_string")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let soc = SocDetails::get_soc_info_by_name(&name);
        soc.npu_performance().unwrap()
    }
    pub fn current_npu_usage() -> f32 {
        0.0
    }
}
