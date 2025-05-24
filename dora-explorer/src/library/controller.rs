use std::any::Any;

use crossterm::cursor;

use super::{control::Control, filter::ExactSubstringSearch, input::InputBuffer, mode::Mode, navigator::{self, gcs::GCSNavigator, local::LocalNavigator, traits::{AnyNavigator, Navigator}, types::{DEnt, FileType}}, ExplorerState};

const EXTENDED_SCROLL_SIZE: u16 = 3;

// given input,
// take a look at current state
// mutate state according to input
// its just a HUGE if else true basically.
pub struct Controller {}

impl Controller {
    // this function mutates the app state
    pub fn perform_actions(control: &Control, state: &mut ExplorerState) {
        match &state.mode {
            Mode::Normal => Controller::handle_normal_mode_control(control, state),
            Mode::Filter => Controller::handle_filter_mode_control(control, state),
        }
    }

    fn handle_normal_mode_control(control: &Control, state: &mut ExplorerState) {
        match control {
            Control::Quit => {
                state.sig_user_input_exit = true;
            },
            Control::Filter => {
                state.mode = Mode::Filter;
                state.input_handler.init_input_buffer();

                // // need to initialise dent_shadow var 
                // // by cloning dents in the current state.
                // state.dents_filterview = Some(state.dents.clone());
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
                Controller::scroll_up(1, state);
            }
            Control::ScrollDown => {
                Controller::scroll_down(1, state);
            }
            Control::ExtendedScrollUp=> {
                Controller::scroll_up(EXTENDED_SCROLL_SIZE, state);
            }
            Control::ScrollDown => {
                Controller::scroll_down(EXTENDED_SCROLL_SIZE, state);
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
    fn handle_filter_mode_control(control: &Control, state: &mut ExplorerState) {
        match control {
            Control::Quit => {
                state.sig_user_input_exit = true;
            },
            Control::Esc => {
                state.mode = Mode::Normal;
                state.input_handler.reset_input_buffer();

                // reverts filter
                state.dents_filterview = None;

            }

            Control::Enter => {
                state.mode = Mode::Normal;
                state.input_handler.reset_input_buffer();

                // keeps filter
                // so does nothing. ui will continue to 
                // use filterview so long as its not a nullvalue
            }
            _ => {
                let current_buffer_string = {
                    match &state.input_handler.input_buffer {
                        InputBuffer::Active(input) => input.value(),
                        InputBuffer::Inactive => "",
                    }
                };

                if current_buffer_string == "" {
                    // reset filter view to be equal to unsullied dents
                    state.dents_filterview = Some(state.dents.clone());
                    return;
                }

                // keeps applying filter on original dents
                // value.
                let dents_fview: Vec<DEnt> = state
                    .dents
                    .clone()
                    .into_iter()
                    .filter(|entry|                 
                        ExactSubstringSearch{}.search(
                            current_buffer_string,
                            &*entry
                                .path
                                .file_name()
                                .unwrap_or(""),
                            true,
                        ).is_some())
                    .collect();

                state.dents_filterview = Some(dents_fview);

                // recalculate renderable

                state.recalculate_view_slice();
            }
        }
    }
}

impl Controller {

    fn scroll_down(n: u16, state: &mut ExplorerState) {
        let cursor_pos = &state.cursor_y;
        let num_dents = &state.get_dents_auto().len();
        let num_renderable = &state.recalculate_renderable_rows();
        if *cursor_pos == *num_renderable-1 {
            let [start,end] = &state.view_slice;
            if *end == (*num_dents as u16) {
                return;
            } else {
                // slide down 1
                state.view_slice = [
                    *start+n,
                    *end+n,
                ]
            }
        }
        else {
            state.cursor_y += n;
        }
    }

    fn scroll_up(n: u16, state: &mut ExplorerState) {
        let cursor_pos = &state.cursor_y;
        if *cursor_pos == 0 {
            let [start,end] = &state.view_slice;
            if *start == 0 {
                return;
            } else {
                // mutate to slide up 1
                state.view_slice = [
                    start-n,
                    end-n,
                ]
            }
        } else {
            state.cursor_y -= n;
        }
    }
}