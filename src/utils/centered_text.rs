// use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, widgets::Paragraph};
use ratatui::{prelude::*, widgets::Paragraph};

// given an area
// center text horizontally 
// and vertically in area.
// using a paragraph
// this function renders.
pub fn render_text_centered_in_area(
    text: String,
    area: Rect,
    buf: &mut Buffer,
) {

    let (para, text_area) = center_text_in_given_area(text, area);
    para.render(
        text_area,
        buf,
    )
}


// given an area
// center text horizontally 
// and vertically in area.
// using a paragraph
// this function renders.
pub fn center_text_in_given_area(
    text: String,
    area: Rect,
) -> (Paragraph<'static>, Rect) { // <'static> is a heap allocation i believe.

    let [
        _a,
        text_area,
        _nb,
    ] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Min(1),
        Constraint::Fill(1),
    ]).areas(area);

    let para = Paragraph::new(text)
        .alignment(Alignment::Center);
    return (para, text_area);
}