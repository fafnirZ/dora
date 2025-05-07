use std::fs::File;

use polars::{frame::DataFrame, prelude::*};

use crate::errors::DoraErrors;

const GS_PREFIX: &str = "gs://";

// given a path
// if suffix: .csv -> csv reader
// if suffix: .xlsx -> excel reader
// if prefix: gs:// -> gcs
// if prefix: /     -> local
pub enum PathLocation {
    Local,
    Gcs,
}

impl PathLocation {
    pub fn determine_location(path: &str) -> Self {
        return {
            if path.starts_with(GS_PREFIX) {
                PathLocation::Gcs
            } else {
                PathLocation::Local
            }
        }
    }
}

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

pub fn read_parquet_from_gcs_synchronously(gs_path: &str, service_account_key_path: Option<&str>) -> Result<DataFrame, PolarsError> {
    // Initialize the GCS client (this part needs to be async, so we'll block on it)
    let gcs_client = tokio::runtime::Runtime::new()
        .map_err(|e| PolarsError::External(Box::new(e)))
        .unwrap()
        .block_on(async {
            if let Some(path) = service_account_key_path {
                let key = ServiceAccountKey::from_file(path).await.map_err(|e| PolarsError::External(Box::new(e)))?;
                GcsClient::from_service_account_key(key).await.map_err(|e| PolarsError::External(Box::new(e)))
            } else {
                GcsClient::default().await.map_err(|e| PolarsError::External(Box::new(e)))
            }
        }).unwrap();

    // Parse the GCS URL
    let parsed_url = Url::parse(gcs_url)
        .map_err(|e| PolarsError::ComputeError(format!("Invalid GCS URL: {}", e)))?;
    let bucket_name = parsed_url.host_str()
        .ok_or(PolarsError::ComputeError("Missing bucket name in GCS URL".into()))?;
    let path = parsed_url.path().trim_start_matches('/');

    // Construct the GetObjectRequest
    let request = GetObjectRequest {
        bucket: bucket_name.to_string(),
        object: path.to_string(),
        ..Default::default()
    };

    // Download the object synchronously (block on the async call)
    let object_data = tokio::runtime::Runtime::new()
        .map_err(|e| PolarsError::External(Box::new(e)))?
        .block_on(async {
            gcs_client.download_object(&request).await.map_err(|e| PolarsError::External(Box::new(e)))
        })
        .unwrap();

    let data = Bytes::from(object_data);

    let cursor = Cursor::new(data);
    
    ParquetReader::new(cursor)
        .finish()

}

pub fn read_from_any_path(path: &str) -> Result<DataFrame, DoraErrors> {
    let location = PathLocation::determine_location(path);
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
            match location {
                PathLocation::Gcs => {
                    return Err(DoraErrors::FileNotFound("GCS not supported yet.".to_string()))
                },
                PathLocation::Local => {
                    let f = File::open(path).unwrap();
                    ParquetReader::new(f)
                        .finish()
                        .unwrap()
                }
            }
        }
        _ => return {
            Err(DoraErrors::FileNotFound("Invalid File Type".to_string()))
        }
    });
}