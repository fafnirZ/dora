
use ratatui::Frame;

use crate::app::App;

pub trait ScreenRenderer {
    fn draw(&self, frame: &mut Frame, app_state: &App);
}

