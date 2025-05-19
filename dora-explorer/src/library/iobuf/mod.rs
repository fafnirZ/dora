use tui_input::Input;


pub enum InputBuffer {
    Active(Input),
    Inactive,
}

pub enum OutputBuffer {
    Normal(String),
    Error(String),
}