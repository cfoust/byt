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
use std::io::Read;
use std::io::stdin;
use std::env;
use std::process;

// SUBMODULES
mod render;
mod envs;
mod io;

// LOCAL INCLUDES
use byt::render::Renderer;
use byt::render::terminal;
use byt::envs::TermMode;
use byt::envs::os_unix::Term;
use byt::io::file::PieceFile;

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
        Some(x) => match PieceFile::open(x.clone()) {
                Ok(v) => v,
                Err(_) => PieceFile::open(x).unwrap(),
        },
        None => PieceFile::open(String::from("Unnamed")).unwrap(),
    };

    // Read one byte at a time.
    let mut byte = [0u8];
    loop {
        stdin().read_exact(&mut byte)
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
