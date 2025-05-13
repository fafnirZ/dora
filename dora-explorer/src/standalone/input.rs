use crate::{
    events::{Event, Events},
    mode::AppMode,
};
use crossterm::event::{Event as CrossTermEvent, KeyCode, KeyEvent, KeyModifiers};
use tui_input::{Input, backend::crossterm::EventHandler};

pub enum Control {
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    Quit,
    Nothing,
    Esc,
    Enter, // enter key the generic version, if there is more nuanced definitions of enter we can define that later, right now i need a control which expresses the enter key in its generic form.
}

// pub enum BufferState {
//     Active(Input),
//     Inactive,
// }

// pub enum MsgBuffer {
//     Normal(String),
//     Error(String),
// }

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
            _ => Control::Nothing,
        };
        control
    }

    fn handle_default(&mut self, key_event: KeyEvent) -> Control {
        match key_event.modifiers {
            KeyModifiers::NONE => match key_event.code {
                KeyCode::Char('q') => Control::Quit,
                KeyCode::Char('k') | KeyCode::Up => Control::ScrollUp,
                KeyCode::Char('j') | KeyCode::Down => Control::ScrollDown,
                KeyCode::Char('h') | KeyCode::Left => Control::ScrollLeft,
                KeyCode::Char('l') | KeyCode::Right => Control::ScrollRight,
                KeyCode::Esc => Control::Esc, // depends on context for esc handling
                KeyCode::Enter => Control::Enter,
                _ => Control::Nothing,
            },
            KeyModifiers::SHIFT => match key_event.code {
                _ => Control::Nothing,
            },
            KeyModifiers::CONTROL => match key_event.code {
                _ => Control::Nothing,
            },
            _ => Control::Nothing,
        }
    }


}
