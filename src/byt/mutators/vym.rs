//! byt - vym
//!
//! Implement a mutator that mimic's vim's key layout.
//!
//! We obviously don't want to implement everything from vim here. The whole point of writing our
//! own editor is to formulate our own paradigms. For me (cfoust) vim's movement and insertion keys
//! represent the pinnacle of efficiency. We don't need all of the bells and whistles necessarily,
//! just enough to facilitate the average programming use case.

// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;
use std::io;

// SUBMODULES

// LOCAL INCLUDES
use byt::editor::mutator::*;
use byt::editor::*;
use byt::render;
use byt::render::Renderable;
use byt::views::file::FileView;
use byt::io::binds::{Arrow, Keymaster, KeyInput};

// TODO add comments and explain everything

fn init_vym(vym : &mut Vym) {
    let mut normal = &mut vym.normal;
    let mut insert = &mut vym.insert;
    let mut rust   = &mut vym.rust;

    // ###########
    // NORMAL MODE
    // ###########
    // Initialize the HJKL motions
    rust.register("vym.right", |state, target, key| {
        target.move_right();
    });
    normal.bind_action([Key::Char('l')], "vym.right");

    rust.register("vym.left", |state, target, key| {
        target.move_left();
    });
    normal.bind_action([Key::Char('h')], "vym.left");

    rust.register("vym.down", |state, target, key| {
        target.move_down();
    });
    normal.bind_action([Key::Char('j')], "vym.left");

    // ###########
    // INSERT MODE
    // ###########
    rust.register("vym.insert", |state, target, key| {
        state.mode = Mode::Insert;
    });

    rust.register("vym.insert_char", |state, target, key| {
        if let Key::Char(c) = key {
            target.insert(c);
        }
    });
    normal.bind_action([Key::Char('i')], "vym.insert");

    // Transition back to normal mode with normal keybindings.
    rust.register("vym.normal", |state, target, key| {
        state.mode = Mode::Normal;
    });

    // Insert mode has its own binding table that defaults to just
    // inserting the character. This is so we can support arbitrary
    // bindings in insert mode in the future (like vim's Ctrl+r, which
    // can insert content from arbitrary registers).
    let insert_char = insert.mutator_action("insert_char");
    insert.get_root().set_wildcard(insert_char);
    insert.bind_action([Key::Ctrl('c')], "vym.normal");
    insert.bind_action([Key::Esc], "vym.normal");
}

enum Mode {
    Normal,
    Insert
}

struct VymState {
    mode : Mode
}

impl VymState {
    pub fn new() -> VymState {
        VymState {
            mode : Mode::Normal
        }
    }
}

pub struct Vym<'a> {
    rust : RustScope<'a, VymState, FileView>,
    normal : Keymaster,
    insert : Keymaster,
}

impl<'a> Vym<'a> {
    pub fn new() -> Vym<'a> {
        let mut vym = Vym {
            rust  : RustScope::new(VymState::new()),
            normal  : Keymaster::new(),
            insert  : Keymaster::new(),
        };

        init_vym(&mut vym);

        vym
    }
}

impl<'a> Mutator<FileView> for Vym<'a> {}

impl<'a> Actionable for Vym<'a> {
    fn actions(&mut self) -> Vec<Action> {
        match self.rust.state().mode {
            Mode::Normal => self.normal.actions(),
            Mode::Insert => self.insert.actions(),
        }
    }
}

impl<'a> Renderable for Vym<'a> {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> io::Result<()> {
        let (rows, cols) = size;
        Ok(())
    }

    fn should_render(&self) -> bool {
        false
    }
}

impl<'a> Scope<FileView> for Vym<'a> {
    fn has_function(&self, name : &str) -> bool {
        self.rust.has_function(name)
    }

    fn call(&mut self, name : &str, target : &mut FileView, key : Key) -> io::Result<()> {
        self.rust.call(name, target, key)
    }
}

impl<'a> KeyInput for Vym<'a> {
    fn consume(&mut self, key : Key) -> Option<()> {
        match self.rust.state().mode {
            Mode::Normal => self.normal.consume(key),
            Mode::Insert => self.insert.consume(key),
        }
    }
}
