use std::thread;
use std::time::Duration;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, Gauge};
use tui::layout::{Layout, Constraint, Direction};
use termion::raw::IntoRawMode; // Add this import for raw mode
use termion::screen::AlternateScreen; // Add this import for AlternateScreen
use sysinfo::System;
use sysinfo::SystemExt;
use sysinfo::CpuExt;
use tui::text::Text;
use tui::widgets::Paragraph;


pub fn display_ram_usage() {
    // Initialize TUI terminal and backend
    let stdout = std::io::stdout().into_raw_mode().unwrap(); // Set terminal in raw mode
    let backend = TermionBackend::new(AlternateScreen::from(stdout)); // Use AlternateScreen
    let mut terminal = Terminal::new(backend).unwrap();

    // Create a gauge widget to display CPU usage
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
            format!("Total memory: {} KB", sys.total_memory()),
            format!("Free memory : {} KB", sys.free_memory()),
            format!("Used memory : {} KB", sys.used_memory()),
            format!("Total swap  : {} KB", sys.total_swap()),
            format!("Free swap   : {} KB", sys.free_swap()),
            format!("Used swap   : {} KB", sys.used_swap()),
        ];


        // Draw the TUI layout with the CPU usage gauge
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());
            f.render_widget(ram_gauge, chunks[0]);
            f.render_widget(Paragraph::new(text.join("\n")), chunks[1]);
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

pub fn display_system_information() {
    
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());
}


