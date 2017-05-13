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
use std::sync::{Arc, Mutex, Condvar};

// SUBMODULES
mod render;
mod brain;
mod envs;

// LOCAL INCLUDES
use byt::render::*;
use byt::brain::*;

/// Initialize and start byt.
pub fn init() {
    // Mutex for the current rendering size
    let data = Arc::new(Mutex::new(Point { row : 0, col : 0}));

    // Channel for communication to the renderer.
    let (tx, rx) = mpsc::channel();

    // Make a Condvar to ensure that the rendering thread starts
    // and initializes before the brain.
    let renderLock = Arc::new((Mutex::new(false), Condvar::new()));

    // Start the rendering thread
    {
        let data = data.clone();
        let renderLock = renderLock.clone();

        thread::spawn(move || {
            // Has to be in its own scope so the lock drops correctly.
            {
                // Untangle the Convar
                let &(ref lock, ref cvar) = &*renderLock;
                // Change the value to true
                let mut renderingStarted = lock.lock().unwrap();
                *renderingStarted = true;
                cvar.notify_one();
            }

            // Start the rendering thread
            render_thread(rx, data);
        });
    }

    // Start the brain thread
    {
        {
            // Wait for the render thread to come up
            let &(ref lock, ref cvar) = &*renderLock;
            let mut renderingStarted = lock.lock().unwrap();
            while !*renderingStarted {
                renderingStarted = cvar.wait(renderingStarted).unwrap();
            }
        }

        let tx = tx.clone();
        thread::spawn(move || { brain_thread(tx, data); })
                // The brain thread decides when we exit.
                .join();
    }
}
