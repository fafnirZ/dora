use polars::{frame::DataFrame, prelude::Column};



pub struct TableUI<'a> {
    header: &'a [Header],
    rows: &'a [Row],
}

impl<'a> TableUI<'a> {
    pub fn new(header: &'a[Header], rows: &'a[Row]) -> Self {
        Self {
            header,
            rows,
        }
    }
}


pub struct TableUIState {
    pub dataframe: DataFrame
}

impl TableUIState {
    pub fn new() -> Self {
        // boilerplate df for now
        let s0 = Column::new("days".into(), [0, 1, 2].as_ref());
        let s1 = Column::new("temp".into(), [22.1, 19.9, 7.].as_ref());
        let df = DataFrame::new(vec![s0, s1]).unwrap();
        Self {
            dataframe: df,
        }
    }
}