use std::thread;
use std::time::Duration;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, Gauge, Chart, Dataset, Axis, GraphType};
use tui::layout::{Layout, Constraint, Direction};
use termion::raw::IntoRawMode; // Add this import for raw mode
use termion::screen::AlternateScreen; // Add this import for AlternateScreen
use sysinfo::System;
use sysinfo::{SystemExt, ProcessExt, NetworkExt};
use sysinfo::{CpuExt, RefreshKind, CpuRefreshKind};
use tui::text::{Text, Span, Spans};
use tui::widgets::Paragraph;
use tui::style::{Style, Color};
use tui::widgets::{Row, Table, Cell};
use std::io;


pub fn display_ram_usage() {
    // Initialize TUI terminal and backend
    let stdout = std::io::stdout().into_raw_mode().unwrap(); // Set terminal in raw mode
    let backend = TermionBackend::new(AlternateScreen::from(stdout)); // Use AlternateScreen
    let mut terminal = Terminal::new(backend).unwrap();

    // Create a gauge widget to display memory usage
    let ram_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
        .label("0%")
        .percent(0);

    let mut sys = System::new_all();

    // Loop to update the CPU usage gauge
    loop {
        sys.refresh_all();

        // Get the current CPU usage as a percentage
        let ram_usage = get_ram_usage();

        // Update the gauge widget with the new CPU usage percentage
        let ram_gauge = ram_gauge.clone().percent(ram_usage.into()).label(format!("{}%", ram_usage));

        let text = vec![
            format!("Total memory: {:.2} GB", convert_kb_to_gb(sys.total_memory())),
            format!("Free memory : {:.2} GB", convert_kb_to_gb(sys.free_memory())),
            format!("Used memory : {:.2} GB", convert_kb_to_gb(sys.used_memory())),
            format!("Total swap  : {:.2} GB", convert_kb_to_gb(sys.total_swap())),
            format!("Free swap   : {:.2} GB", convert_kb_to_gb(sys.free_swap())),
            format!("Used swap   : {:.2} GB", convert_kb_to_gb(sys.used_swap())),
        ];

        let rows: Vec<Row> = text.iter().map(|line| Row::new(vec![line.to_string()])).collect();

        let table = Table::new(rows)
            .block(Block::default().title("Memory Details").borders(Borders::ALL))
            .widths(&[Constraint::Percentage(100)]);


        // Draw the TUI layout with the CPU usage gauge
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(f.size());
            f.render_widget(ram_gauge, chunks[0]);
            f.render_widget(table, chunks[1]);
        }).unwrap();

        // Wait for a short period of time before updating the gauge again
        thread::sleep(Duration::from_millis(100));
    }
}

fn get_ram_usage() -> u8 {
    let mut sys = System::new_all();
    sys.refresh_all();
    let ram_time = sys.used_memory();
    let total_time = sys.total_memory();

    // Calculate CPU usage as a percentage
    let ram_usage = ((ram_time as f64 / total_time as f64) * 100.0) as u8;

    ram_usage
}

fn convert_kb_to_gb(kb: u64) -> f64 {
    kb as f64 / 1024.0 / 1024.0 / 1024.0
}

pub fn display_system_information() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Unwrap and print the system information with default values
    println!("System name:             {:?}", sys.name().unwrap_or("N/A".to_string()));
    println!("System kernel version:   {:?}", sys.kernel_version().unwrap_or("N/A".to_string()));
    println!("System OS version:       {:?}", sys.os_version().unwrap_or("N/A".to_string()));
    println!("System host name:        {:?}", sys.host_name().unwrap_or("N/A".to_string()));
}

pub fn display_system_information2() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new system instance
    let mut sys = System::new_all();
    sys.refresh_all();

    // Initialize TUI terminal and backend
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend)?;

    // Create a TUI loop to display system information
    loop {
        // Gather system information
        let system_name = sys.name().unwrap();
        let kernel_version = sys.kernel_version().unwrap();
        let os_version = sys.os_version().unwrap();
        let host_name = sys.host_name().unwrap();

        let system_info = Text::from(format!(
            "System name: {}\nKernel version: {}\nOS version: {}\nHost name: {}",
            system_name, kernel_version, os_version, host_name
        ));
        

        // Create a Paragraph widget to display the system information
        let paragraph = Paragraph::new(system_info)
        .block(Block::default().title("System Information").borders(Borders::ALL));

        // Clear the terminal and draw the TUI with the system information
        terminal.draw(|f| {
            // Clear the terminal
            

            // Render the Paragraph widget
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size()); // Use the terminal size

            f.render_widget(paragraph, chunks[0]);
            
        })?;
         
         
    }

}

