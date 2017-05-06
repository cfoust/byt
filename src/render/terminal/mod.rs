//! byt - terminal
//!
//! This module implements a Renderer that outputs ANSI escape codes to
//! STDOUT for use in an ANSI-compliant terminal.

use render::*;

pub struct TermRenderer {
    /// Maintains the size of the renderer so callers can know to
    /// stay in bounds.
    pub size : Point
}

impl Renderer for TermRenderer {
    fn init(&self) {
    }

    fn clear(&self) -> &Renderer {
        self
    }

    fn done(&self) {
    }

    fn draw(&self, dest : Point, out : &str) -> &Renderer {
        self
    }

    fn size(&self) -> Point {
        Point { .. self.size }
    }
}

