
pub enum Control {
    ScrollUp,
    ExtendedScrollUp,
    ScrollDown,
    ExtendedScrollDown,
    ScrollLeft,
    ScrollRight,
    ToggleShowDotFiles,
    Filter,
    Quit,
    Nothing,
    Esc,
    Enter, // enter key the generic version, if there is more nuanced definitions of enter we can define that later, right now i need a control which expresses the enter key in its generic form.
}