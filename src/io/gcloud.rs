use std::fs::File;

use polars::{frame::DataFrame, prelude::*};

use crate::errors::DoraErrors;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use std::io::Cursor;


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

async fn read_parquet_from_gcs_async(gs_path: &str) -> Result<DataFrame, PolarsError> {
    if let Some((prefix, bucket, path)) = split_gs_path_split(gs_path) {
        if prefix != "gs://" {
            return Err(PolarsError::InvalidOperation(
                "expected gs:// prefix.".into(),
            ));
        }
        let config = ClientConfig::default().with_auth().await.unwrap();
        let gcs_client = Client::new(config);

        let object_data = gcs_client
            .download_object(
                &GetObjectRequest {
                    bucket: bucket.to_string(),
                    object: path.to_string(),
                    ..Default::default()
                },
                &Range::default(),
            )
            .await
            .unwrap();

        let cursor = Cursor::new(object_data);

        return ParquetReader::new(cursor).finish();
    }
    return Err(PolarsError::InvalidOperation(
        "read parquet from gcs async failed.".into(),
    ));
}

pub fn read_parquet_from_gcs_sync(gs_path: &str) -> Result<DataFrame, PolarsError> {
    return Ok(tokio::runtime::Runtime::new()
        .map_err(|e| PolarsError::InvalidOperation(e.to_string().into()))
        .unwrap()
        .block_on(async {
            read_parquet_from_gcs_async(gs_path)
                .await
                .map_err(|e| PolarsError::InvalidOperation(e.to_string().into()))
        })
        .unwrap());
}
