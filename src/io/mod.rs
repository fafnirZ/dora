use std::fs::File;

use polars::{frame::DataFrame, prelude::*};

use crate::errors::DoraErrors;

const GS_PREFIX: &str = "gs://";

// given a path
// if suffix: .csv -> csv reader
// if suffix: .xlsx -> excel reader
// if prefix: gs:// -> gcs
// if prefix: /     -> local
// pub enum PathLocation {
//     Local,
//     Gcs,
// }

// impl PathLocation {
//     pub fn determine_location(path: &str) -> Self {
//         return {
//             if path.starts_with(GS_PREFIX) {
//                 PathLocation::Gcs
//             } else {
//                 PathLocation::Local
//             }
//         }
//     }
// }

pub enum FileType {
    Csv,
    Excel, // todo
    Parquet,
}

impl FileType {
    pub fn determine_extension(path: &str) -> Option<Self> {
        if path.ends_with(".csv") {
            return Some(FileType::Csv);
        } else if 
            path.ends_with(".xlsx")
            || path.ends_with(".xls")
        {
            return Some(FileType::Excel)
        } else if
            path.ends_with(".parquet")
            || path.ends_with(".pq")
        {
            return Some(FileType::Parquet)
        }
        None
    }
}


pub fn read_from_any_path(path: &str) -> Result<DataFrame, DoraErrors> {
    // let location = PathLocation::determine_location(path);
    let extension = match FileType::determine_extension(path) {
        Some(res) => res,
        None => {
            return Err(DoraErrors::FileNotFound("File not found.".to_string()))
        }
    };

    return Ok(match extension {
        FileType::Csv => {
            CsvReadOptions::default()
                .try_into_reader_with_file_path(Some(path.into()))
                .unwrap()
                .finish()
                .unwrap()
        },
        FileType::Parquet => {
            let f = File::open(path).unwrap();
            ParquetReader::new(f)
                .finish()
                .unwrap()
        }
        _ => return {
            Err(DoraErrors::FileNotFound("Invalid File Type".to_string()))
        }
    });
}