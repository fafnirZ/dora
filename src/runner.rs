use crate::app::App;


pub fn run_app() {
    let terminal = ratatui::init();
    App::new().main_loop(&mut terminal);
}