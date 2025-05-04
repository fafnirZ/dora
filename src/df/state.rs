// use polars::frame::DataFrame;
use polars::prelude::*;
use super::super::header::Header;


const SLICE_SIZE: i64 = 8;

// for now its the all encompasing state object
// will figure out how to break it up later.
pub struct DataFrameState {
    source_path: String, // source file path to file
    // df: LazyFrame, // dataframe object itself
    // query: Option<Expr>,
    dataframe: DataFrame,

    // not sure if it should be owned here
    // but we figure it out later.
    view_slice: [i64;2],    // the current viewable slice.
}

impl DataFrameState {
    pub fn new(file_path: &str) -> Self {
        // boilerplate df for now
        // let s0 = Column::new("days".into(), [0, 1, 2,999].as_ref());
        // let s1 = Column::new("temp".into(), [22.1, 19.9, 7., 999999.9].as_ref());
        // let df = DataFrame::new(vec![s0, s1]).unwrap();
        

        // only supports csv right now
        let df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(file_path.into()))
            .unwrap()
            .finish()
            .unwrap();

        Self {
            source_path: String::from(file_path),
            dataframe: df,
            view_slice: [0, SLICE_SIZE],
        }
    }

    pub fn get_headers(&self) -> Vec<Header> {
        let df = &self.dataframe;
        
        let df_schema = df.schema();
        let mut headers: Vec<Header> = vec![];
        for (col_name, _dt) in df_schema.iter() {
            headers.push(
                Header{name: col_name.to_string()}
            );
        }
        headers
    }

    // polars column
    pub fn get_columns(&self) -> Vec<Column> {
        let df = &self.dataframe;
        // get columns
        let mut columns = vec![];
        for col_name in self.get_headers().iter() {
            let col = df.column(&col_name.name).unwrap();
            // let dt = series.dtype();
            columns.push(
                col.clone(), // copy for now 
            )
        }
        columns
    }

    pub fn get_column(&self, name: &String) -> &Column {
        self.dataframe.column(name).unwrap()
    }
}

impl DataFrameState {
    pub fn get_file_name(&self) -> String {
        let s = self.source_path.clone();
        let last_element = s.rfind('/').map(|index| &s[index + 1..]).unwrap();
        last_element.to_string()
    }

    pub fn get_view_slice(&self) -> &[i64;2] {
        &self.view_slice
    }
    pub fn set_view_slice(&mut self, new_indices: [i64;2]) {
        self.view_slice = new_indices;
    }
}