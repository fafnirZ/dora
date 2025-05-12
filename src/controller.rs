use polars::prelude::DataType;

use crate::{
    app::App,
    commands::controller::CommandHandler,
    df::state::CursorFocus,
    input::{BufferState, Control},
    io::read_excel_from_any_path,
    mode::AppMode,
    page::PageState,
    search::{
        approximate_substring_v1::SimpleApproximateSearch,
        controller::shift_current_result_cursor_value_into_view,
        search::par_find_substring_matches,
        traits::{AnySearchResult, SearchAlgorithmImplementations},
    },
    table::controller::{
        shift_column_cursor_left, shift_column_cursor_right, shift_displayed_df_value_slice_down,
        shift_displayed_df_value_slice_left, shift_displayed_df_value_slice_right,
        shift_displayed_df_value_slice_up, shift_row_cursor_down, shift_row_cursor_up,
    },
};

// given input,
// take a look at current state
// mutate state according to input
// its just a HUGE if else true basically.
pub struct Controller {}

impl Controller {
    // this function mutates the app state
    pub fn perform_actions(control: &Control, app_state: &mut App) {
        let app_mode = Controller::determine_app_mode(app_state);

        match app_mode {
            AppMode::Normal => Controller::handle_normal_mode_control(control, app_state),
            AppMode::Filter => Controller::handle_filter_mode_control(control, app_state),
            AppMode::Search => Controller::handle_search_mode_control(control, app_state),
            AppMode::Help => Controller::handle_help_mode_control(control, app_state),
            AppMode::Command => Controller::handle_command_mode_control(control, app_state),
            AppMode::SheetSelection => {
                Controller::handle_sheet_selection_mode_control(control, app_state)
            }
        }
    }

    fn determine_app_mode(app_state: &App) -> &AppMode {
        &app_state.input_handler.mode_state
    }

