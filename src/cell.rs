use ratatui::layout::Rect;

use crate::app::App;

// default configs
pub const HEADER_HEIGHT: u16 = 3;
pub const CELL_HEIGHT: u16 = 1;
pub const CELL_WIDTH: u16 = 15;
pub const LINE_NUMBER_CELL_WIDTH: u16 = 5;

pub fn get_cell_area(app_state: &App, x: u16, y: u16) -> Rect {
    let config = &app_state.config_state;

    return Rect::new(
        x,
        y,
        config.cell_width,
        config.cell_height,
    )
}
pub fn get_header_area(app_state: &App, x: u16, y: u16) -> Rect {
    let config = &app_state.config_state;
    return Rect::new(
        x,
        y,
        config.cell_width,
        config.header_height,
    )
}


