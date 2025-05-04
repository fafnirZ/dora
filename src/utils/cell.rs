use ratatui::layout::Rect;



pub const CELL_HEIGHT: u16 = 3;
pub const CELL_WIDTH: u16 = 20;


pub fn get_cell_area(x: u16, y: u16) -> Rect {
    return Rect::new(
        x,
        y,
        CELL_WIDTH,
        CELL_HEIGHT,
    )
}


