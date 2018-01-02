//! byt - vym
//!
//! Implement a mutator that mimic's vim's key layout.

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
use byt::io::binds::{Arrow, Keymaster, KeyInput};

// TODO add comments and explain everything

fn init_vym(vym : &mut Vym) {
    let mut keys = &mut vym.keys;
    let mut rust = &mut vym.rust;

    rust.register("vym.right", |state, target| {
        target.current_file().unwrap().move_right();
    });
    keys.bind_action([Key::Char('l')], "vym.right");

    rust.register("vym.left", |state, target| {
        target.current_file().unwrap().move_left();
    });
    keys.bind_action([Key::Char('h')], "vym.left");
}

struct VymState {
    mode : u8
}

pub struct Vym<'a> {
    rust : RustScope<'a, VymState, Editor>,
    keys : Keymaster
}

impl<'a> Vym<'a> {
    pub fn new() -> Vym<'a> {
        let mut vym = Vym {
            rust  : RustScope::new(VymState { mode : 0 }),
            keys  : Keymaster::new()
        };

        init_vym(&mut vym);

        vym
    }
}

impl<'a> Mutator<Editor> for Vym<'a> {
}

impl<'a> Actionable for Vym<'a> {
    fn actions(&mut self) -> Vec<Action> {
        self.keys.actions()
    }
}

impl<'a> Renderable for Vym<'a> {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> io::Result<()> {
        Ok(())
    }

    fn should_render(&self) -> bool {
        false
    }
}

impl<'a> Scope<Editor> for Vym<'a> {
    fn has_function(&self, name : &str) -> bool {
        self.rust.has_function(name)
    }

    fn call(&mut self, name : &str, target : &mut Editor) -> io::Result<()> {
        self.rust.call(name, target)
    }
}

impl<'a> KeyInput for Vym<'a> {
    fn consume(&mut self, key : Key) -> Option<()> {
        self.keys.consume(key)
    }
}
