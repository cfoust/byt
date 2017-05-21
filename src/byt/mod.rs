//! byt
//!
//! This library provides a way to initialize an instance of byt.
//!
//! In the future it'd be good if this included a way to change the
//! default file descriptors for STDOUT/STDERR. Then you could run
//! automated tests just by piping in a file.

// EXTERNS
extern crate libc;

// LIBRARY INCLUDES
use std::io;
use std::io::Read;
use std::process;

// SUBMODULES
mod render;
mod envs;

// LOCAL INCLUDES
use byt::render::Renderer;
use byt::render::terminal;
use byt::envs::TermMode;
use byt::envs::os_unix::Term;

/// Initialize and start byt.
pub fn init() {
    let mut target = terminal::TermRenderer::new();

    // Get the size of the terminal window.
    let size = target.size();
    target.clear().done();

    // For now we just move to the center
    target.move_cursor(size.row / 2, size.col / 2);
    target.write("BYT");

    // Struct with methods for manipulating the terminal.
    let term = Term::new();
    // Set the terminal to raw mode on startup
    term.set_mode(TermMode::Raw);

    // Read one byte at a time.
    let mut byte = [0u8];
    loop {
        io::stdin().read_exact(&mut byte)?;
                   .expect("Failed to read byte from stdin");

        if byte[0] == 113 {
            term.set_mode(TermMode::Cooked);
            term.write("\n");
            process::exit(0);
        }
    }
}
