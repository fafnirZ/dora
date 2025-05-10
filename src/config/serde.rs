use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    cell_height: Option<u16>,
    cell_width: Option<u16>,
    word_wrap: Option<bool>,
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