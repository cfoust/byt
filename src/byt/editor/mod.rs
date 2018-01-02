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
use std::vec::Drain;

// SUBMODULES
pub mod mutator;
mod tests;

// LOCAL INCLUDES
use byt::views::file::FileView;
use byt::io::binds::{Keymaster, KeyInput};
use byt::render;
use byt::editor::mutator::MutatePair;

#[derive(Clone, PartialEq, Debug)]
/// An action will try to run the function in the scope specified.
pub enum Action {
    Mutator(String),
    There(String),
}

/// Allows for the entity to produce Actions to be executed.
pub trait Actionable {
    /// All of the actions that have not yet been consumed.
    fn actions(&mut self) -> Vec<Action>;
}

/// Contains all editor state, responds to user input, and
/// renders appropriately.
pub struct Editor {
    /// Akin to vim's buffers. All of the open files in the editor.
    files : Vec<MutatePair<FileView>>,

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
    pub fn current_file(&mut self) -> Option<&mut MutatePair<FileView>> {
        self.files.get_mut(self.current_file)
    }

    /// Attempt to open a file and make it the current pane.
    pub fn open(&mut self, path : &str) -> io::Result<()> {
        self.files.push(MutatePair::new(FileView::new(path)?));
        self.current_file = self.files.len() - 1;
        Ok(())
    }
}

impl KeyInput for Editor {
    fn consume(&mut self, key : Key) -> Option<()> {
        {
            let mut file = self.current_file().unwrap();
            let result = file.consume(key);

            if result.is_some() {
                for action in file.actions() {
                    if let Action::Mutator(name) = action {
                        file.call_action(name.as_str());
                    }
                }

                return Some(());
            }
        }
            
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
    fn actions(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();

        for action in self.actions.drain(..) {
            actions.push(action);
        }

        actions
    }
}
