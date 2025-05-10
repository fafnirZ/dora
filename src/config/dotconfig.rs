use std::{fs::{self, create_dir}, path::{Path, PathBuf}};

use directories::UserDirs;

use crate::errors::DoraErrors;

// -> $HOME/.dora/
pub fn get_expected_config_path() -> PathBuf {
    let ud = UserDirs::new()
        .unwrap();
    let user_home = ud
        .home_dir();
    let expected_folder_path = Path::new(".dora");
    return user_home.join(expected_folder_path)
}

// initialise folder if doesnt exist
pub fn init_shell_config_folder() -> Result<(), DoraErrors> {
    let expected_config_path = get_expected_config_path();
    if !expected_config_path.exists() {
        // init
        fs::create_dir(expected_config_path)
            .map_err(|e| DoraErrors::IOError(e.to_string()))?;
        
        Ok(())
    } else {
        if !(expected_config_path
        .metadata()
        .unwrap()
        .is_dir()) {
            return Err(DoraErrors::IOError("~/.dora folder does not exist".to_string()))
        }
        Ok(())
    }
}