pub fn display_cpu_usage() {

    let mut sys = System::new();

    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        for cpu in sys.cpus() {
            println!("{}% ", cpu.cpu_usage());
        }
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        thread::sleep(Duration::from_millis(100));
    }
}


    


pub fn display_network_usage() {
    // Initialize TUI terminal and backend
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend).unwrap();

    let mut sys = System::new_all();
    sys.refresh_all();

    let network_block = Block::default()
        .title("Network Usage")
        .borders(Borders::ALL);

    loop {
      
        sys.refresh_networks();
        let mut network_text: Vec<String> = Vec::new();

        for (interface_name, interface) in sys.networks() {
            let text = format!(
                "{}: {:.2} KB/s in, {:.2} KB/s out\n",
                interface_name,
                interface.received() as f64 / 1024.0,
                interface.transmitted() as f64 / 1024.0
            );
            network_text.push(text);
        }

        let network_block_clone = network_block.clone();

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size()); // Use the terminal size

                let network_paragraph = Paragraph::new(network_text.join(""))
                    .block(network_block_clone)
                    .style(Style::default().fg(Color::Blue))
                    .alignment(tui::layout::Alignment::Left);

                f.render_widget(network_paragraph, chunks[0]);
            })
            .unwrap();

        thread::sleep(Duration::from_millis(100));
    }
}

// pub fn display_each_cpu_usage() {

//         // Create a new system instance
//         let mut sys = System::new_all();

//         // Initialize TUI terminal and backend
//         let stdout = std::io::stdout().into_raw_mode().unwrap();
//         let backend = TermionBackend::new(stdout);
//         let mut terminal = Terminal::new(backend).unwrap();

//         // Main loop to continuously display CPU usage
//         loop {

//             terminal.clear().unwrap();
//             // Refresh CPU information
//             sys.refresh_cpu();

//             // Get the list of CPU cores
//             let cpu_list = sys.cpus();

//             // Create a vector of Text widgets to display CPU usage for each core
// // Create a vector of Text widgets to display CPU usage for each core
//         let cpu_text: Vec<Text> = cpu_list
//             .iter()
//             .enumerate()
//             .map(|(core_id, cpu)| {
//                 Text::raw(format!("Core {}: {:.2}%\n", core_id, cpu.cpu_usage()))
//             })
//             .collect();

//         // Create a vector of rows for the table, where each row is a paragraph
//         let cpu_rows: Vec<Row> = cpu_text.iter().map(|text| {
//             Row::new(vec![Cell::from(text.clone())])
//         }).collect();

//         // Create a table widget to display the CPU usage text
//         let table = Table::new(cpu_rows)
//             .block(Block::default().title("CPU Usage").borders(Borders::ALL))
//             .header(Row::new(vec![Cell::from("Usage")]))
//             .widths(&[Constraint::Percentage(100)]);

//         // Draw the TUI with the CPU usage text
//         terminal
//             .draw(|f| {
//                 let chunks = Layout::default()
//                     .direction(Direction::Vertical)
//                     .margin(0)
//                     .constraints([Constraint::Percentage(100)].as_ref())
//                     .split(f.size()); // Use the terminal size

//                 f.render_widget(table, chunks[0]);
//             })
//             .unwrap();


//             // Sleep for a while to control the refresh rate
//             thread::sleep(Duration::from_millis(100));
//         }
    
// }

pub fn display_each_cpu_usage() {
    // Create a new system instance
    let mut sys = System::new_all();

    // Create a separate terminal for TUI
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend).unwrap();

    // Main loop to continuously display CPU usage
    loop {
        // Refresh CPU information
        sys.refresh_cpu();

        // Get the list of CPU cores
        let cpu_list = sys.cpus();

        // Create a vector of Text widgets to display CPU usage for each core
        let cpu_text: Vec<Text> = cpu_list
            .iter()
            .enumerate()
            .map(|(core_id, cpu)| {
                Text::raw(format!("Core {}: {:.2}%\n", core_id, cpu.cpu_usage()))
            })
            .collect();

        // Create a table widget to display the CPU usage text
        let table = Table::new(
            cpu_text.iter().map(|text| {
                Row::new(vec![Cell::from(text.clone())])
            })
        )
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .header(Row::new(vec![Cell::from("Usage")]))
        .widths(&[Constraint::Percentage(100)]);

        // Clear the TUI terminal and draw the TUI with the CPU usage text
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size()); // Use the terminal size

            f.render_widget(table, chunks[0]);
        }).unwrap();

        // Sleep for a while to control the refresh rate
        thread::sleep(Duration::from_secs(1));
    }
}



