use color_eyre::eyre::Report;
use dora_tui::app::{App};

fn main() -> Result<(), Report>  {
    color_eyre::install().expect("didnt install?");
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}