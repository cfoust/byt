//! byt - editor
//!
//! Everything relating to editor state.
// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;
use std::io;
use std::io::{
    Error,
    ErrorKind
};

// SUBMODULES
mod mutator;
mod tests;

// LOCAL INCLUDES
use byt::views::file::FileView;
use byt::io::binds::{Keymaster, KeyInput};
use byt::render;

#[derive(Clone, PartialEq, Debug)]
/// An action will try to run the function in the scope specified.
pub enum Action {
    Mutator(String),
    View(String),
    Editor(String),
}

/// Allows for the entity to produce Actions to be executed.
/// Similar to rendering, actions are
pub trait Actionable {
    /// Pop an action off of the action stack.
    fn grab_action(&mut self) -> Option<Action>;

    /// Attempt to perform the action. Will error if the Action is
    /// of an invalid type for this entity.
    fn perform_action(&mut self, action : Action) -> io::Result<()>;

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

    /// All of the global mutators for the editor.
    mutators : Vec<Box<mutator::Mutator<Editor>>>,

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
            mutators : Vec::new(),
            actions : Vec::new(),
        }
    }

    /// Get the file that's currently open.
    pub fn current_file(&mut self) -> Option<&mut FileView> {
        self.files.get_mut(self.current_file)
    }

    /// Attempt to open a file and make it the current pane.
    pub fn open(&mut self, path : &str) -> io::Result<()> {
        self.files.push(FileView::new(path)?);
        self.current_file = self.files.len() - 1;
        Ok(())
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
        // TODO This is dangerous. Fix this.
        self.files[self.current_file].should_render()
    }
}

impl Actionable for Editor {
    fn grab_action(&mut self) -> Option<Action> {
        self.actions.pop()
    }

    fn perform_action(&mut self, action : Action) -> io::Result<()> {
        if let Action::Editor(name) = action {
            return Ok(());
        }

        return Err(Error::new(ErrorKind::InvalidInput, "Wrong entity"));
    }

    fn has_action(&self) -> bool {
        self.actions.len() > 0
    }
}

impl mutator::Mutatable<Editor> for Editor {
    fn register_mutator(&mut self, mutator : Box<mutator::Mutator<Editor>>) -> io::Result<()> {
        self.mutators.push(mutator);
        Ok(())
    }
}
