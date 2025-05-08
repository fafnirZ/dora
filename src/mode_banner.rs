use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, StatefulWidget, Widget}};

use crate::{app::App, input::BufferState, mode::AppMode, utils::centered_text::render_text_centered_in_area};



pub struct ModeBanner {}

impl ModeBanner {
    pub fn new() -> Self { Self{} }
    
    fn determine_writing(
        state: &<ModeBanner as ratatui::prelude::StatefulWidget>::State
    ) -> String {
        let state = state;
        let mode_str = match state.input_handler.mode_state {
            AppMode::Normal => String::from("--normal--"),
            AppMode::Filter => String::from("--filter--"),
            AppMode::Search => String::from("--search--"),
            AppMode::Help => String::from("--help--"),
            AppMode::Command => String::from("--command--"),
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
        let state_writing_fmt = ModeBanner::determine_writing(state);
        // render_text_centered_in_area(state_writing_fmt, area, buf);
        
        Paragraph::new(state_writing_fmt)
            .render(area, buf);
    }
}
