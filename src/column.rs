use polars::frame::column::Column as PlColumn;


pub struct Column {
    values: PlColumn,
}

impl Column {
    pub fn new(values: PlColumn) -> Self {
        Self{
            values: values
        } 
    }
}