use std::env;
use std::io::{self, Write};
use sys_stats::SocDetails;
use sys_stats::{CPUStats, GPUStats, MemoryStats};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut show_all_gpus = args.iter().any(|arg| arg == "--all");

    // If no command line arguments provided, show interactive prompt
    if args.len() == 1 {
        println!("System Statistics Tool (Example)");
        println!("===============================");
        println!("1. Show default info (selected GPU only)");
        println!("2. Show full info (all GPUs)");
        print!("Choose option (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => show_all_gpus = false,
            "2" => show_all_gpus = true,
            _ => {
                println!("Invalid option. Using default (selected GPU only).");
                show_all_gpus = false;
            }
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

    let gpu_data = GPUStats::get_gpus_list();

    if show_all_gpus {
        println!("All GPUs:");
        match gpu_data {
            Ok(data) => {
                if data.is_empty() {
                    eprintln!("No GPUs detected in system");
                    eprintln!("Possible reasons:");
                    eprintln!("  - No graphics drivers installed");
                    eprintln!("  - Graphics drivers not compatible");
                    eprintln!("  - System has no graphics hardware");
                    eprintln!("  - Running in virtual machine without GPU passthrough");
                } else {
                    for (index, gpu) in data.iter().enumerate() {
                        println!("----------------------");
                        if index == 0 {
                            println!("[SELECTED GPU]");
                        }
                        println!("{:#?}", gpu);
                        println!("----------------------");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to get GPU list: {:?}", e);
                eprintln!("Error details:");
                if e.to_string().contains("DXCore") {
                    eprintln!("  - DXCore initialization failed");
                    eprintln!("  - DirectX runtime may not be installed");
                    eprintln!("  - Windows version may not support DXCore");
                } else if e.to_string().contains("adapter") {
                    eprintln!("  - No graphics adapters found");
                    eprintln!("  - Graphics drivers may not be installed");
                } else if e.to_string().contains("NVML") {
                    eprintln!("  - NVIDIA Management Library failed");
                    eprintln!("  - NVIDIA drivers may not be installed");
                } else {
                    eprintln!("  - Unknown GPU detection error");
                }
            }
        }

        // Show adapters list only when option 2 is selected
        match GPUStats::get_all_adapters_list() {
            Ok(info) => {
                if info.is_empty() {
                    println!("No adapters found in system");
                } else {
                    println!("Adapters:");
                    println!("{:#?}", info);
                }
            }
            Err(e) => {
                eprintln!("Failed to get adapters list: {:?}", e);
            }
        };
    } else {
        match GPUStats::get_gpu_info() {
            Ok(info) => {
                println!("Selected GPU: ");
                println!("{:#?}", info);
                println!("----------------------");
            }
            Err(e) => {
                eprintln!("Failed to get GPU info: {:?}", e);
                eprintln!("Error details:");
                if e.to_string().contains("No GPU found") {
                    eprintln!("  - No graphics hardware detected");
                    eprintln!("  - Graphics drivers may not be installed");
                    eprintln!("  - System may be running headless");
                } else if e.to_string().contains("DXCore") {
                    eprintln!("  - DirectX runtime issues");
                    eprintln!("  - Windows version compatibility problem");
                } else {
                    eprintln!("  - Unknown GPU detection error");
                }
            }
        };
    }

    let socs = SocDetails::get_current_soc_info();
    println!("----------------------");
    println!("{:#?}", socs);
    println!("----------------------");

    // Wait for key press before exiting
    if args.len() == 1 {
        println!("\nPress any key to exit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}
