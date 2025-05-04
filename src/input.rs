use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use crate::{app, events::{Event, Events}, mode_banner::AppModeState};

pub enum Control {
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    Help,
    Filter,
    Search,
    Quit,
    Nothing,
    Esc,
}

pub enum BufferState {
    Active(Input),
    Inactive,
}

pub struct InputHandler<'a> {
    events: Events,
    buffer_state: BufferState,
    app_mode_state: &'a mut AppModeState,
}

impl<'a> InputHandler<'a> {
    pub fn new(app_mode_state: &'a mut AppModeState) -> Self {
        Self {
            events: Events::new(),
            buffer_state: BufferState::Inactive,
            app_mode_state: app_mode_state,
        }
    }

    pub fn next(&mut self) -> Control {
        let polled_event = self.events.next().unwrap();
        let control = match polled_event {
            Event::Input(key) => {
                return self.handle_default(key);
            }
            _ => Control::Nothing
        };
        control
    }

    fn handle_default(&mut self, key_event: KeyEvent) -> Control {
        match key_event.modifiers {
            KeyModifiers::NONE => match key_event.code {
                KeyCode::Char('q') => Control::Quit,
                KeyCode::Char('k') | KeyCode::Up=> Control::ScrollUp,
                KeyCode::Char('j') | KeyCode::Down => Control::ScrollDown,
                KeyCode::Char('h') | KeyCode::Left => Control::ScrollLeft,
                KeyCode::Char('l') | KeyCode::Right => Control::ScrollRight,
                KeyCode::Esc => Control::Esc, // depends on context for esc handling
                KeyCode::Char('&') => Control::Filter,
                KeyCode::Char('/') => Control::Search,
                KeyCode::Char('?') => Control::Help,
                _ => Control::Nothing
            },
            KeyModifiers::SHIFT => match key_event.code {
                _ => Control::Nothing
            },
            KeyModifiers::CONTROL => match key_event.code {
                _ => Control::Nothing
            },
            _ => Control::Nothing,
        }
    }



    fn is_input_buffering(&self) -> bool {
        matches!(self.buffer_state, BufferState::Active(_))
    }

    fn reset_buffer(&mut self) {
        self.buffer_state = BufferState::Inactive;
        self.mode = InputMode::Default;
    }

}