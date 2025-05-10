use std::path::{Path, PathBuf};

use directories::UserDirs;

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
pub fn init_shell_config_folder() {

}
