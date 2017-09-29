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
use std::env;
use std::process;

// SUBMODULES
mod render;
mod envs;
mod buffer;

// LOCAL INCLUDES
use byt::render::Renderer;
use byt::render::terminal;
use byt::envs::TermMode;
use byt::envs::os_unix::Term;
use byt::buffer::Buffer;

/// Initialize and start byt.
pub fn init() {
    // Struct with methods for manipulating the terminal.
    let term = Term::new();
    // Set the terminal to raw mode on startup
    term.start();
    term.set_mode(TermMode::Raw);

    let mut target = terminal::TermRenderer::new();

    // Get the size of the terminal window.
    let size = target.size();

    // For now we just move to the center
    target.move_cursor(size.row / 2, size.col / 2);
    target.write("BYT");

    // For now, just make a buffer from the first file
    // supplied as an argument.
    let buffer = match env::args().nth(1) {
        // TODO: better error handling here
        Some(x) => match Buffer::from_file(x.clone()) {
                Ok(v) => v,
                Err(_) => Buffer::new(x),
        },
        None => Buffer::new(String::from("Unnamed")),
    };

    // Read one byte at a time.
    let mut byte = [0u8];
    loop {
        io::stdin().read_exact(&mut byte)
                   .expect("Failed to read byte from stdin");
        let code = byte[0];

        print!("{}\n", byte[0]);
        if code == 113 {
            term.stop();
            term.set_mode(TermMode::Cooked);
            process::exit(0);
        }
    }
}
