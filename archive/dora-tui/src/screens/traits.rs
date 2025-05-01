
use ratatui::Frame;

use crate::app::App;

pub trait ScreenRenderer {
    fn draw(&self, frame: &mut Frame, app_state: &App);
}


pub trait CursorNavigator {
    // pages which implement this must
    // implement a navigation function
    // which is a state machine (if else statements)
    // which will update the is_selected state in various buttons
    fn navigate(&self);
}