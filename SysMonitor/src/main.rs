mod widgets;
use widgets::memory::display_ram_usage;
use widgets::sys_information::display_system_information;
use widgets::network::display_network_usage;
use widgets::cpu::display_cpu_usage;
use clap::Parser;

/// Simple program to display system information
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Option to display memory usage
    #[arg(short, long)]
    memory: bool,

    /// Option to display system information
    #[arg(short, long)]
    system_info: bool,

    /// Option to display network usage
    #[arg(short, long)]
    network: bool,

    /// Option to display CPU usage
    #[arg(short, long)]
    cpu: bool,
}

fn main() {
    let args = Args::parse();
    
    if args.memory {
        display_ram_usage();
    }

    if args.system_info {
        display_system_information();
    }

    if args.network {
        display_network_usage();
    }

    if args.cpu {
        display_cpu_usage();
    }
}




