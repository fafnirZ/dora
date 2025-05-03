

pub struct TableBanner {}




pub struct TableBannerState {
    file_name: String,
}


impl TableBannerState {
    pub fn new() -> Self {
        Self {
            ..Self::default()
        }
    }
}

impl Default for TableBannerState {
    fn default() -> Self {
        return Self {
            file_name: String::from("test.parquet"),
        }
    }
}