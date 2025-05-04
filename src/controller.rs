use crate::{app::App, input::Control, mode_banner::AppMode};


// given input,
// take a look at current state
// mutate state according to input
// its just a HUGE if else true basically.
pub struct Controller {}


impl Controller {

    // this function mutates the app state
    pub fn perform_actions(
        control: &Control,
        app_state: &mut App,
    ) {

        let app_mode = Controller::determine_app_mode(app_state);

        match app_mode {
            AppMode::Normal => Controller::handle_normal_mode_control(control, app_state),
            AppMode::Filter => Controller::handle_filter_mode_control(control, app_state),
            AppMode::Search => Controller::handle_search_mode_control(control, app_state),
            AppMode::Help => Controller::handle_help_mode_control(control, app_state),
        }
    }

    fn determine_app_mode(
        app_state: &App,
    ) -> &AppMode {
        &app_state.mode_state.state
    }

    fn handle_normal_mode_control(
        control: &Control,
        app_state: &mut App,
    ) {
        match control {
            Control::ScrollDown => {
                // TODO: handle out of bounds
                let df_state = &mut app_state.dataframe_state;
                let curr_view = df_state.get_view_slice();
                let increment_value = 1;
                let sliding_window_increment = [
                    curr_view[0]+increment_value,
                    curr_view[1]+increment_value,
                ];
                df_state.set_view_slice(sliding_window_increment);
            }

            Control::Filter => {
                app_state.mode_state.state = AppMode::Filter;
            },
            Control::Search => {
                app_state.mode_state.state = AppMode::Search;
            },
            Control::Help => {
                app_state.mode_state.state = AppMode::Help;
            },
            _ => {},
        }
    }
    fn handle_filter_mode_control(
        control: &Control,
        app_state: &mut App,
    ) {
        match control {
            Control::Esc => {
                app_state.mode_state.state = AppMode::Normal;
            },
            _ => {},
        }
    }
    fn handle_search_mode_control(
        control: &Control,
        app_state: &mut App,
    ) {
        match control {
            Control::Esc => {
                app_state.mode_state.state = AppMode::Normal;
            },
            _ => {},
        }
    }
    fn handle_help_mode_control(
        control: &Control,
        app_state: &mut App,
    ) {
        match control {
            Control::Esc => {
                app_state.mode_state.state = AppMode::Normal;
            },
            _ => {},
        }
    }

}