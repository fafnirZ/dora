use std::io::LineWriter;
use crossterm::{execute, terminal::EnterAlternateScreen};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::app::App;


pub fn run_app() {
    let mut output = std::io::stderr();
    execute!(output, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(LineWriter::new(output));
    let mut terminal = Terminal::new(backend).unwrap();
    App::new().main_loop(&mut terminal).unwrap();
}