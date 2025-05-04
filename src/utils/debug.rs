use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Stylize}, widgets::{Paragraph, Widget}};


// colors an area to a particular color
// this way we can see exactly where the area is being
// rendered.
pub fn debug_render_area_bg(
    area: Rect,
    buf: &mut Buffer,
    color: Color,
) {
    let p = Paragraph::new("").bg(color);
    p.render(area, buf);
}