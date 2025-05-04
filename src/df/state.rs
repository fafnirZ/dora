// use polars::frame::DataFrame;
use polars::prelude::*;
use super::super::header::Header;


const SLICE_SIZE: i64 = 24;
const MAX_ROWS_RENDERED: i64 = SLICE_SIZE;
const MAX_COLS_RENDERED: i64 = 5;



pub enum CursorFocus {
    Row,
    Column,
}
// for now its the all encompasing state object
// will figure out how to break it up later.
pub struct DataFrameState {
    source_path: String, // source file path to file
    // df: LazyFrame, // dataframe object itself
    // query: Option<Expr>,
    dataframe: DataFrame,

    ///////////////////////////////////////
    // the following are for UI purposes
    //
    // not sure if it should be owned here
    // but we figure it out later.
    ///////////////////////////////////////
    view_slice: [i64;2],    // the current viewable slice.
    cursor_x: i64,          // dataframe cursor for col NOTE: is limited by the number of columns renderable
    cursor_y: i64,          // dataframe cursor for row NOTE: is limited by the number of rows renderable
    cursor_focus: CursorFocus,   // dataframe cursor focus on row or column (renders different highlights)
}

impl DataFrameState {
    pub fn new(file_path: &str) -> Self {
        
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
            cursor_x: 0,
            cursor_y: 0,
            cursor_focus: CursorFocus::Row,
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

    // setter getters
    pub fn get_view_slice(&self) -> &[i64;2] {
        &self.view_slice
    }
    pub fn set_view_slice(&mut self, new_indices: [i64;2]) {
        self.view_slice = new_indices;
    }

    pub fn get_cursor_x(&self) -> &i64 {
        &self.cursor_x
    }
    pub fn set_cursor_x(&mut self, cursor_x: i64) {
        if cursor_x > MAX_COLS_RENDERED {
            self.cursor_x = MAX_COLS_RENDERED;
            return;
        }
        self.cursor_x = cursor_x;
    }
    pub fn get_cursor_y(&self) -> &i64 {
        &self.cursor_y
    }
    pub fn set_cursor_y(&mut self, cursor_y: i64) {
        // limit the max y to be MAX_ROWS Rendered
        if cursor_y > MAX_ROWS_RENDERED {
            self.cursor_y = MAX_ROWS_RENDERED;
            return;
        }
        self.cursor_y = cursor_y;
    }

    pub fn get_cursor_focus(&self) -> &CursorFocus {
        &self.cursor_focus
    }
    pub fn set_cursor_focus(&mut self, cursor_focus: CursorFocus) {
        self.cursor_focus = cursor_focus;
    }
}