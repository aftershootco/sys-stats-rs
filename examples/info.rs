
use mem_info::GPUStats;
use mem_info::MemoryStats;
use mem_info::CPUStats;

fn main() {

    let gpus = GPUStats::get_gpus_list();
    
    for gpu in gpus.iter(){
        println!("----------------------");
        println!("{:#?}", gpu);
        println!("----------------------");
    }
    
    let memory_stats = MemoryStats::get_system_memory_info().unwrap();
    println!("----------------------");
    println!("{:#?}", memory_stats);
    println!("----------------------");
    
    let cpu_stats = CPUStats::get_cpu_info().unwrap();
    println!("----------------------");
    println!("{:#?}", cpu_stats);
    println!("----------------------");
}
