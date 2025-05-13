use std::ffi::OsString;

use dora::run_app;

fn main() {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let file_path = args[0].to_str().unwrap();

    run_app(file_path);
}
