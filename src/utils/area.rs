// utilities for padding areas:

use ratatui::layout::{Constraint, Layout, Rect};

pub fn horizontal_pad_area(input_area: Rect, percentages: [i32;3]) -> Rect {
    // TODO make sure percentages add up to 100;
    let [
        _,
        result,
        _,
    ] = Layout::horizontal([
        Constraint::Percentage(percentages[0] as u16),
        Constraint::Percentage(percentages[1] as u16),
        Constraint::Percentage(percentages[2] as u16),
    ]).areas(input_area);

    result
}