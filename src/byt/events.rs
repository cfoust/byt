//! Contains all of the events that the byt editor handles.

// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES

// LOCAL INCLUDES
use byt::editor::Action;

pub enum Event {
    /// An action that should be handled.
    Function(Action),
    /// Any keypress registered by stdio.
    KeyPress(Key)
}
