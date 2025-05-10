use crate::app::App;

pub fn shift_current_result_cursor_value_into_view(app_state: &mut App) {
    // shift the cursor value into view
    let result_cursor = app_state.search_result_state.result_cursor.unwrap();
    let result_location = app_state.search_result_state.result_indices[result_cursor].0;
    shift_displayed_df_row_to_a_particular_index(app_state, result_location as i64)
}

pub fn shift_displayed_df_row_to_a_particular_index(app_state: &mut App, index: i64) {
    // TODO: handle out of bounds
    let df_state = &mut app_state.dataframe_state;
    let new_view = [index, index + (df_state.rows_rendered as i64)];
    df_state.set_row_view_slice(new_view);
}
