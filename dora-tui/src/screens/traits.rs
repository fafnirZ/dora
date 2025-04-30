use std::sync::Arc;

use ratatui::Frame;

use crate::app::App;

use super::main_screen::MainScreen;

pub trait ScreenRenderer {
    fn draw(&self, frame: &mut Frame, app_state: &App);
}

pub enum Screen {
    MainScreen(MainScreen),
}

// impl ScreenRenderer for Screen {
//     fn draw(&self, frame: &mut Frame) {
//         match self {
//             MainScreen => 
//         }
//     }
// }