use super::dotconfig::read_config_file;

// default configs
const WORD_WRAP: bool = false;
const HEADER_HEIGHT: u16 = 3;
const CELL_HEIGHT: u16 = 1;
const CELL_WIDTH: u16 = 15;
const LINE_NUMBER_CELL_WIDTH: u16 = 5;

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
        // deserialise from config file.
        let deserialised_config = read_config_file()
            .unwrap(); // yes it will panic


        Self {
            header_height: HEADER_HEIGHT,
            cell_height: CELL_HEIGHT,
            cell_width: CELL_WIDTH,
            line_number_cell_width: LINE_NUMBER_CELL_WIDTH,
            word_wrap: WORD_WRAP,
        }
    }

    
}
