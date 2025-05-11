use std::fs::File;

use polars::{frame::DataFrame, prelude::*};

use crate::errors::DoraErrors;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use std::io::Cursor;

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
        };
    }
}
