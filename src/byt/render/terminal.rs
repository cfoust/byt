//! byt - render::terminal
//!
//! This module implements a Renderer that outputs ANSI escape codes to
//! STDOUT for use in an ANSI-compliant terminal.

// EXTERNS

// LIBRARY INCLUDES

// SUBMODULES

// LOCAL INCLUDES
use byt::render::*;
use byt::envs::os_unix::{Term};

pub struct TermRenderer {
    /// Maintains the size of the renderer so callers can know to
    /// stay in bounds.
    size : Point,
    term : Term,
}

impl TermRenderer {
    /// Construct a new TermRenderer.
    pub fn new() -> TermRenderer {
        TermRenderer {
            size : Point { row : 0, col : 0 },
            term : Term::new(),
        }
    }
}

impl Renderer for TermRenderer {
    fn clear(&mut self) -> &mut Renderer {
        self.term.clear_screen();
        self
    }

    fn done(&mut self) {
    }

    fn write(&mut self, out : &str) -> &mut Renderer {
        self.term.write(out);
        self
    }

    fn move_cursor(&mut self, row : u16, col : u16) -> &mut Renderer {
        self.term.move_cursor(row, col);
        self
    }

    fn size(&mut self) -> Point {
        let (row, col) = self.term.get_size();
        Point { row, col }
    }
}

