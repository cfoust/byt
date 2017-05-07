//! byt - terminal
//!
//! This module implements a Renderer that outputs ANSI escape codes to
//! STDOUT for use in an ANSI-compliant terminal.

// EXTERNS

// LIBRARY INCLUDES
use std::io::{self, Write};
use std::process::Command;
use libc::{
    ioctl,
    c_void,
    TIOCGWINSZ,
    winsize
};

// SUBMODULES

// LOCAL INCLUDES
use render::*;

const LEAD : &str = "\x1B[";   // Leader for all commands.
const CMD_CLEAR : &str = "2J"; // Clears the display.

pub struct TermRenderer {
    /// Maintains the size of the renderer so callers can know to
    /// stay in bounds.
    size : Point,

    /// Reference to STDOUT.
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

    /// Get the current size of the terminal window and
    /// store it in self.size.
    fn get_size(&mut self) {
        let (mut rows, mut cols);

        // Get the size of the terminal by calling ioctl.
        unsafe {
            let mut ws = winsize {
                ws_row: 0,
                ws_col: 0,
                ws_xpixel: 0,
                ws_ypixel: 0,
            };

            // Grab a pointer to the struct.
            let ptr : *mut c_void = &mut ws as *mut _ as *mut c_void;

            // Run ioctl to check the window size.
            // TODO: If this fails, try TIOCGSIZE.
            let result = ioctl(1, TIOCGWINSZ, ptr);

            match result {
                0 => {rows = ws.ws_row; cols = ws.ws_col},
                _ => panic!("Grabbing terminal size from ioctl failed")
            }
        }

        self.size = Point { row : rows, col : cols };
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

    fn size(&mut self) -> Point {
        self.get_size();
        Point { .. self.size }
    }
}

