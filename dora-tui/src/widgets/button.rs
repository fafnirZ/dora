use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Modifier, Style}, widgets::{Block, Borders, Paragraph, StatefulWidget, Widget}, Frame
};
pub struct Button<'a> {
    label: &'a str,
    state: ButtonState,
    normal_style: Style,
    highlighted_style: Style,
    block: Block<'a>,
}

enum ButtonState {
    Normal,
    Highlighted,
}
impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Button {
            label,
            state: ButtonState::Normal,
            normal_style: Style::default(),
            highlighted_style: Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD),
            block: Block::default().borders(Borders::ALL),
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.block;
        let inner_area = block.inner(area);
        block.render(area, buf);

        if inner_area.height > 0 && inner_area.width > 0 {
            let label = self.label;
            let style = match self.state {
                ButtonState::Normal => self.normal_style,
                ButtonState::Highlighted => self.highlighted_style,
            };

            let x = inner_area.x + (inner_area.width - label.len() as u16) / 2;
            let y = inner_area.y + inner_area.height / 2;

            buf.set_stringn(x, y, label, inner_area.width as usize, style);
        }
    }

}