//! byt - brain
//!
//! The brain thread handles all aspects of input and output that
//! are not covered by the rendering thread. Things like file IO,
//! keyboard input, and other important details are this module's
//! responsibility.
//!
//! It sends messages to the rendering thread over a channel provided
//! to it so that rendering tasks don't block.

// EXTERNS

// LIBRARY INCLUDES
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::io;
use std::io::Read;
use std::process;

// SUBMODULES

// LOCAL INCLUDES
use byt::render::{Point, Renderer};
use byt::render::threaded;
use byt::envs::TermMode;
use byt::envs::os_unix::Term;

/// Start the brain thread when given a channel to send messages
/// to the renderer and a mutex for the size of the window we're
/// rendering to.
pub fn brain_thread(sender : mpsc::Sender<threaded::RenderMessage>,
                    size   : Arc<Mutex<Point>>) {
    // Create a new renderer that sends messages to the actual renderer,
    // but implements the same trait.
    let mut target = threaded::ThreadRenderer::new(sender, size);

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
        io::stdin().read_exact(&mut byte)
                   .expect("Failed to read byte from stdin");

        if byte[0] == 113 {
            term.set_mode(TermMode::Cooked);
            term.write("\n");
            process::exit(0);
        }
    }
}
