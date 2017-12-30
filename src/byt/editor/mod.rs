//! byt - editor
//!
//! Everything relating to editor state.
// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES

// LOCAL INCLUDES
use byt::views::file::FileView;
use byt::io::binds::Keymaster;

#[derive(Clone, PartialEq, Debug)]
pub struct Action {
}

pub struct Editor {
    /// Akin to vim's buffers. All the open files in the editor.
    files : Vec<FileView>,

    /// Stores all of the global keybindings.
    keys : Keymaster,

    /// The index of the current file.
    current_file : usize,

    /// Stores any action that we've generated but hasn't
    /// been consumed yet.
    actions : Vec<Action>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            files : Vec::new(),
            keys  : Keymaster::new(),
            current_file : 0,
            actions : Vec::new(),
        }
    }

    /// Consume a key of input.
    pub fn consume(&mut self, key : Key) -> Option<()> {
        // TODO: do it pane-locally first
        self.keys.consume(key)
    }

    /// Pop an action off the stack.
    pub fn grab_action(&mut self) -> Option<Action> {
        self.actions.pop()
    }

    /// Check whether there is an action to be consumed.
    pub fn has_action(&self) -> bool {
        self.actions.len() > 0
    }
}
