use std::{fs::{self, create_dir, File}, io::Read, path::{Path, PathBuf}};

use directories::UserDirs;

use crate::errors::DoraErrors;

use super::serde::Config;

// -> $HOME/.dora/
pub fn get_expected_config_folder_path() -> PathBuf {
    let ud = UserDirs::new()
        .unwrap();
    let user_home = ud
        .home_dir();
    let expected_folder_path = Path::new(".dora");
    return user_home.join(expected_folder_path)
}

// -> $HOME/.dora/config.toml
pub fn get_expected_config_file_path() -> PathBuf {
    let folder_path = get_expected_config_file_path();
    let file_path = Path::new("config.toml");
    return folder_path.join(file_path);
}




// initialise folder if doesnt exist
fn init_shell_config_folder() -> Result<(), DoraErrors> {
    let expected_config_path = get_expected_config_folder_path();
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


pub fn read_config_file() -> Result<Config, DoraErrors> {
    let folder_path = get_expected_config_folder_path();
    // init config folder if doesnt exist
    if !folder_path.exists() { 
        let _ = init_shell_config_folder(); 
    }

    // note, while we do init the `.dora` folder
    // we do not initialise their config file. 
    // the user is expected to create their own 
    // otherwise stick with system defaults.
    let file_path = get_expected_config_file_path();
    println!("file exists?: {}", file_path.exists());
    panic!("panicked");
    // if file_path.exists() {
    //     let mut file = File::open(file_path)
    //         .map_err(|e| DoraErrors::IOError(e.to_string()))?;
    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents)
    //         .map_err(|e| DoraErrors::IOError(e.to_string()))?;

    //     let config: Config = toml::from_str(&contents)
    //         .map_err(|e| DoraErrors::IOError(e.to_string()))?;
    //     return Ok(config);
    // }
    return Err(DoraErrors::IOError("~/.dora/config.toml not found".to_string()))
}