//! byt - render::terminal
//!
//! The TermRenderer uses Termion to perform the necessary operations.

// EXTERNS

// LIBRARY INCLUDES
use std::io::{Write, ErrorKind, Error, Result};
use termion::*;

// SUBMODULES

// LOCAL INCLUDES
use super::*;

pub struct TermRenderer<'a> {
    out : &'a mut Write
}

impl<'a> TermRenderer<'a> {
    /// Construct a new TermRenderer.
    pub fn new(out : &'a mut Write) -> TermRenderer {
        TermRenderer {
            out
        }
    }
}

impl<'a> Renderer for TermRenderer<'a> {
    fn write(&mut self, text : &str) -> Result<()> {
        write!(self.out, "{}", text)
    }

    fn move_cursor(&mut self, row : u16, col : u16) -> Result<()> {
        write!(self.out, "{}", cursor::Goto(row, col))
    }

    fn size(&mut self) -> Result<(u16, u16)> {
        terminal_size()
    }
}
