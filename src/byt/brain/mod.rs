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

// SUBMODULES

// LOCAL INCLUDES
use byt::render::{Point, Renderer};
use byt::render::threaded;

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

    loop {
        thread::sleep(Duration::from_millis(5000));
    }
}
