//! byt - render::terminal
//!
//! The TermRenderer uses Termion to perform the necessary operations.

// EXTERNS

// LIBRARY INCLUDES
use std::io::Result;
use std::io::Write;
use termion::*;

// SUBMODULES

// LOCAL INCLUDES
use super::*;

pub struct TermRenderer<'a> {
    out : &'a Write
}

impl<'a> TermRenderer<'a> {
    /// Construct a new TermRenderer.
    pub fn new(out : &'a Write) -> TermRenderer {
        TermRenderer {
            out
        }
    }
}

impl<'a> Renderer for TermRenderer<'a> {
    fn write(&mut self, text : &str) -> Result<()> {
        Ok(())
    }

    fn move_cursor(&mut self, row : u16, col : u16) -> Result<()> {
        Ok(())
    }

    fn size(&mut self) -> Result<(u16, u16)> {
        Ok((0, 0))
    }
}

