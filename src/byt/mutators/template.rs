//! byt - template
//!
//! Template for a new mutator.

// EXTERNS

// LIBRARY INCLUDES

// SUBMODULES

// LOCAL INCLUDES
use byt::editor::mutator::*;
use byt::editor::*;

// TODO add comments and explain everything

struct BasicMutator {
}

impl Mutator<Editor> for BasicMutator {
}

impl Actionable for BasicMutator {
    fn actions(&mut self) -> Vec<Action> {
        Vec::new()
    }
}

impl Renderable for BasicMutator {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> io::Result<()> {
        Ok(())
    }

    fn should_render(&self) -> bool {
        false
    }
}

impl Scope<Editor> for BasicMutator {
    fn has_function(&self, name : &str) -> bool {
        false
    }

    fn call(&mut self, name : &str, target : &mut T) -> io::Result<()> {
        Ok(())
    }
}

impl KeyInput for BasicMutator {
    fn consume(&mut self, key : Key) -> Option<()> {
        None
    }
}
