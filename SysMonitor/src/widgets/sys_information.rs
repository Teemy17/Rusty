use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use termion::raw::IntoRawMode; // Add this import for raw mode
use termion::screen::AlternateScreen; // Add this import for AlternateScreen
use sysinfo::{System, SystemExt};
use tui::text::Text;
use tui::widgets::Paragraph;
use std::io;

pub fn display_system_information() -> Result<(), Box<dyn std::error::Error>> {
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