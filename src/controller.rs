use crossterm::cursor;

use crate::{app::{self, App}, df::state::CursorFocus, input::Control, mode_banner::AppMode, table::controller::{shift_column_cursor_left, shift_column_cursor_right, shift_displayed_df_value_slice_down, shift_displayed_df_value_slice_left, shift_displayed_df_value_slice_right, shift_displayed_df_value_slice_up, shift_row_cursor_down, shift_row_cursor_up}};


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
        let df_state = &mut app_state.dataframe_state;
        match control {
            Control::ScrollDown => {
                df_state.set_cursor_focus(CursorFocus::Row);
                let cursor_y = df_state.get_cursor_y();
                let row_view_slice = df_state.get_row_view_slice();
                let slice_length = row_view_slice[1] - row_view_slice[0];
                if *cursor_y == (slice_length-1) {
                    if row_view_slice[0] < 0 {} // reached the very beginning of the table
                    else {
                        shift_displayed_df_value_slice_down(app_state);
                    }
                } else {
                    shift_row_cursor_down(app_state);
                }
            }
            Control::ScrollUp => {
                df_state.set_cursor_focus(CursorFocus::Row);
                let cursor_y = df_state.get_cursor_y();
                let row_view_slice = df_state.get_row_view_slice();
                let df_max_rows = df_state.get_df_shape().0;
                if *cursor_y == 0 {
                    if row_view_slice[1] > df_max_rows {} // reached the very beginning of the table
                    else {
                        shift_displayed_df_value_slice_up(app_state);
                    }
                } else {
                    shift_row_cursor_up(app_state);
                }
            }
            Control::ScrollLeft => {
                df_state.set_cursor_focus(CursorFocus::Column);
                let cursor_x = df_state.get_cursor_x();
                let col_view_slice = df_state.get_col_view_slice();
                if *cursor_x == 0 {
                    if col_view_slice[0] == 0 {} // reached the very beginning of the table 
                    else {
                        shift_displayed_df_value_slice_left(app_state);
                    }
                } else {
                    shift_column_cursor_left(app_state); 
                }
            }
            Control::ScrollRight => {
                df_state.set_cursor_focus(CursorFocus::Column);
                let cursor_x = df_state.get_cursor_x();
                let col_view_slice = df_state.get_col_view_slice();
                let slice_length = col_view_slice[1] - col_view_slice[0];
                let df_max_cols = df_state.get_df_shape().1;
                if *cursor_x == (slice_length-1) {
                    if col_view_slice[1] > df_max_cols {} // reached the very end of the table
                    else {
                        shift_displayed_df_value_slice_right(app_state);
                    }
                } else {
                    shift_column_cursor_right(app_state); 
                }
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