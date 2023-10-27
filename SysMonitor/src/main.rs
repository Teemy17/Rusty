use SysMonitor::{display_ram_usage, display_system_information, display_cpu_usage, display_network_usage
                , display_each_cpu_usage2
                };

fn main() {
    display_ram_usage()
}
                




// use sysinfo::{ProcessExt, System, SystemExt};
// use std::time::Duration;

// fn main() {
//     let mut system = System::new_all();

//     loop {
//         system.refresh_all();

//         println!("Total memory: {} KB", system.total_memory());
//         println!("Free memory : {} KB", system.free_memory());
//         println!("Used memory : {} KB", system.used_memory());

        // println!("Total swap  : {} KB", system.total_swap());
        // println!("Free swap   : {} KB", system.free_swap());
        // println!("Used swap   : {} KB", system.used_swap());

//         println!("Processes: {}", system.processes().len());
     

 

//         std::thread::sleep(Duration::from_secs(1));
//     }
// }
