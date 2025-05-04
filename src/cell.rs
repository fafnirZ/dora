use ratatui::layout::Rect;


pub const HEADER_HEIGHT: u16 = 3;
pub const CELL_HEIGHT: u16 = 1;
pub const CELL_WIDTH: u16 = 15;


pub fn get_cell_area(x: u16, y: u16) -> Rect {
    return Rect::new(
        x,
        y,
        CELL_WIDTH,
        CELL_HEIGHT,
    )
}
pub fn get_header_area(x: u16, y: u16) -> Rect {
    return Rect::new(
        x,
        y,
        CELL_WIDTH,
        HEADER_HEIGHT,
    )
}


