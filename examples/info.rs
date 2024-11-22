use sys_stats::SocDetails;
use sys_stats::{CPUStats, GPUStats, MemoryStats, NPUStats};

fn main() {
    let gpu = GPUStats::get_gpu_info().unwrap();
    println!("----------------------");
    println!("{:#?}", gpu);
    println!("----------------------");

    let memory_stats = MemoryStats::get_system_memory_info().unwrap();
    println!("----------------------");
    println!("{:#?}", memory_stats);
    println!("----------------------");

    let cpu_stats = CPUStats::get_cpu_info().unwrap();
    println!("----------------------");
    println!("{:#?}", cpu_stats);
    println!("----------------------");

    let socs = SocDetails::get_current_soc_info();
    println!("----------------------");
    println!("{:#?}", socs);
    println!("----------------------");

    println!("----------------------");
    println!("NPU avaliable : {}", NPUStats::is_npu_available());
    println!("NPU tops at: {} TOPS", NPUStats::total_npu_capability());
    println!("----------------------");
}
