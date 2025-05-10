use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::{app::App, input::BufferState, mode::AppMode};

pub struct ModeBanner {}

impl ModeBanner {
    pub fn new() -> Self {
        Self {}
    }

    fn determine_writing(
        state: &<ModeBanner as ratatui::prelude::StatefulWidget>::State,
    ) -> String {
        let state = state;
        let mode_str = match state.input_handler.mode_state {
            AppMode::Normal => String::from("--normal--"),
            AppMode::Filter => String::from("filter:"),
            AppMode::Search => String::from("search:"),
            AppMode::Help => String::from("help:"),
            AppMode::Command => String::from(":"),
        };

        let input_buffer_string = {
            match &state.input_handler.buffer_state {
                BufferState::Active(input) => input.value(),
                BufferState::Inactive => "",
            }
        };

        return mode_str + input_buffer_string;
    }
}

impl StatefulWidget for ModeBanner {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [left_side, right_side] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(area);

        let err_buf = &state.input_handler.error_buffer;
        let is_err = { err_buf != "" };

        //
        // left side in app banner
        // is for commands and stuff
        //
        let mut common_style = Style::new();

        if is_err {
            common_style = common_style.bg(Color::Red);
        } else {
            // default color
            common_style = common_style.bg(Color::Rgb(67, 67, 113));
        }

        let state_writing_fmt = ModeBanner::determine_writing(state);

        Paragraph::new(state_writing_fmt)
            .style(common_style)
            .render(left_side, buf);

        //
        // right side is for errors
        //
        Paragraph::new(err_buf.clone())
            .style(common_style)
            .alignment(Alignment::Right)
            .render(right_side, buf);
    }
}
