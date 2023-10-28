use std::thread;
use std::time::Duration;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use termion::raw::IntoRawMode; // Add this import for raw mode
use termion::screen::AlternateScreen; // Add this import for AlternateScreen
use sysinfo::{System, SystemExt, NetworkExt,};
use tui::widgets::Paragraph;
use tui::style::{Style, Color};


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