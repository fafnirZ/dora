use std::{any::Any, f32::consts::E, path::{Path, PathBuf}};

use google_cloud_storage::{client::{Client, ClientConfig}, http::objects::list::ListObjectsRequest};

use crate::library::{errors::ExplorerError, ExplorerState};

use super::{traits::{AnyPath, Navigator}, types::{DEnt, FileType}};


pub struct GCSNavigator{}


impl Navigator for GCSNavigator {

    fn go_out_of_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {
    
        if let AnyPath::GSPath(cwd) = &state.cwd {
            let new_path = Self::remove_last_segment_gs(cwd).expect("removing last segment shouldnt fail");
             // check if the new path is a dir 
            // propagates error early and exits fn
            let client = &state.cloud_client;
            let unwrapped_client = client.as_ref().expect("Cloud client was not initialised");
            Self::getdents_from_path(&unwrapped_client, &new_path)?;
            
            // updating cwd
            state.cwd = AnyPath::GSPath(new_path);

            // refresh dents
            Self::refresh_d_ents(state)?;

            // refresh cursor
            state.cursor_y = 0;

            // refresh view slice
            let renderable_rows = state.recalculate_renderable_rows();
            state.view_slice = [0, renderable_rows];

            Ok(())
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
        
    }

    fn go_into_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        
        if let AnyPath::GSPath(cwd) = &state.cwd {

            let cursor_pos = *&state.cursor_y;
            let absolute_pos = &state.view_slice[0] + cursor_pos;
            let selected_d_ent_name = &state
                .dents[absolute_pos as usize]
                .path
                .file_name()
                .expect("well it should not be null")
                .to_string();
            
            let new_path = format!(
                "{}/{}",
                cwd, selected_d_ent_name,
            );
            let client = &state.cloud_client;
            let unwrapped_client = client.as_ref().expect("Cloud client was not initialised");
            // check if the new path is a dir 
            // propagates error early and exits fn
            Self::getdents_from_path(&unwrapped_client, &new_path)?;
            
            // updating cwd
            state.cwd = AnyPath::GSPath(new_path);

            // refresh dents
            Self::refresh_d_ents(state)?;

            // refresh cursor
            state.cursor_y = 0;

            // refresh view slice
            let renderable_rows = state.recalculate_renderable_rows();
            state.view_slice = [0, renderable_rows];

            Ok(())
            
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
    }

    fn refresh_d_ents(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        if let AnyPath::GSPath(cwd) = &state.cwd {
            let client = &state.cloud_client;
            let unwrapped_client = client.as_ref().expect("Cloud client was not initialised");
            state.dents = Self::getdents_from_path(&unwrapped_client, cwd)?;
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
        println!("path: {:?}", path);
        if let Some((bucket_prefix, bucket, cwd)) = split_gs_path_split(path) {
            if bucket_prefix != "gs://" {
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
                    // remove trailing slash
                    let prefix = prefix.trim_end_matches('/');
                    dents.push(
                        DEnt::new(
                            AnyPath::GSPath(
                                format!("{}{}/{}",
                                    bucket_prefix.to_string(),
                                    bucket.to_string(),
                                    prefix.to_string())),
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
            println!("{:?}", dents);
            Ok(dents)

        } else {
            return Err(ExplorerError::NotARemotePath("Invalid gcs path".to_string()))
        }
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

    // assuming non trailing '/' allowed.
    fn remove_last_segment_gs(path: &str) -> Option<String> {
        if path.starts_with("gs://") {
            let trimmed_path = &path[5..]; // Remove the "gs://" prefix
            Self::remove_last_segment_inner(trimmed_path).map(|s| format!("gs://{}", s))
        } else {
            Self::remove_last_segment_inner(path).map(|s| s.to_string())
        }
    }

    fn remove_last_segment_inner(path: &str) -> Option<&str> {
        if path.is_empty() {
            return None;
        }
        let chars: Vec<char> = path.chars().collect();
        let mut last_slash_index = None;
        for (i, _) in chars.iter().enumerate().rev() {
            if chars[i] == '/' {
                last_slash_index = Some(i);
                break;
            }
        }

        match last_slash_index {
            Some(index) => {
                if index == 0 {
                    // Handle the case where the path starts with '/', resulting in just "/"
                    Some("/")
                } else {
                    // Remove the last segment and any trailing slash
                    let parent_path = &path[..index];
                    if parent_path.ends_with('/') && parent_path.len() > 1 {
                        Some(&parent_path[..parent_path.len() - 1])
                    } else {
                        Some(parent_path)
                    }
                }
            }
            None => {
                // No slash found, meaning it's just a single element, so removing it results in an empty path
                None
            }
        }
    }
}
