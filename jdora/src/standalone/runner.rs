use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::thread::panicking;
use std::{io::LineWriter, panic};

use crate::{library::exit::print_results, standalone::app::App};

fn restore_terminal_on_close_hook() {
    let original_panic_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        // Restore terminal states first so that the backtrace on panic can
        // be printed with proper line breaks
        disable_raw_mode().unwrap();
        execute!(std::io::stderr(), LeaveAlternateScreen).unwrap();
        original_panic_hook(info);
    }));
}

fn drop() {
    // If panicked, restoring of terminal states would have been done in the
    // panic hook. Avoid doing that twice since that would clear the printed
    // backtrace.
    if !panicking() {
        disable_raw_mode().unwrap();
        execute!(std::io::stderr(), LeaveAlternateScreen).unwrap();
    }
}

pub fn run_app(file_path: Option<String>) {
    // cleanup on panic hook
    restore_terminal_on_close_hook();

    // https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode
    // this is why the inputs arent being displayed
    // and why you don't need to hit enter in order for inputs to be registered.
    enable_raw_mode().unwrap();

    let mut output = std::io::stderr();
    execute!(output, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(LineWriter::new(output));
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new(file_path);
    app
        .main_loop(&mut terminal)
        .unwrap();

    // cleanup
    // in normal end
    drop();

    // print final results
    print_results(&app.explorer_state);
}
