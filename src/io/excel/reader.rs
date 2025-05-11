// the idea is to leverage
// calamine to open the .xlsx file
// then serialise the rows into an in memory
// csv byte buffer
// then pass the cursor into polars dataframe.

use std::io::Cursor;

use polars::error::PolarsError;
use polars::prelude::*;
use calamine::open_workbook_auto_from_rs;

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
        let cal_reader = open_workbook_auto_from_rs(self.cursor)
            .map_err(|e| DoraErrors::IOError(e.to_string()))?;

        // TODO handle sheet later.
        // but right now we take first one and convert to dataframe :)
        

        
        Ok(())
    }
}