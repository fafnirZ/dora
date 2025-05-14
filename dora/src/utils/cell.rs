use ratatui::layout::Rect;

use crate::config::ConfigState;

// needs to be a mutable borrow, because
// the functions which call this require app to be mutable
// if otherwise compiler complains of both mutable and immutable in scope.
pub fn get_cell_area(config: &ConfigState, x: u16, y: u16) -> Rect {
    return Rect::new(x, y, config.cell_width, config.cell_height);
}
pub fn get_header_area(config: &ConfigState, x: u16, y: u16) -> Rect {
    return Rect::new(x, y, config.cell_width, config.header_height);
}
