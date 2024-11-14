use mem_info::PlatformMemoryUsage;

fn main() {

    let gpu_info = PlatformMemoryUsage::get_gpus_list();
    
    for gpu in gpu_info.iter(){
        println!("----------------------");
        println!("{:#?}", gpu);
        println!("----------------------");
    }
}
