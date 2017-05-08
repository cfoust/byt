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
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};

// SUBMODULES
mod render;
mod brain;

// LOCAL INCLUDES
use byt::render::*;
use byt::brain::*;

/// Initialize and start byt.
pub fn init() {
    // Mutex for the current rendering size
    let data = Arc::new(Mutex::new(Point { row : 0, col : 0}));
    // Channel for communication to the renderer.
    let (tx, rx) = mpsc::channel();

    // Start the brain thread
    {
        let tx = tx.clone();
        let data = data.clone();

        thread::spawn(move || { brain_thread(tx, data); });
    }

    // Start the rendering thread
    let render = thread::spawn(move || { render_thread(rx, data); });

    render.join();
}
