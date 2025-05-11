// the idea is to leverage
// calamine to open the .xlsx file
// then serialise the rows into an in memory
// csv byte buffer
// then pass the cursor into polars dataframe.

pub struct ExcelReader{}

