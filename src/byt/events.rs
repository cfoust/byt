//! Contains all of the events that the byt editor handles.

// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES

// LOCAL INCLUDES
use byt::editor::Action;

pub enum Event {
    /// Any keypress registered by stdio.
    KeyPress(Key),
    Nothing
}
