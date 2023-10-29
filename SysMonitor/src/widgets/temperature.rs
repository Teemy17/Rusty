use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{ Block, Borders };
use tui::layout::{ Layout, Constraint, Direction };
use termion::raw::IntoRawMode; 
use termion::screen::AlternateScreen; 
use sysinfo::{ System, SystemExt, ComponentExt };
use tui::widgets::Paragraph;
use std::io;
use SysMonitor::exit;

pub fn display_temperature() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Initialize TUI terminal and backend
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    // Spawn a separate thread to handle keyboard input
    std::thread::spawn(move || {
        exit(tx);
    });

    loop {
        sys.refresh_all();

        let mut temperature_text: Vec<String> = Vec::new();

        for component in sys.components() {
            let component_name = component.label();
            let component_temperature = component.temperature();

            let text = format!("{}: {:.2}°C\n", component_name, component_temperature);
            temperature_text.push(text);
        }

        let paragraph = Paragraph::new(temperature_text.join("")).block(
            Block::default().title("Components temperature").borders(Borders::ALL)
        );

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size()); // Use the terminal size

                f.render_widget(paragraph, chunks[0]);
            })
            .unwrap();

        if let Ok(_) = rx.try_recv() {
            break;
        }
    }
}
