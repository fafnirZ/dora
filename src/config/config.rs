use super::{dotconfig::read_config_file, serde::Config};

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

    // generic downcast from any
    // and provide fallback value.
    // if wasnt able to downcast will fall back to the system default
    // if not found in config, will fall back to system default
    fn from_config_or<T: Clone + 'static>(config: Config, attribute_name: &str, fall_back_value: T) -> T {
        match config.get_attr(attribute_name) {
            Some(val) => {
                if let Some(t_val) = val.downcast_ref::<T>() {
                    t_val.clone()
                } else {
                    fall_back_value.clone() // Return the fallback if downcast fails
                }
            }
            None => fall_back_value.clone(), // Return a clone of the fallback
        }
    }
}
