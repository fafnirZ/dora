use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use crate::errors::DoraErrors;

pub fn read_bytes_from_local_sync(path: &str) -> Result<Cursor<Vec<u8>>, DoraErrors> {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        return Err(DoraErrors::IOError(
            format!("Local file not found at: {}", path).to_string(),
        ));
    }

    let mut file = File::open(path_obj).map_err(|e| DoraErrors::IOError(e.to_string()))?;

    // bytes buffer
    let mut contents = Vec::new();

    file.read_to_end(&mut contents)
        .map_err(|e| DoraErrors::IOError(e.to_string()))?;

    let cursor = Cursor::new(contents);
    return Ok(cursor);
}
