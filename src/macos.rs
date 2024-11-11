// src/macos.rs
use objc2::class;
use objc2::{msg_send};
use objc2::runtime::{AnyObject};
use objc2_metal::{MTLCreateSystemDefaultDevice};

pub struct MacMemoryUsage;

impl MacMemoryUsage {
    pub fn total_gpu_memory() -> u64 {
        unsafe {
            let process_info: *mut AnyObject = msg_send![class!(NSProcessInfo), processInfo];
            msg_send![process_info, physicalMemory]
        }
    }

    pub fn max_gpu_memory() -> u64 {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, recommendedMaxWorkingSetSize]
        }
    }

    pub fn current_gpu_memory_usage() -> u64 {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, currentAllocatedSize]
        }
    }

    pub fn has_unified_memory() -> bool {
        unsafe {
            let mtl_device = MTLCreateSystemDefaultDevice();
            msg_send![mtl_device, hasUnifiedMemory]
        }
    }
}