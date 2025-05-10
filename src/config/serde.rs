use std::any::Any;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    cell_height: Option<u16>,
    cell_width: Option<u16>,
    word_wrap: Option<bool>,
}

impl Config {
    pub fn get_attr(&self, attr_name: &str) -> Option<&dyn Any> {
        match attr_name {
            "cell_height" => Some(&self.cell_height),
            "cell_width" => Some(&self.cell_width),
            "word_wrap" => Some(&self.word_wrap),
            _ => None,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            cell_height: Some(1),
            cell_width: Some(15),
            word_wrap: Some(true),
        }
    }
}