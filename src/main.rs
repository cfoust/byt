// EXTERNS
extern crate libc;

// LIBRARY INCLUDES
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};

// SUBMODULES
mod render;

// LOCAL INCLUDES
use render::*;

fn main() {
    let data = Arc::new(Mutex::new(Point { row : 0, col : 0}));

    let (tx, rx) = mpsc::channel();

    // The main thread
    {
        let tx = tx.clone();
        let data = data.clone();

        thread::spawn(move || {
            let mut term = threaded::ThreadRenderer::new(tx, data);
            term.clear()
                .move_cursor(Point { row : 2, col : 10 })
                .write("hello");
        });
    }

    // The rendering thread
    {
        thread::spawn(move || {
            let mut term = terminal::TermRenderer::new();
            loop {
                let data = rx.recv().unwrap();
                match data {
                    threaded::RenderMessage::Clear => term.clear().done(),
                    threaded::RenderMessage::Move(row, col) => term.move_cursor(Point { row, col }).done(),
                    threaded::RenderMessage::Write(out) => term.write(out.as_str()).done(),
                    _ => panic!("Something else"),
                }
            }
        });
    }

    loop {}
}
