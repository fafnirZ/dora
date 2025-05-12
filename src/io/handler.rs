use std::fs::File;

use polars::{frame::DataFrame, prelude::*};

use crate::errors::DoraErrors;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use std::io::Cursor;

use super::excel::ExcelReader;
use super::file_type::FileType;
use super::gcloud::read_bytes_from_gcs_sync;
use super::local::read_bytes_from_local_sync;
use super::path_location::PathLocation;

pub fn read_from_any_path(path: &str) -> Result<DataFrame, DoraErrors> {
    let location = PathLocation::determine_location(path);
    let extension = match FileType::determine_extension(path) {
        Some(res) => res,
        None => return Err(DoraErrors::FileNotFound("File not found.".to_string())),
    };

    let cursor = match location {
        PathLocation::Gcs => read_bytes_from_gcs_sync(path)?,
        PathLocation::Local => read_bytes_from_local_sync(path)?,
    };

    return Ok(match extension {
        FileType::Csv => CsvReadOptions::default()
            .with_infer_schema_length(None) // infer schema with entire file.
            .into_reader_with_file_handle(cursor)
            .finish()
            .map_err(|e| DoraErrors::IOError(e.to_string()))?,

        FileType::Parquet => ParquetReader::new(cursor)
            .finish()
            .map_err(|e| DoraErrors::IOError(e.to_string()))?,

        FileType::NdJson => JsonReader::new(cursor)
            .with_json_format(JsonFormat::JsonLines)
            .finish()
            .map_err(|e| DoraErrors::IOError(e.to_string()))?,

        FileType::Excel => {
            panic!(
                "Excel files should not be read using this function, the developer did a whoopsie!"
            );
        } // _ => return Err(DoraErrors::FileNotFound("Invalid File Type".to_string())),
    });
}

// helper
pub fn get_cursor_from_any_path(path: &str) -> Result<Cursor<Vec<u8>>, DoraErrors> {
    let location = PathLocation::determine_location(path);
    let cursor = match location {
        PathLocation::Gcs => read_bytes_from_gcs_sync(path)?,
        PathLocation::Local => read_bytes_from_local_sync(path)?,
    };
    return Ok(cursor);
}

// excel sheets shouldnt be read
// from read_from_any_path
// since you need to provide an additional argument
pub fn read_excel_from_any_path(path: &str, sheet_index: usize) -> Result<DataFrame, DoraErrors> {
    let cursor = get_cursor_from_any_path(path)?;

    let df = ExcelReader::new(cursor).read_sheet(sheet_index)?;
    Ok(df)
}
