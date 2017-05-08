//! byt - render::threaded
//!
//! This is an implementation of Renderer that sends all render calls
//! to another thread via the provided transaction handle.

// EXTERNS

// LIBRARY INCLUDES
use std::thread;
use std::sync::{Arc,Mutex};
use std::sync::mpsc;
use std::ops::Deref;

// SUBMODULES

// LOCAL INCLUDES
use byt::render::{Point, Renderer};

/// Enum containing types of messages that can be sent to a ThreadRenderer.
pub enum RenderMessage {
    Clear,
    ClearToEndOfLine,
    /// Move the cursor to a row and column.
    Move(u16, u16),
    /// Write a string.
    Write(String),
}

/// Performs rendering calls by sending them along a channel to another thread.
pub struct ThreadRenderer {
    sender : mpsc::Sender<RenderMessage>,
    size   : Arc<Mutex<Point>>,
}

impl ThreadRenderer {
    pub fn new(sender : mpsc::Sender<RenderMessage>,
               size   : Arc<Mutex<Point>>) -> ThreadRenderer {
        ThreadRenderer {
            sender,
            size
        }
    }

    fn send(&self, msg : RenderMessage) {
        match self.sender.send(msg) {
            Ok(a) => (),
            Err(_) => panic!("Failed to send rendering message")
        }
    }
}

impl Renderer for ThreadRenderer {
    fn clear(&mut self) -> &mut Renderer {
        self.send(RenderMessage::Clear);
        self
    }

    fn done(&mut self) {
        //self.send(RenderMessage::Done);
    }

    fn move_cursor(&mut self, dest : Point) -> &mut Renderer {
        self.send(RenderMessage::Move(dest.row, dest.col));
        self
    }

    fn write(&mut self, out : &str) -> &mut Renderer {
        self.send(RenderMessage::Write(out.to_string()));
        self
    }

    fn size(&mut self) -> Point {
        // Grab a lock on the size mutex.
        let size = self.size.lock();

        // If we can get it, grab the size data.
        let size = match size {
            Ok(n)  => Point { .. *n.deref() },
            Err(_) => panic!("Failed to get lock on size"),
        };

        // Return the size.
        size
    }
}
