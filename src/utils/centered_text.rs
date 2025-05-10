// use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, widgets::Paragraph};
use ratatui::{prelude::*, widgets::Paragraph};

// given an area
// center text horizontally
// and vertically in area.
// using a paragraph
// this function renders.
pub fn render_text_centered_in_area(text: String, area: Rect, buf: &mut Buffer) {
    let (para, text_area) = center_text_in_given_area(text, area);
    para.render(text_area, buf)
}

// also applies styles to the other areas.
// for example a style like background :) which
// actually will be rendered.
pub fn render_text_centered_text_with_style(text: String, area: Rect, style: Style, buf: &mut Buffer) {
    // <'static> is a heap allocation i believe.

    let [a, text_area, b] =
        Layout::vertical([Constraint::Fill(1), Constraint::Min(1), Constraint::Fill(1)])
            .areas(area);

    let a_para = Paragraph::new("").style(style);
    let para = Paragraph::new(text)
        .style(style)
        .alignment(Alignment::Center);
    let b_para = Paragraph::new("").style(style);
    
    a_para.render(a, buf);
    para.render(text_area, buf);
    b_para.render(b, buf);
}


// given an area
// center text horizontally
// and vertically in area.
// using a paragraph
// this function renders.
// this does default styles
pub fn center_text_in_given_area(text: String, area: Rect) -> (Paragraph<'static>, Rect) {
    // <'static> is a heap allocation i believe.

    let [_a, text_area, _b] =
        Layout::vertical([Constraint::Fill(1), Constraint::Min(1), Constraint::Fill(1)])
            .areas(area);

    let para = Paragraph::new(text)
        .alignment(Alignment::Center);
    return (para, text_area);
}

