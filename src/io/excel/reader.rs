// the idea is to leverage
// calamine to open the .xlsx file
// then serialise the rows into an in memory
// csv string buffer.
// then pass the cursor into polars dataframe.

use std::{cell, io::Cursor};

// use polars::error::PolarsError;
use polars::prelude::*;
use calamine::{open_workbook_auto_from_rs, Reader, Sheets};

use crate::errors::DoraErrors;

pub struct ExcelReader{
    cursor: Cursor<Vec<u8>>
}

// this will 'feel' like all the other polars io readers.
impl ExcelReader {
    pub fn new(cursor: Cursor<Vec<u8>>) -> Self {
        Self {
            cursor: cursor,
        }
    }

    pub fn finish(&self) -> Result<DataFrame, DoraErrors> {
        let mut file_contents = open_workbook_auto_from_rs(self.cursor.clone())
            .map_err(|e| DoraErrors::IOError(e.to_string()))?;
        
        // TODO handle sheet later.
        // but right now we take first one and convert to dataframe :)
        let csv_buffers = ExcelReader::worksheets_to_csv_string_bufs(&mut file_contents);
        // consumes the buffers, because I don't want them to be doubly owned.
        // this obj is useless after this funtion.
        let sheet_1_contents = csv_buffers[0].to_owned(); // TODO, for initial testing purposes since I dont wanna deal with multisheets just yet.
        let cursor = Cursor::new(sheet_1_contents);

        // to polars dataframe.
        let df = CsvReader::new(cursor)
            .finish()
            .map_err(|e| DoraErrors::IOError(e.to_string()))?;
        Ok(df)
    }

    fn worksheets_to_csv_string_bufs(
        excel_file_contents: &mut Sheets<Cursor<Vec<u8>>>,
    ) -> Vec<String> {
        // each string in the vec will contain all data in a single sheet
        // inside of a string buffer of csv serialsied data.
        let mut sheet_store: Vec<String> = Vec::new();
        let worksheets = excel_file_contents.worksheets();
        for (_sheet_name, sheet_data) in worksheets.iter() {
            let mut sheet_buf = String::new();
            for row in sheet_data.rows() {
                let mut cell_vec = Vec::new();
                // serialise to csv format
                for cell in row {
                    cell_vec.push(cell.to_string().clone());
                }

                // utilise join so we dont get an additional , at the end
                // and dont have to do any special logic.
                let tmp = cell_vec.join(",");
                sheet_buf += &tmp;
                sheet_buf.push('\n');
            }
            sheet_store.push(sheet_buf);
        }
        sheet_store     
    }
}