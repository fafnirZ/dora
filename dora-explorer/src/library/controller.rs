use std::any::Any;

use crossterm::cursor;

use super::{control::Control, navigator::{self, gcs::GCSNavigator, local::LocalNavigator, traits::{AnyNavigator, Navigator}, types::FileType}, ExplorerState};

// given input,
// take a look at current state
// mutate state according to input
// its just a HUGE if else true basically.
pub struct Controller {}

impl Controller {
    // this function mutates the app state
    pub fn perform_actions(control: &Control, state: &mut ExplorerState) {
        match control {
            Control::Quit => {
                state.sig_user_input_exit = true;
            },
            Control::ToggleShowDotFiles => {
                let curr = &state.show_dotfiles;
                state.show_dotfiles = !curr;
                match &state.navigator {
                    AnyNavigator::LocalNavigator => {
                        LocalNavigator::refresh_d_ents(state)
                        .unwrap_or_else(|_| {
                            return
                        }); // if not a directory do nothing for now:)
                    },
                    AnyNavigator::GCSNavigator => {
                        // NOTE: don't support hiding .dotfiles in gcs
                    },
                }
            },
            Control::ScrollUp => {
                let cursor_pos = &state.cursor_y;
                if *cursor_pos == 0 {
                    let [start,end] = &state.view_slice;
                    if *start == 0 {
                        return;
                    } else {
                        // mutate to slide up 1
                        state.view_slice = [
                            start-1,
                            end-1,
                        ]
                    }
                } else {
                    state.cursor_y -= 1;
                }
            }
            Control::ScrollDown => {
                let cursor_pos = &state.cursor_y;
                let num_dents = &state.dents.len();
                let num_renderable = &state.recalculate_renderable_rows();
                if *cursor_pos == *num_renderable-1 {
                    let [start,end] = &state.view_slice;
                    if *end == (*num_dents as u16) {
                        return;
                    } else {
                        // slide down 1
                        state.view_slice = [
                            *start+1,
                            *end+1,
                        ]
                    }
                }
                else {
                    state.cursor_y += 1;
                }
            }
            Control::ScrollRight => {

                match &state.navigator {
                    AnyNavigator::LocalNavigator => {
                        LocalNavigator::go_into_folder(state)
                        .unwrap_or_else(|_| {
                            return
                        }); // if not a directory do nothing for now:)
                    },
                    AnyNavigator::GCSNavigator => {
                        GCSNavigator::go_into_folder(state)
                        .unwrap_or_else(|_| {
                            return
                        }); // if not a directory do nothing for now:)
                    },
                }
                state.recalculate_view_slice();
            },
            Control::ScrollLeft => {
                match &state.navigator {
                    AnyNavigator::LocalNavigator => {
                        LocalNavigator::go_out_of_folder(state)
                        .unwrap_or_else(|_| {
                            return
                        }); // if not a directory do nothing for now:)
                    },
                    AnyNavigator::GCSNavigator => {
                        GCSNavigator::go_out_of_folder(state)
                        .unwrap_or_else(|_| {
                            return
                        }); // if not a directory do nothing for now:)
                    },
                }
                state.recalculate_view_slice();
            }
            Control::Enter => {
                // match file type
                // if its a directory, go into it
                // if its a file, exit program and return file path.
                let curr_pos = &state.cursor_y;
                let absolute_pos = &state.view_slice[0] + curr_pos;
                let selected_dent = &state.dents[absolute_pos as usize];
                match selected_dent.ftype {
                    FileType::Dir => {
                        match &state.navigator {
                            AnyNavigator::LocalNavigator => {
                                LocalNavigator::go_into_folder(state)
                                .unwrap_or_else(|_| {
                                    return
                                }); // if not a directory do nothing for now:)
                            },
                            AnyNavigator::GCSNavigator => {
                                GCSNavigator::go_into_folder(state)
                                .unwrap_or_else(|_| {
                                    return
                                }); // if not a directory do nothing for now:)
                            },
                        }
                    },
                    FileType::File => {
                        // exit program and return file path
                        state.sig_file_selected_exit = true;
                    },
                    _ => {},
                }
                
            }
            _ => {}
        }
    }

}
