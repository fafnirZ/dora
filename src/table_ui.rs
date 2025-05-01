use polars::frame::DataFrame;
use polars::prelude::Column as PlColumn;

use crate::header::Header;
use crate::column::Column;


pub struct TableUI<'a> {
    header: &'a [Header],
    columns: &'a [Column],
}

impl<'a> TableUI<'a> {
    pub fn new(header: &'a[Header], columns: &'a[Column]) -> Self {
        Self {
            header,
            columns,
        }
    }
}


pub struct TableUIState {
    pub dataframe: DataFrame
}

impl TableUIState {
    pub fn new() -> Self {
        // boilerplate df for now
        let s0 = PlColumn::new("days".into(), [0, 1, 2].as_ref());
        let s1 = PlColumn::new("temp".into(), [22.1, 19.9, 7.].as_ref());
        let df = DataFrame::new(vec![s0, s1]).unwrap();
        Self {
            dataframe: df,
        }
    }
}