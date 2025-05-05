// utility functions
// to be called in src/controller.rs
// for mutating table state

use crate::app::App;


// the following 4 functions does the following:
// update the data slice for dataframe rows being
// displayed.
pub fn shift_displayed_df_value_slice_down(
    app_state: &mut App,
) {
    let increment_value = 1;
    // TODO: handle out of bounds
    // NOTE: oob doesnt matter, polars.slice wraps around YAY!
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_row_view_slice();
    let sliding_window_increment = [
        curr_view[0]+increment_value,
        curr_view[1]+increment_value,
    ];
    df_state.set_row_view_slice(sliding_window_increment);
}

pub fn shift_displayed_df_value_slice_up(
    app_state: &mut App,
) {
    let increment_value = -1;
    // TODO: handle out of bounds
    // NOTE: oob doesnt matter, polars.slice wraps around YAY!
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_row_view_slice();
    let sliding_window_increment = [
        curr_view[0]+increment_value,
        curr_view[1]+increment_value,
    ];
    df_state.set_row_view_slice(sliding_window_increment);
}


pub fn shift_displayed_df_value_slice_left(
    app_state: &mut App,
) {
    let increment_value = -1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_col_view_slice();
    let sliding_window_increment = [
        curr_view[0]+increment_value,
        curr_view[1]+increment_value,
    ];
    df_state.set_col_view_slice(sliding_window_increment);
}

pub fn shift_displayed_df_value_slice_right(
    app_state: &mut App,
) {
    let increment_value = 1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    let curr_view = df_state.get_col_view_slice();
    let sliding_window_increment = [
        curr_view[0]+increment_value,
        curr_view[1]+increment_value,
    ];
    df_state.set_col_view_slice(sliding_window_increment);
}


pub fn shift_displayed_df_value_to_a_particular_index(
    app_state: &mut App,
    index: i64
) {
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    let new_view= [
        index,
        index+(df_state.rows_rendered as i64),
    ];
    df_state.set_col_view_slice(new_view);
}




// the following four functions update the table cursors

pub fn shift_row_cursor_down(
    app_state: &mut App,
) {
    let increment_value = 1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    
    let curr_y = df_state.get_cursor_y();
    df_state.set_cursor_y(curr_y+increment_value);
}

pub fn shift_row_cursor_up(
    app_state: &mut App,
) {
    let increment_value = -1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    
    let curr_y = df_state.get_cursor_y();
    df_state.set_cursor_y(curr_y+increment_value);
}


pub fn shift_column_cursor_left(
    app_state: &mut App,
) {
    let increment_value = -1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    
    let curr_x = df_state.get_cursor_x();
    df_state.set_cursor_x(curr_x+increment_value);
}
pub fn shift_column_cursor_right(
    app_state: &mut App,
) {
    let increment_value = 1;
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    
    let curr_x = df_state.get_cursor_x();
    df_state.set_cursor_x(curr_x+increment_value);
}
