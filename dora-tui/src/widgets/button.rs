use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Modifier, Style}, widgets::{Block, Borders, Paragraph, StatefulWidget, Widget}, Frame
};
struct Button<'a> {
    label: &'a str,
    state: ButtonState,
    normal_style: Style,
    highlighted_style: Style,
    block: Option<Block<'a>>
}

enum ButtonState {
    Normal,
    Highlighted,
}
pub impl<'a> Button<'_> {
    pub fn new(label: &'a str) -> Self {
        Button {
            label,
            state: ButtonState::Normal,
            normal_style: Style::default(),
            highlighted_style: Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD),
            block: None,
        }
    }
    

    pub fn state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    pub fn highlighted_style(mut self, style: Style) -> Self {
        self.highlighted_style = style;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    // Function to render a button (using Paragraph and Block - example)
    pub fn render_button(frame: &mut Frame<'_>, area: Rect, text: &str) {
        let _button = Block::default()
            .borders(Borders::ALL)
            .render(area, &mut frame.buffer_mut());
        Paragraph::new(text).render(area, &mut frame.buffer_mut());
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.block.unwrap_or_else(|| Block::default().borders(Borders::ALL));
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