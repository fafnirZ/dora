use std::ffi::OsString;

use jdora::run_app;

fn main() {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let file_path = args.get(0)
        .map(
            |os_str| os_str.to_string_lossy().into_owned()
        );
    run_app(file_path);
}