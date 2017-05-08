//! byt - brain
//!

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

pub fn brain_thread(sender : mpsc::Sender<threaded::RenderMessage>,
                    size   : Arc<Mutex<Point>>) {
    // Create a new renderer that sends messages to the actual renderer,
    // but implements the same trait.
    let mut target = threaded::ThreadRenderer::new(sender, size);
    let mut i = 0;

    loop {
        let size = target.size();
        target.clear();

        for row in 0 .. size.row {
            target.move_cursor(Point { row, col : 0 });
            target.write(format!("THIS IS A TEST {}", i).as_str());
        }
        i += 1;
        if (i > 9) {
            i = 0;
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
