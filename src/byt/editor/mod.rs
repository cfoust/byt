//! byt - editor
//!
//! Everything relating to editor state.
// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;
use std::io;

// SUBMODULES

// LOCAL INCLUDES
use byt::views::file::FileView;
use byt::io::binds::{Keymaster, KeyInput};
use byt::render;

#[derive(Clone, PartialEq, Debug)]
/// Represents a mutation of the editing state. Actions are guaranteed to execute in order, with
/// actions emitted in the editor as a whole taking precedence over those emitted in individual
/// panes.
pub struct Action {
}

/// Allows for the entity to produce Actions to be executed. 
/// Similar to rendering, actions are 
pub trait Actionable {
    /// Pop an action off of the action stack.
    fn grab_action(&mut self) -> Option<Action>;

    /// Check whether there is an action that can be handled.
    fn has_action(&self) -> bool;
}

/// Contains all editor state, responds to user input, and
/// renders appropriately. 
pub struct Editor {
    /// Akin to vim's buffers. All of the open files in the editor.
    files : Vec<FileView>,

    /// Stores all of the global keybindings.
    keys : Keymaster,

    /// The index of the current file inside self.files. In the future
    /// this will be a bit more elegant, but it's fine for the time being.
    current_file : usize,

    /// Stores any action that we've generated but hasn't
    /// been consumed yet.
    actions : Vec<Action>,

    /// Whether or not we should render at the next opportunity.
    should_render : bool,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            files : Vec::new(),
            keys  : Keymaster::new(),
            current_file : 0,
            should_render : false,
            actions : Vec::new(),
        }
    }

    pub fn current_file(&mut self) -> Option<&mut FileView> {
        self.files.get_mut(self.current_file)
    }
}

impl KeyInput for Editor {
    fn consume(&mut self, key : Key) -> Option<()> {
        // TODO: do it pane-locally first
        self.keys.consume(key)
    }
}

impl render::Renderable for Editor {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> io::Result<()> {
        self.current_file().unwrap().render(renderer, size)
    }

    fn should_render(&self) -> bool {
        self.should_render
    }
}

impl Actionable for Editor {
    /// Pop an action off the stack.
    fn grab_action(&mut self) -> Option<Action> {
        self.actions.pop()
    }

    /// Check whether there is an action to be consumed.
    fn has_action(&self) -> bool {
        self.actions.len() > 0
    }
}
