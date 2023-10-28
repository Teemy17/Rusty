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

pub fn display_cpu_usage() {
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