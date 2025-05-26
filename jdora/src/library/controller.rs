use std::any::Any;

use crossterm::cursor;

use crate::library::internal::node::try_resolve_node_path_mut;

use super::{control::Control, filter::ExactSubstringSearch, input::InputBuffer, internal::{node::try_resolve_node_path, node_path::NodePath}, mode::Mode, ExplorerState };

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
     
            Control::ScrollUp => {
                Controller::scroll_up(1, state);
            }
            Control::ScrollDown => {
                Controller::scroll_down(1, state);
            }
            Control::ExtendedScrollUp => {
                Controller::scroll_up(EXTENDED_SCROLL_SIZE, state);
            }
            Control::ExtendedScrollDown => {
                Controller::scroll_down(EXTENDED_SCROLL_SIZE, state);
            }
            Control::Enter => {

                let cursor_y = &state.cursor_y; // todo make absolute cursor
                let node_paths: Vec<NodePath> = state.root_node_structure
                    .clone()
                    .into_iter()
                    .map(|(_, node_path)| node_path)
                    .collect();
        
                let node_path = &node_paths[*(cursor_y) as usize];
                
                // resolves parent node
                let parent_node_path = node_path.parent(); 

                // need to resolve a mutable reference to the node path.
                // because we need to mutate that reference directly
                let resolved_node = try_resolve_node_path_mut(&mut state.root_node_state, &parent_node_path).unwrap(); 
                
                // get node leaf
                let node_path_leaf = node_path.leaf().unwrap(); // will break, oh well figure out later

                resolved_node.toggle_hide_child(&node_path_leaf);
                
                
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
                    return;
                }

            }
        }
    }
}

impl Controller {

    fn scroll_down(n: u16, state: &mut ExplorerState) {
        state.cursor_y += n;

        // // this is currently NON performant
        // let res = state.root_node_state.build_children_line_boundaries(0, &NodePath::new());
        // for (np, (start,end)) in res {
        //     if &state.cursor_y >= &start && &state.cursor_y < &end {
        //         state.node_path = np;
        //     }
        // }
    }

    fn scroll_up(n: u16, state: &mut ExplorerState) {
       state.cursor_y -= n;
        // // this is currently NON performant
        // let res = state.root_node_state.build_children_line_boundaries(0, &NodePath::new());
        // for (np, (start,end)) in res {
        //     if &state.cursor_y >= &start && &state.cursor_y < &end {
        //         state.node_path = np;
        //     }
        // }
    }
}