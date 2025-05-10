use std::any::Any;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
    // since the values are all nullable
    // the default should really be null.
    // if we ever want to have a serialisation 
    // default, we probably shouldnt use this function
    // for that.
    fn default() -> Self {
        Self {
            cell_height: None,
            cell_width: None,
            word_wrap: None,
        }
    }
}