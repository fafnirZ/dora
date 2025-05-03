use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{app::App, utils::centered_text::render_text_centered_in_area};


pub enum AppMode {
    Normal,
    Filter, // `&`
    Search, // `/`
    Help,   // `?`
}


pub struct ModeBanner {}

impl ModeBanner {
    pub fn new() -> Self { Self{} }
    
    fn determine_writing(
        state: &<ModeBanner as ratatui::prelude::StatefulWidget>::State
    ) -> String {
        let state = state;
        match state.state {
            AppMode::Normal => String::from("--normal--"),
            AppMode::Filter => String::from("--filter--"),
            AppMode::Search => String::from("--search--"),
            AppMode::Help => String::from("--help--"),
        }
    }
}

impl StatefulWidget for ModeBanner {
    type State = AppModeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let state_writing_fmt = ModeBanner::determine_writing(state);
        render_text_centered_in_area(state_writing_fmt, area, buf);
    }
}


// state class

pub struct AppModeState {
    pub state: AppMode,
}

impl AppModeState {
    pub fn new() -> Self {
        Self {
            ..Self::default()
        }
    }

    pub fn state(&self) -> &AppMode {
        &self.state
    }

    pub fn update_state(&mut self, new_state: AppMode) {
        self.state = new_state;
    }
}

impl Default for AppModeState {
    fn default() -> Self {
        return Self {
            state: AppMode::Normal,
        }
    }
}