use std::fs::File;

use polars::{frame::DataFrame, prelude::*};

use crate::errors::DoraErrors;
use std::io::Cursor;
use google_cloud_storage::client::{ClientConfig, Client};
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::download::Range;
// use url::Url;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::Read;


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

// example:
// gs://bucket/rest/of/path
// (gs://, bucket, rest/of/path)
fn split_gs_path_split(path: &str) -> Option<(&str, &str, &str)> {
    if !path.starts_with("gs://") {
        return None;
    }

    let rest = &path[5..];
    let parts: Vec<&str> = rest.splitn(2, '/').collect();

    match parts.len() {
        1 => Some(("gs://", parts[0], "")),
        2 => Some(("gs://", parts[0], parts[1])),
        _ => None, // Should not happen with splitn(2, '/')
    }
}

// pub fn read_parquet_from_gcs_synchronously(gs_path: &str, service_account_key_path: Option<&str>) -> Result<DataFrame, PolarsError> {
//     // Initialize the GCS client (this part needs to be async, so we'll block on it)
//     let gcs_client = tokio::runtime::Runtime::new()
//         .map_err(|e| PolarsError::ComputeError(Box::new(e)))
//         .unwrap()
//         .block_on(async {
//             if let Some(path) = service_account_key_path {
//                 let key = ServiceAccountKey::from_file(path)
//                     .await.map_err(|e| PolarsError::ComputeError(Box::new(e)));
//                 GcsClient::from_service_account_key(key).await.map_err(|e| PolarsError::ComputeError(Box::new(e)))
//             } else {
//                 GcsClient::default().await.map_err(|e| PolarsError::ComputeError(Box::new(e)))
//             }
//         }).unwrap();

//     // Parse the GCS URL
//     let parsed_url = Url::parse(gcs_url)
//         .map_err(|e| PolarsError::ComputeError(format!("Invalid GCS URL: {}", e)))?;
//     let bucket_name = parsed_url.host_str()
//         .ok_or(PolarsError::ComputeError("Missing bucket name in GCS URL".into()))?;
//     let path = parsed_url.path().trim_start_matches('/');

//     // Construct the GetObjectRequest
//     let request = GetObjectRequest {
//         bucket: bucket_name.to_string(),
//         object: path.to_string(),
//         ..Default::default()
//     };

//     // Download the object synchronously (block on the async call)
//     let object_data = tokio::runtime::Runtime::new()
//         .map_err(|e| PolarsError::ComputeError(Box::new(e)))?
//         .block_on(async {
//             gcs_client.download_object(&request).await.map_err(|e| PolarsError::ComputeError(Box::new(e)))
//         })
//         .unwrap();

//     let data = Bytes::from(object_data);

//     let cursor = Cursor::new(data);
    
//     ParquetReader::new(cursor)
//         .finish()

// }

async fn read_parquet_from_gcs_async(gs_path: &str) -> Result<DataFrame, PolarsError>{
    if let Some((prefix, bucket, path)) = split_gs_path_split(gs_path) {
        if prefix != "gs://" {
            return Err(PolarsError::InvalidOperation("expected gs:// prefix.".into())); 
        }
        let config = ClientConfig::default()
            .with_auth()
            .await
            .unwrap();
        let gcs_client = Client::new(config);

        let object_data = gcs_client.download_object(&GetObjectRequest {
                bucket: bucket.to_string(),
                object: path.to_string(),
                ..Default::default()
            }, &Range::default())        
            .await
            .unwrap();


        let cursor = Cursor::new(object_data);

        return ParquetReader::new(cursor)
            .finish()
    }
    return Err(PolarsError::InvalidOperation("read parquet from gcs async failed.".into()));
}


pub fn read_parquet_from_gcs_sync(gs_path: &str) -> Result<DataFrame, PolarsError> {
    return Ok(tokio::runtime::Runtime::new()
        .map_err(|e| PolarsError::InvalidOperation(e.to_string().into()))
        .unwrap()
        .block_on(async {
            read_parquet_from_gcs_async(gs_path)
            .await
            .map_err(
                |e| PolarsError::InvalidOperation(e.to_string().into())
            )
        })
        .unwrap())
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
                    match read_parquet_from_gcs_sync(path) {
                        Ok(res) => return Ok(res),
                        Err(err) => return Err(DoraErrors::IOError(err.to_string()))
                    }
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