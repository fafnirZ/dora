// use polars::frame::DataFrame;
use crate::{config::ConfigState, io::read_from_any_path, table::header::Header};
use polars::prelude::*;


// only use these as initialisation values
// we will update dynamically later.
const SLICE_SIZE: i64 = 50;
const MAX_ROWS_RENDERED: i64 = SLICE_SIZE;
const MAX_COLS_RENDERED: i64 = 10;

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
    pub dataframe: DataFrame,

    ///////////////////////////////////////
    // the following are for UI purposes
    //
    // not sure if it should be owned here
    // but we figure it out later.
    ///////////////////////////////////////
    row_view_slice: [i64; 2],  // the current viewable slice.
    col_view_slice: [i64; 2],  // the current viewable slice.
    cursor_x: i64, // dataframe cursor for col NOTE: is limited by the number of columns renderable
    cursor_y: i64, // dataframe cursor for row NOTE: is limited by the number of rows renderable
    cursor_focus: CursorFocus, // dataframe cursor focus on row or column (renders different highlights)

    // other UI state
    // when this changes we need to re-calculate how many rows and cols we are allowed to render.
    pub table_area: [u16; 2], // height, width
    pub rows_rendered: u16,   // number of rows rendered
    pub cols_rendered: u16,   // number of columns rendered
}

impl DataFrameState {
    pub fn new(file_path: &str) -> Self {
        // only supports csv right now
        // let df = CsvReadOptions::default()
        //     .try_into_reader_with_file_path(Some(file_path.into()))
        //     .unwrap()
        //     .finish()
        //     .unwrap();
        let df = read_from_any_path(file_path).unwrap();

        Self {
            source_path: String::from(file_path),
            dataframe: df,
            col_view_slice: [0, MAX_COLS_RENDERED],
            row_view_slice: [0, MAX_ROWS_RENDERED],
            cursor_x: 0,
            cursor_y: 0,
            cursor_focus: CursorFocus::Row,
            table_area: [0, 0], // height, width
            rows_rendered: MAX_ROWS_RENDERED as u16,
            cols_rendered: MAX_COLS_RENDERED as u16,
        }
    }

    // height, width
    pub fn get_df_shape(&self) -> (i64, i64) {
        let df = &self.dataframe;
        let shape = df.shape();
        (shape.0 as i64, shape.1 as i64)
    }

    pub fn get_headers(&self) -> Vec<Header> {
        let df = &self.dataframe;

        let df_schema = df.schema();
        let mut headers: Vec<Header> = vec![];
        for (col_name, _dt) in df_schema.iter() {
            headers.push(Header {
                name: col_name.to_string(),
            });
        }
        headers
    }

    // get headers which fall within the current viewable slice
    // the columns are NOT done this way, the columns are given
    // to the table UI as the entire series (due to filtering and querying)
    // requirements.
    pub fn get_headers_in_col_slice(&self) -> Vec<Header> {
        let df = &self.dataframe;

        let df_schema = df.schema();
        let mut headers: Vec<Header> = vec![];
        for (idx, (col_name, _dt)) in df_schema.iter().enumerate() {
            if idx < self.col_view_slice[0] as usize || idx > self.col_view_slice[1] as usize {
                continue;
            }
            headers.push(Header {
                name: col_name.to_string(),
            });
        }
        headers
    }

    // polars column
    pub fn get_columns(&self) -> Vec<&Column> {
        let df = &self.dataframe;
        // get columns
        let mut columns = vec![];
        for col_name in self.get_headers().iter() {
            let col = df.column(&col_name.name).unwrap();
            columns.push(col)
        }
        columns
    }
    pub fn get_columns_in_col_slice(&self) -> Vec<&Column> {
        let df = &self.dataframe;
        // get columns
        let mut columns = vec![];
        for (idx, col_name) in self.get_headers().iter().enumerate() {
            if idx < self.col_view_slice[0] as usize || idx > self.col_view_slice[1] as usize {
                continue;
            }
            let col = df.column(&col_name.name).unwrap();
            columns.push(col)
        }
        columns
    }

    pub fn get_column(&self, name: &String) -> &Column {
        self.dataframe.column(name).unwrap()
    }

    pub fn get_column_by_index(&self, index: i64) -> Option<&Column> {
        let headers = self.get_headers();
        for (idx, col_name) in headers.iter().enumerate() {
            if (idx as i64) == index {
                return Some(self.dataframe.column(&col_name.name).unwrap());
            }
        }
        return None;
    }
}

impl DataFrameState {
    pub fn get_file_name(&self) -> String {
        let s = self.source_path.clone();
        let last_element = s.rfind('/').map(|index| &s[index + 1..]).unwrap();
        last_element.to_string()
    }

    // refresh renderable table size
    pub fn recalculate_renderable_cells(&mut self, config_state: &ConfigState) {
        // get the current table area
        let table_area = self.table_area;

        ///////////////////////////////////
        // calculate the number of rows  //
        // and columns we can render     //
        ///////////////////////////////////
        let minus_one_for_good_luck_because_it_needs_padding = 1;
        let rows_renderable = ((table_area[0] - config_state.header_height)
            / config_state.cell_height
            - minus_one_for_good_luck_because_it_needs_padding)
            .min(MAX_ROWS_RENDERED as u16);

        let cols_renderable =
            (table_area[1] / config_state.cell_width).min(MAX_COLS_RENDERED as u16);

        self.rows_rendered = rows_renderable;
        self.cols_rendered = cols_renderable;
    }
    pub fn recalculate_view_slices(&mut self) {
        ////////////////////////////////////
        // update row and col view slices //
        ////////////////////////////////////
        self.row_view_slice[0] = self.cursor_y;
        self.row_view_slice[1] = self.cursor_y + self.rows_rendered as i64;
        self.col_view_slice[0] = self.cursor_x;
        self.col_view_slice[1] = self.cursor_x + self.cols_rendered as i64;
    }

    pub fn refresh_renderable_table_size(&mut self, config_state: &ConfigState) {
        self.recalculate_renderable_cells(config_state);
        self.recalculate_view_slices();
    }

    // setter getters
    pub fn get_row_view_slice(&self) -> &[i64; 2] {
        &self.row_view_slice
    }
    pub fn set_row_view_slice(&mut self, new_indices: [i64; 2]) {
        self.row_view_slice = new_indices;
    }

    pub fn get_col_view_slice(&self) -> &[i64; 2] {
        &self.col_view_slice
    }
    pub fn set_col_view_slice(&mut self, new_indices: [i64; 2]) {
        self.col_view_slice = new_indices;
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
