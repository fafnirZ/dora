// use polars::frame::DataFrame;
use crate::{config::ConfigState, io::{read_excel_from_any_path, read_from_any_path}, table::header::Header};
use polars::prelude::*;

// only use these as initialisation values
// we will update dynamically later.
const SLICE_SIZE: u16 = 50;
const MAX_ROWS_RENDERED: u16 = SLICE_SIZE;
const MAX_COLS_RENDERED: u16 = 10;

const NULL_DF_ERR: &'static str = "Dataframe State's Dataframe attribute is None.";

pub enum CursorFocus {
    Row,
    Column,
}
// for now its the all encompasing state object
// will figure out how to break it up later.
pub struct DataFrameState {
    pub source_path: String, // source file path to file
    // df: LazyFrame, // dataframe object itself
    // query: Option<Expr>,
    pub dataframe: Option<DataFrame>,

    ///////////////////////////////////////
    // the following are for UI purposes
    //
    // not sure if it should be owned here
    // but we figure it out later.
    ///////////////////////////////////////
    row_view_slice: [u16; 2],  // the current viewable slice.
    col_view_slice: [u16; 2],  // the current viewable slice.
    // !!!!NOTE CURSORS ARE RELATIVE TO THE VIEW SLICE!!!!!
    cursor_x: u16, // dataframe cursor for col NOTE: is limited by the number of columns renderable
    cursor_y: u16, // dataframe cursor for row NOTE: is limited by the number of rows renderable
    cursor_focus: CursorFocus, // dataframe cursor focus on row or column (renders different highlights)

    // other UI state
    // when this changes we need to re-calculate how many rows and cols we are allowed to render.
    pub table_area: [u16; 2], // height, width
    pub rows_rendered: u16,   // number of rows rendered
    pub cols_rendered: u16,   // number of columns rendered
}

impl DataFrameState {
    pub fn new(file_path: &str) -> Self {
        Self {
            source_path: String::from(file_path),
            dataframe: None,
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

    // sets self.df = polars df
    // this allows lazy evaluation of the dataframe
    pub fn collect(&mut self) {
        let df = read_from_any_path(&self.source_path).unwrap();
        self.dataframe = Some(df);
    }

    // given an arbitrary df
    // set Df state's df
    // this is particularly useful
    // for 
    pub fn collect_from_excel_sheet(&mut self, sheet_index: usize) {
        let df = read_excel_from_any_path(
            &self.source_path,
            sheet_index,
        )
        .unwrap();
        self.dataframe = Some(df);
    }

    // height, width
    pub fn get_df_shape(&self) -> (u16, u16) {
        let df = self.dataframe.as_ref().expect(NULL_DF_ERR);

        let shape = df.shape();
        (shape.0 as u16, shape.1 as u16)
    }

    pub fn get_headers(&self) -> Vec<Header> {
        let df = self.dataframe.as_ref().expect(NULL_DF_ERR);

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
        let df = self.dataframe.as_ref().expect(NULL_DF_ERR);

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
        let df = self.dataframe.as_ref().expect(NULL_DF_ERR);
        // get columns
        let mut columns = vec![];
        for col_name in self.get_headers().iter() {
            let col = df.column(&col_name.name).unwrap();
            columns.push(col)
        }
        columns
    }
    pub fn get_columns_in_col_slice(&self) -> Vec<&Column> {
        let df = self.dataframe.as_ref().expect(NULL_DF_ERR);
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
        let df = self.dataframe.as_ref().expect(NULL_DF_ERR);

        df.column(name).unwrap()
    }

    pub fn get_column_by_index(&self, index: u16) -> Option<&Column> {
        let df = self.dataframe.as_ref().expect(NULL_DF_ERR);
        let headers = self.get_headers();
        for (idx, col_name) in headers.iter().enumerate() {
            if (idx as u16) == index {
                return Some(df.column(&col_name.name).unwrap());
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
        let rows_renderable = 
            ((
                ((table_area[0] - config_state.header_height) / config_state.cell_height) as f64)
                .floor() as u16
            ).min(MAX_ROWS_RENDERED as u16);

        let cols_renderable =
            (((table_area[1] / config_state.cell_width)as f64).floor() as u16).min(MAX_COLS_RENDERED as u16);

        let (table_rows, table_cols) = self.get_df_shape();
        self.rows_rendered = rows_renderable
            .min(table_rows);
        self.cols_rendered = cols_renderable
            .min(table_cols);
    }
    pub fn recalculate_view_slices(&mut self) {
        ////////////////////////////////////
        // update row and col view slices //
        ////////////////////////////////////
        //
        // keeping old implementation for now
        // previous implementation was cursor centric
        // now we are view slice centric
        // and adjust the cursor to be within the bounds
        // self.row_view_slice[0] = self.cursor_y;
        // self.row_view_slice[1] = self.cursor_y + self.rows_rendered;
        // self.col_view_slice[0] = self.cursor_x;
        // self.col_view_slice[1] = self.cursor_x + self.cols_rendered;

        let curr_row_view_slice = self.row_view_slice.clone();
        let curr_col_view_slice = self.col_view_slice.clone();
        // bound by max table size
        let max_cols_in_table = self.get_df_shape().1;
        let max_rows_in_table = self.get_df_shape().0;

        // generate new row sizes bounded by:
        //    renderable sizes
        //    actual table bounds
        let new_row_view_slice = [
            curr_row_view_slice[0],
            (curr_row_view_slice[0]+self.rows_rendered).min(max_rows_in_table),
        ];
        let new_col_view_slice = [
            curr_col_view_slice[0],
            (curr_col_view_slice[0]+self.rows_rendered).min(max_cols_in_table),
        ];
        // self.row_view_slice = new_row_view_slice;
        // self.col_view_slice = new_col_view_slice;
        self.set_col_view_slice(new_col_view_slice);
        self.set_row_view_slice(new_row_view_slice);
        // println!("rendered {},{}", self.rows_rendered, self.cols_rendered);

        // for simplicitys sake
        // just set the cursor to 0
        // if the user decided to change size mid navigation
        // itll be annoying but not the worst
        // we just gotta acknowledge that its view slice first.
        // this occurs when you resize the cells
        if self.cursor_x >= self.cols_rendered {
            self.cursor_x = 0;
        }
        if self.cursor_y >= self.rows_rendered {
            self.cursor_y = 0;
        }

    }

    pub fn refresh_renderable_table_size(&mut self, config_state: &ConfigState) {
        self.recalculate_renderable_cells(config_state);
        self.recalculate_view_slices();
    }

    // setter getters
    pub fn get_row_view_slice(&self) -> &[u16; 2] {
        &self.row_view_slice
    }
    pub fn set_row_view_slice(&mut self, new_indices: [u16; 2]) {
        self.row_view_slice = new_indices;
    }

    pub fn get_col_view_slice(&self) -> &[u16; 2] {
        &self.col_view_slice
    }
    pub fn set_col_view_slice(&mut self, new_indices: [u16; 2]) {
        self.col_view_slice = new_indices;
    }

    pub fn get_cursor_x(&self) -> &u16 {
        &self.cursor_x
    }
    pub fn set_cursor_x(&mut self, cursor_x: u16) {
        if cursor_x as u16 > self.cols_rendered{
            self.cursor_x = self.cols_rendered;
            return;
        }
        self.cursor_x = cursor_x;
    }
    pub fn get_cursor_y(&self) -> &u16 {
        &self.cursor_y
    }
    pub fn set_cursor_y(&mut self, cursor_y: u16) {
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
