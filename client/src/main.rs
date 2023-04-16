use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        // let size = f.size();
        // let block = Block::default().title("Block").borders(Borders::ALL);
        // f.render_widget(block, size);
        ui(f);
    })?;
    // ui(terminal);

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
