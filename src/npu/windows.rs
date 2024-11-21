use crate::npu::{NPUData, NPUUsage};

impl NPUUsage {
    fn get_npu_info() -> Result<NPUData, String> {
        Ok(NPUData {
            name: "NPU".to_string(),
            capability: 50.0,
            usage: 0.0,
        })
    }
    fn total_npu_capability() -> f32 {
        // get soc details so we can get the npu capability
        // check if we are on intel
        // let platform = get_platform();
    }
    fn current_npu_usage() -> f32 {
        0.0
    }
}
