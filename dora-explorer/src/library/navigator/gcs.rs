use std::{any::Any, f32::consts::E, path::{Path, PathBuf}};

use google_cloud_storage::{client::{Client, ClientConfig}, http::objects::list::ListObjectsRequest};

use crate::library::{errors::ExplorerError, ExplorerState};

use super::{traits::{AnyPath, Navigator}, types::{DEnt, FileType}};


pub struct GCSNavigator{}


impl Navigator for GCSNavigator {

    fn go_out_of_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {

        if let AnyPath::GSPath(cwd) = &state.cwd {
            Ok(())
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
        
    }

    fn go_into_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        
        if let AnyPath::GSPath(cwd) = &state.cwd {
            Ok(())
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
    }

    fn refresh_d_ents(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        if let AnyPath::GSPath(cwd) = &state.cwd {
            Ok(())
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
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



impl GCSNavigator {


    async fn getdents_from_path_async(client: &Client, path: &str) ->Result<Vec<DEnt>, ExplorerError> {
        let mut dents: Vec<DEnt> = Vec::new();
        if let Some((prefix, bucket, cwd)) = split_gs_path_split(path) {
            if prefix != "gs://" {
                return Err(ExplorerError::NotARemotePath("expected gs:// prefix.".into()));
            }

            let request = ListObjectsRequest {
                bucket: bucket.to_string(),
                prefix: Some(cwd.to_string()),
                delimiter: Some("/".to_string()),
                ..Default::default()
            };

            let result = client
                .list_objects(&request)
                .await
                .map_err(|e| ExplorerError::NotARemotePath(e.to_string()))?;
            
            // get prefixes i.e. directories
            if let Some(prefixes) = result.prefixes {
                for prefix in prefixes {
                    dents.push(
                        DEnt::new(
                            AnyPath::GSPath(prefix.to_string()),
                            FileType::Dir,
                        )
                    )
                }
            } 

            // get items i.e. Files
            if let Some(items) = result.items {
                for item in items {
                    let item_name = item.name;
                    dents.push(
                        DEnt::new(
                            AnyPath::GSPath(item_name),
                            FileType::File,
                        )
                    )
                }
            } 

            // TODO next page token.
            // havent figured out how to use it yet.

        } 
        return Err(ExplorerError::NotARemotePath("Invalid gcs path".to_string()))
    }


    pub fn getdents_from_path(client: &Client, path: &str) -> Result<Vec<DEnt>, ExplorerError>{
        return Ok(tokio::runtime::Runtime::new()
            .map_err(|e| ExplorerError::NotARemotePath(e.to_string()))
            .unwrap()
            .block_on( async {
                Self::getdents_from_path_async(client, path)
                    .await
                    .map_err(|e| ExplorerError::NotARemotePath(e.to_string()))
            })
            .unwrap()
        )
    }


    async fn get_client_async() -> Result<Client, ExplorerError> {
        let config = ClientConfig::default()
            .with_auth()
            .await
            .map_err(|e| ExplorerError::NotARemotePath(e.to_string()))?;
        let client = Client::new(config);
        Ok(client)
    }

    // local path implementation
    pub fn get_client() -> Result<Client, ExplorerError>{
        return Ok(tokio::runtime::Runtime::new()
            .map_err(|e| ExplorerError::NotARemotePath(e.to_string()))
            .unwrap()
            .block_on( async {
                Self::get_client_async()
                    .await
                    .map_err(|e| ExplorerError::NotARemotePath(e.to_string()))
            })
            .unwrap()
        )
    }
}
