use crate::cell::{CELL_HEIGHT, CELL_WIDTH, HEADER_HEIGHT, LINE_NUMBER_CELL_WIDTH};

// default configs
const WORD_WRAP: bool = false;

#[derive(Debug)]
pub struct ConfigState {
    // visual configs
    pub header_height: u16,
    pub cell_height: u16,
    pub cell_width: u16,
    pub line_number_cell_width: u16,
    pub word_wrap: bool,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            header_height: HEADER_HEIGHT,
            cell_height: CELL_HEIGHT,
            cell_width: CELL_WIDTH,
            line_number_cell_width: LINE_NUMBER_CELL_WIDTH,
            word_wrap: WORD_WRAP,
        }
    }
}
