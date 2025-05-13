
use polars::prelude::*;


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
