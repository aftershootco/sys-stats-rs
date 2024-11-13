use mem_info::PlatformMemoryUsage;


fn main() {
    println!("----------------------");

    let gpu_info = PlatformMemoryUsage::get_gpu_info().unwrap();
    println!("{:#?}", gpu_info);

    println!("----------------------");
}
