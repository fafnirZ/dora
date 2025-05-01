use crate::app::App;


pub fn run_app() {
    let mut terminal = ratatui::init();
    App::new().main_loop(&mut terminal);
}