    fn handle_normal_mode_control(control: &Control, app_state: &mut App) {
        let df_state = &mut app_state.dataframe_state;
        match control {
            Control::ScrollDown => {
                df_state.set_cursor_focus(CursorFocus::Row);
                let cursor_y = df_state.get_cursor_y();
                let row_view_slice = df_state.get_row_view_slice();
                let slice_length = row_view_slice[1] - row_view_slice[0];
                let df_max_rows = df_state.get_df_shape().0;
                if *cursor_y == (slice_length - 1) {
                    if row_view_slice[0] > df_max_rows {
                    }
                    // reached the very beginning of the table
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
                if *cursor_y == 0 {
                    if row_view_slice[1] == 0 {
                    }
                    // reached the very beginning of the table
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
                    if col_view_slice[0] == 0 {
                    }
                    // reached the very beginning of the table
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
                let cols_renderable = df_state.cols_rendered;
                let col_view_slice = df_state.get_col_view_slice();
                let df_max_cols = df_state.get_df_shape().1;
                if *cursor_x >= (cols_renderable - 1) {
                    // println!("{},{},{},{},{}", cursor_x, col_view_slice[0], col_view_slice[1], cols_renderable, df_max_cols);
                    if col_view_slice[1] >= df_max_cols {
                    }
                    // reached the very end of the table
                    else {
                        shift_displayed_df_value_slice_right(app_state);
                    }
                } else {
                    shift_column_cursor_right(app_state);
                }
            }

            Control::Filter => {
                app_state.input_handler.mode_state = AppMode::Filter;
                app_state.input_handler.init_input_buffer();
            }
            Control::Search => {
                app_state.input_handler.mode_state = AppMode::Search;
                app_state.input_handler.init_input_buffer();
            }
            Control::Help => {
                app_state.input_handler.mode_state = AppMode::Help;
                app_state.input_handler.init_input_buffer();
            }
            Control::Command => {
                app_state.input_handler.mode_state = AppMode::Command;
                app_state.input_handler.init_input_buffer();
            }
            _ => {}
        }
    }

    // filter must be exact matching
    // filter will impact the underlying dataframe
    // by creating a dataframe filter expression to be
    // applied on underlying dataframe.
    fn handle_filter_mode_control(control: &Control, app_state: &mut App) {
        match control {
            Control::Esc => {
                app_state.input_handler.reset_buffer();
                app_state.input_handler.reset_error_buffer();
                app_state.input_handler.mode_state = AppMode::Normal;
            }
            _ => {}
        }
    }

    // search only highlights and allows skipping
    // search is matched with levenshtein distance
    fn handle_search_mode_control(control: &Control, app_state: &mut App) {
        match control {
            Control::Esc => {
                app_state.input_handler.reset_buffer();
                app_state.input_handler.reset_error_buffer();
                app_state.input_handler.mode_state = AppMode::Normal;
            }
            Control::Enter => {
                if app_state.search_result_state.result_indices.len() < 1 {
                    return; // do nothing
                }

                // note the cursor will only increment by 1
                // since it will only move onto the next result found
                // in the result vector
                // in other words the cursor denotes the index in the result vector
                // NOTE: i know it will break if the result vector was to change, but whatever.
                match app_state.search_result_state.result_cursor {
                    Some(_) => {
                        let result_len = app_state.search_result_state.result_indices.len();
                        let next_cursor_index =
                            (app_state.search_result_state.result_cursor.unwrap() + 1)
                                % (result_len); // % len forces it to wrap around to the beginning :)
                        app_state.search_result_state.result_cursor = Some(next_cursor_index);
                    }
                    None => {
                        app_state.search_result_state.result_cursor = Some(0);
                    }
                };

                shift_current_result_cursor_value_into_view(app_state);
            }
            Control::ScrollDown => {
                app_state.search_result_state.result_cursor =
                    Some(app_state.search_result_state.result_cursor.unwrap() + 1);
            }
            Control::ScrollUp => {
                app_state.search_result_state.result_cursor =
                    Some(app_state.search_result_state.result_cursor.unwrap() - 1);
            }
            _ => {
                let current_buffer_string = {
                    match &app_state.input_handler.buffer_state {
                        BufferState::Active(input) => input.value(),
                        BufferState::Inactive => "",
                    }
                };

                // let series_str_typed: Vec<String> = series.string();
                let column_index = app_state.dataframe_state.get_cursor_x();
                let column = app_state
                    .dataframe_state
                    .get_column_by_index(*column_index)
                    .unwrap();

                if *column.dtype() != DataType::String {
                    return; // do nothing
                }

                // replaces nulls with string "nulls"
                let series: Vec<String> = column
                    .as_series()
                    .unwrap()
                    .str()
                    .unwrap()
                    .into_iter()
                    .map(|opt_str| match opt_str {
                        Some(s) => s.to_string(),
                        None => "None".to_string(),
                    })
                    .collect();

                // finds results
                // TODO: control which algorithm to use
                // right now itll be hardcoded.
                // let algorithm = ExactSubstringSearch{};
                let algorithm = SearchAlgorithmImplementations::SimpleApproximateSearch(
                    SimpleApproximateSearch {},
                );

                match algorithm {
                    SearchAlgorithmImplementations::SimpleApproximateSearch(algo) => {
                        let results =
                            par_find_substring_matches(&algo, &series, current_buffer_string)
                                .into_iter() // owns the values now
                                .map(|(idx, res)| {
                                    (idx, AnySearchResult::SimpleApproximateSearch(res))
                                })
                                .collect();

                        // set results
                        app_state.search_result_state.result_indices = results;
                    }
                    SearchAlgorithmImplementations::ExactSubstringSearch(algo) => {
                        let results =
                            par_find_substring_matches(&algo, &series, current_buffer_string)
                                .into_iter() // owns the values now
                                .map(|(idx, res)| (idx, AnySearchResult::ExactSubstringSearch(res)))
                                .collect();

                        // set results
                        app_state.search_result_state.result_indices = results;
                    }
                }
            }
        }
    }
    fn handle_help_mode_control(control: &Control, app_state: &mut App) {
        match control {
            Control::Esc => {
                app_state.input_handler.reset_buffer();
                app_state.input_handler.reset_error_buffer();
                app_state.input_handler.mode_state = AppMode::Normal;
            }
            _ => {}
        }
    }

    fn handle_command_mode_control(control: &Control, app_state: &mut App) {
        match control {
            Control::Esc => {
                app_state.input_handler.reset_buffer();
                app_state.input_handler.reset_error_buffer();
                app_state.input_handler.mode_state = AppMode::Normal;
            }
            Control::Enter => {
                // this implementation of copying the value
                // solves the mutable and immutable borrow problem.
                // when that occurs you should find a way to get rid of one of them
                // in this case we don't pass in a mutable value anymore we pass in
                // a cloned value's reference.
                let buffer_value = match &app_state.input_handler.buffer_state {
                    BufferState::Active(buffer) => buffer.value().to_string(),
                    _ => String::new(),
                };
                match CommandHandler::try_execute(app_state, &buffer_value) {
                    Ok(_) => {
                        app_state.input_handler.error_buffer = String::new(); // clear previous error buffers
                    }
                    Err(err) => {
                        app_state.input_handler.error_buffer = err.to_string(); // sets error buffer
                    }
                }
            }
            _ => {} // do nothing
        }
    }

    fn handle_sheet_selection_mode_control(control: &Control, app_state: &mut App) {
        match control {
            Control::Enter => {
                let sheet_index = app_state.sheet_selector_state.cursor as usize;
                let df_state = &mut app_state.dataframe_state;
                df_state.collect_from_excel_sheet(sheet_index);
                app_state.page_state = PageState::TablePage; // render table page
                app_state.input_handler.mode_state = AppMode::Normal; // handle table mode inputs
            }
            Control::ScrollUp => {
                let cursor = app_state.sheet_selector_state.cursor;
                if cursor == 0 {
                    return;
                }
                app_state.sheet_selector_state.cursor = cursor - 1;
            }
            Control::ScrollDown => {
                let cursor = app_state.sheet_selector_state.cursor;
                let item_len = app_state
                    .sheet_selector_state
                    .sheet_names
                    .as_ref()
                    .expect("Sheet selector's sheet names is set to null.")
                    .len();
                if cursor == (item_len as u16) - 1 {
                    return;
                }
                app_state.sheet_selector_state.cursor = cursor + 1;
            }
            _ => {}
        }
    }
}
