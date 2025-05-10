use std::fs::File;

use polars::{frame::DataFrame, prelude::*};

use crate::errors::DoraErrors;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use std::io::Cursor;

use super::file_type::FileType;
use super::gcloud::read_parquet_from_gcs_sync;
use super::path_location::PathLocation;


pub fn read_from_any_path(path: &str) -> Result<DataFrame, DoraErrors> {
    let location = PathLocation::determine_location(path);
    let extension = match FileType::determine_extension(path) {
        Some(res) => res,
        None => return Err(DoraErrors::FileNotFound("File not found.".to_string())),
    };

    return Ok(match extension {
        FileType::Csv => CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(path.into()))
            .unwrap()
            .finish()
            .unwrap(),
        FileType::Parquet => match location {
            PathLocation::Gcs => match read_parquet_from_gcs_sync(path) {
                Ok(res) => return Ok(res),
                Err(err) => return Err(DoraErrors::IOError(err.to_string())),
            },
            PathLocation::Local => {
                let f = File::open(path).unwrap();
                ParquetReader::new(f).finish().unwrap()
            }
        },
        _ => return Err(DoraErrors::FileNotFound("Invalid File Type".to_string())),
    });
}
