use objc2_metal::MTLCreateSystemDefaultDevice;
use mem_info::PlatformMemoryUsage;


fn main() {


    println!("----------------------");

    // gpu info
    let gpu_info = PlatformMemoryUsage::get_gpu_info().unwrap();
    println!("GPU Info: {:?}", gpu_info);


    println!("----------------------");

    // Get the total gpu memory of the system
    println!("total_gpu_memory: {:?} MB", PlatformMemoryUsage::total_gpu_memory());

    // Get the current allocated gpu memory
    println!("current_gpu_memory_usage: {:?} MB", PlatformMemoryUsage::current_gpu_memory_usage());

    // Get the current free gpu memory
    println!("current_gpu_memory_free: {:?} MB", PlatformMemoryUsage::current_gpu_memory_free());


    println!("----------------------");

    // Check if the system has unified memory
    println!("has_unified_memory: {:?} MB", PlatformMemoryUsage::has_unified_memory());

    println!("----------------------");

    // Get the total cpu memory of the system
    println!("total_cpu_memory: {:?} MB", PlatformMemoryUsage::total_cpu_memory());

    // Get the current allocated cpu memory
    println!("current_cpu_memory_usage: {:?}", PlatformMemoryUsage::current_cpu_memory_usage());

    
    println!("Free memory: {:?}", PlatformMemoryUsage::current_cpu_memory_free());

    println!("----------------------");


    println!("swap_memory: {:?}", PlatformMemoryUsage::current_cpu_memory_swap());

    println!("----------------------");
   

}
