//! Contains all of the events that the byt editor handles.

// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES

// LOCAL INCLUDES

pub enum Event {
    /// An action that should be handled.
    Action(String),
    /// Any keypress registered by stdio.
    KeyPress(Key)
}
