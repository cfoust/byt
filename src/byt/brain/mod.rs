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
use std::thread;
use std::time::Duration;
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
    let mut i = 0;

    let size = target.size();
    target.clear().done();

    target.move_cursor(size.row / 2, size.col / 2);
    target.write("BYT");

    let term = Term::new();
    term.set_mode(TermMode::Raw);

    let mut a = [0u8];
    loop {
        let input = io::stdin().read_exact(&mut a);

        if (a[0] == 113) {
            term.set_mode(TermMode::Cooked);
            process::exit(0);
        }

        println!("{}", a[0]);
    }
}
