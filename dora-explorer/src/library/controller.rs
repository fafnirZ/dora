use super::{control::Control, ExplorerState};



// given input,
// take a look at current state
// mutate state according to input
// its just a HUGE if else true basically.
pub struct Controller {}

impl Controller {
    // this function mutates the app state
    pub fn perform_actions(control: &Control, state: &mut ExplorerState) {
        match control {
            Control::ScrollUp => {
                let cursor_pos = &state.cursor_y;
                if *cursor_pos == 0 {
                    return;
                } else {
                    state.cursor_y -= 1;
                }
            }
            Control::ScrollDown => {
                let cursor_pos = &state.cursor_y;
                let num_dents = &state.dents.len();
                if *cursor_pos == (*num_dents as u16)-1 {
                    return;
                } else {
                    state.cursor_y += 1;
                }
            }
            _ => {}
        }
    }

}
