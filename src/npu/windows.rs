use crate::npu::{NPUData, NPUUsage};

impl NPUUsage {
    pub fn is_npu_available() -> bool {
        false
    }

    pub fn get_npu_info() -> Result<NPUData, String> {
        Ok(NPUData {
            name: "NPU".to_string(),
            capability: 50.0,
            usage: 0.0,
        })
    }

    pub fn total_npu_capability() -> f32 {
        // get soc details so we can get the npu capability
        0.0
    }

    pub fn current_npu_usage() -> f32 {
        0.0
    }
}
