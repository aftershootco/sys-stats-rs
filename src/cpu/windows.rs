use crate::gpu::{CPUData, CPUUsage};

use winapi::shared::winerror::FAILED;
use std::ptr;
use anyhow::Result;

impl CPUUsage{
    pub fn get_cpu_info() -> Result<CPUData, String> {
        let mut cpu_info = CPUData::new();
        Ok(cpu_info)
    }
    
    pub fn num_of_cores() -> u32 {
        0
    }
    
    pub fn average_usage() -> f32 {
        0.0
    }
    
}