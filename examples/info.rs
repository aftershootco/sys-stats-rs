use sys_stats::SocDetails;
use sys_stats::{CPUStats, GPUStats, MemoryStats, NPUStats};

fn main() {
    match GPUStats::get_gpu_info() {
        Ok(info) => {
            println!("----------------------");
            println!("{:#?}", info);
            println!("----------------------");
        }
        Err(e) => {
            eprintln!("Failed to get GPU info: {:?}", e);
        }
    };

    match MemoryStats::get_system_memory_info() {
        Ok(info) => {
            println!("----------------------");
            println!("{:#?}", info);
            println!("----------------------");
        }
        Err(e) => {
            eprintln!("Failed to get Memory info: {:?}", e);
        }
    }

    match CPUStats::get_cpu_info() {
        Ok(info) => {
            println!("----------------------");
            println!("{:#?}", info);
            println!("----------------------");
        }
        Err(e) => {
            eprintln!("Failed to get CPU info: {:?}", e);
        }
    }

    match NPUStats::get_npu_info() {
        Ok(info) => {
            println!("----------------------");
            println!("{:#?}", info);
            println!("----------------------");
        }
        Err(e) => {
            eprintln!("Failed to get NPU info: {:?}", e);
        }
    }

    let socs = SocDetails::get_current_soc_info();
    println!("----------------------");
    println!("{:#?}", socs);
    println!("----------------------");
}
