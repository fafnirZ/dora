// actions to perform on exit

use super::ExplorerState;

pub fn print_results(state: &ExplorerState) {

    let user_input_exit = &state.sig_user_input_exit;
    if *user_input_exit {
        // should print cwd since no file selected
        // should print the current directory we are in instead
        let cwd = &state.cwd;
        println!("{}", cwd.to_str().unwrap());
    }


    // should print the path of the selected file
    let file_selected_exit = &state.sig_file_selected_exit;
    if *file_selected_exit {
        // print the path in cwd + current cursor on close.
        let cursor_idx = &state.cursor_y;
        let absolute_idx = &state.view_slice[0] + *cursor_idx;
        let selected_dent= &state.dents[absolute_idx as usize];
        println!("{}",selected_dent.path.to_str().unwrap());
    }

}