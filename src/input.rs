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
    Help,
    Filter,
    Search,
    Quit,
    Nothing,
    Esc,
    Command, // vim like command
    Enter, // enter key the generic version, if there is more nuanced definitions of enter we can define that later, right now i need a control which expresses the enter key in its generic form.
}

pub enum BufferState {
    Active(Input),
    Inactive,
}

pub struct InputHandler {
    events: Events,
    pub buffer_state: BufferState,
    pub mode_state: AppMode,

    // this is the location
    // for storing the error messages the
    // app wishes to communicate to the user.
    pub error_buffer: String,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            events: Events::new(),
            buffer_state: BufferState::Inactive,
            mode_state: AppMode::Normal,
            error_buffer: String::new(),
        }
    }

    pub fn next(&mut self) -> Control {
        let polled_event = self.events.next().unwrap();
        let control = match polled_event {
            Event::Input(key) => {
                if self.is_input_buffering() {
                    return self.handle_buffered_input(key);
                }
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
                KeyCode::Char('&') => Control::Filter,
                KeyCode::Char('/') => Control::Search,
                KeyCode::Char('?') => Control::Help,
                KeyCode::Char(':') => Control::Command,
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

    fn handle_buffered_input(&mut self, key_event: KeyEvent) -> Control {
        let input = match &mut self.buffer_state {
            BufferState::Active(input) => input,
            BufferState::Inactive => return Control::Nothing,
        };

        match key_event.code {
            KeyCode::Esc => return Control::Esc,
            KeyCode::Enter => {
                // need to think about how best to interpret the enter keycode.
                // to be honest I think conditional logic for interpreting the enter
                // key in different ways should not be placed in this layer, but should be
                // handled in the controller where it has access to the current states
                // however other forms of interpreting 'enter' key might arise when the enter
                // is interpreted in conjunction with other key terms for example shift enter?
                // or something like that
                return Control::Enter;
            }
            _ => {
                if input
                    .handle_event(&CrossTermEvent::Key(key_event)) // this function is the one which actually processes the key event.
                    .is_some()
                {
                    // TODO
                    return Control::Nothing;
                }
                return Control::Nothing;
            }
        }
    }

    // input buffer

    pub fn init_input_buffer(&mut self) {
        self.buffer_state = BufferState::Active(Input::default());
    }

    pub fn is_input_buffering(&self) -> bool {
        matches!(self.buffer_state, BufferState::Active(_))
    }

    pub fn reset_buffer(&mut self) {
        self.buffer_state = BufferState::Inactive;
        self.mode_state = AppMode::Normal;
    }

    // error buffer
    pub fn reset_error_buffer(&mut self) {
        self.error_buffer = String::new();
    }
}
