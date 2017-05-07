//! byt - terminal
//!
//! This module implements a Renderer that outputs ANSI escape codes to
//! STDOUT for use in an ANSI-compliant terminal.

use std::io::{self, Write};
use render::*;

const LEAD : &str = "\x1B[";   // Leader for all commands.
const CMD_CLEAR : &str = "2J"; // Clears the display.

pub struct TermRenderer {
    /// Maintains the size of the renderer so callers can know to
    /// stay in bounds.
    size : Point,

    /// Handle to STDOUT.
    stdout : io::Stdout,
}

impl TermRenderer {
    /// Construct a new TermRenderer.
    pub fn new() -> TermRenderer {
        TermRenderer {
            size : Point { row : 0, col : 0 },
            stdout : io::stdout()
        }
    }

    /// Prepend a command with the required leader.
    fn cmd(&mut self, rest : &str) {
        self.stdout.write(format!("{}{}", LEAD, rest).as_bytes());
    }

    /// Write a string at the current position.
    fn write(&mut self, out : &str) {
        self.stdout.write(out.as_bytes());
    }

    /// Move the cursor to a given position.
    fn move_cursor(&mut self, dest : Point) {
        self.cmd(format!("{};{}H",
                         dest.row.to_string(),
                         dest.col.to_string()).as_str());
    }
}

impl Renderer for TermRenderer {
    fn clear(&mut self) -> &mut Renderer {
        self.cmd(CMD_CLEAR);
        self
    }

    fn done(&mut self) {
        self.stdout.flush();
    }

    fn draw(&mut self, dest : Point, out : &str) -> &mut Renderer {
        self.move_cursor(dest);
        self.write(out);
        self
    }

    fn update_size(&mut self, new_size : Point) -> &mut Renderer {
        self.size = new_size;
        self
    }

    fn size(&self) -> Point {
        Point { .. self.size }
    }
}

