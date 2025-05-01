use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::events::{Event, Events};

pub enum Control {
    Quit,
    Nothing,
}



pub struct InputHandler {
    events: Events,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            events: Events::new(),
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
                KeyCode::Esc => Control::Quit, // for now
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
}