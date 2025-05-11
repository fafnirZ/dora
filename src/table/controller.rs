// utility functions
// to be called in src/controller.rs
// for mutating table state

use crate::app::App;

// the following 4 functions does the following:
// update the data slice for dataframe rows being
// displayed.
pub fn shift_displayed_df_value_slice_down(app_state: &mut App) {
    let increment_value = 1;
    // TODO: handle out of bounds
    // NOTE: oob doesnt matter, polars.slice wraps around YAY!
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_row_view_slice();
    let df_row_len = df_state.get_df_shape().0;
    if curr_view[1] == df_row_len {
        return;
    }
    let sliding_window_increment = [
        curr_view[0] + increment_value,
        curr_view[1] + increment_value,
    ];
    df_state.set_row_view_slice(sliding_window_increment);
}

pub fn shift_displayed_df_value_slice_up(app_state: &mut App) {
    let increment_value = -1;
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_row_view_slice();
    if curr_view[0] == 0 {
        return;
    }
    let sliding_window_increment = [
        curr_view[0] + increment_value,
        curr_view[1] + increment_value,
    ];
    df_state.set_row_view_slice(sliding_window_increment);
}

pub fn shift_displayed_df_value_slice_left(app_state: &mut App) {
    let increment_value = -1;
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_col_view_slice();
    if curr_view[0] == 0 {
        return;
    }
    let sliding_window_increment = [
        curr_view[0] + increment_value,
        curr_view[1] + increment_value,
    ];
    df_state.set_col_view_slice(sliding_window_increment);
}

pub fn shift_displayed_df_value_slice_right(app_state: &mut App) {
    let increment_value = 1;
    let df_state = &app_state.dataframe_state;
    let df_col_len = df_state.get_df_shape().1;
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_col_view_slice();
    if curr_view[1] >= df_col_len - 1 {
        return;
    }
    let sliding_window_increment = [
        curr_view[0] + increment_value,
        curr_view[1] + increment_value,
    ];
    df_state.set_col_view_slice(sliding_window_increment);
}

// the following four functions update the table cursors

pub fn shift_row_cursor_down(app_state: &mut App) {
    let increment_value = 1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;

    let df_row_len = df_state.get_df_shape().0;
    let curr_y = df_state.get_cursor_y();
    if *curr_y >= df_row_len {
        return;
    }
    df_state.set_cursor_y(curr_y + increment_value);
}

pub fn shift_row_cursor_up(app_state: &mut App) {
    let increment_value = -1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;

    let curr_y = df_state.get_cursor_y();
    if *curr_y <= 0 {
        return;
    }
    df_state.set_cursor_y(curr_y + increment_value);
}

pub fn shift_column_cursor_left(app_state: &mut App) {
    let increment_value = -1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    let curr_x = df_state.get_cursor_x();
    if *curr_x <= 0 {
        return;
    }
    df_state.set_cursor_x(curr_x + increment_value);
}

pub fn shift_column_cursor_right(app_state: &mut App) {
    let increment_value = 1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;

    let df_col_len = df_state.get_df_shape().1;
    let curr_x = df_state.get_cursor_x();
    if *curr_x >= df_col_len - 1 {
        return;
    }
    df_state.set_cursor_x(curr_x + increment_value);
}
