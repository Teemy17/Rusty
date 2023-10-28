use std::thread;
use std::time::Duration;
use tui::{
    backend::TermionBackend,
    widgets::{Block, Borders, Gauge, Row, Table},
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use sysinfo::{System, SystemExt};
use termion::raw::RawTerminal;
use std::io::stdout;

pub fn display_ram_usage() {
    // Initialize TUI terminal and backend
    let stdout = stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(RawTerminal::from(stdout)));
    let mut terminal = Terminal::new(backend).unwrap();

    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let ram_usage = get_ram_usage(&sys);

        let ram_display = create_ram_display(&sys, ram_usage);

        // Draw the TUI layout
        draw_ui(&mut terminal, &ram_display);

        thread::sleep(Duration::from_millis(100));

        if is_exit_key_pressed() {
            break; // Exit the loop
        }
    }
}

fn get_ram_usage(sys: &System) -> u8 {
    let ram_time = sys.used_memory();
    let total_time = sys.total_memory();

    ((ram_time as f64 / total_time as f64) * 100.0) as u8
}

fn create_ram_display(sys: &System, ram_usage: u8) -> (Gauge, Table) {
    let ram_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
        .label(format!("{}%", ram_usage))
        .percent(ram_usage as u16);

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

    (ram_gauge, table)
}

fn draw_ui(terminal: &mut Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>, ram_display: &(Gauge, Table)) {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(f.size());
        f.render_widget(ram_display.0.clone(), chunks[0]);
        f.render_widget(ram_display.1.clone(), chunks[1]);
    }).unwrap();
}

fn is_exit_key_pressed() -> bool {
    for key in std::io::stdin().keys() {
        if let Ok(key) = key {
            if let Key::Ctrl('q') = key {
                return true;
            }
        }
    }
    false
}

fn convert_kb_to_gb(kb: u64) -> f64 {
    kb as f64 / 1024.0 / 1024.0 / 1024.0
}
