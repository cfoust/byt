//! byt - render
//!
//! This module provides a simple trait for drawing characters on a given
//! viewport. The goal is that this can be abstracted to render to something
//! like X if in the future that becomes desirable. For the time being we just
//! want to 'render' to the terminal window.

// EXTERNS

// LIBRARY INCLUDES
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// SUBMODULES
pub mod terminal;
pub mod threaded;

// LOCAL INCLUDES

/// Describes a position in the rendering context in eerms of rows and columns.
pub struct Point {
    pub row : u16,
    pub col : u16,
}

/// Trait for some simple methods to create renderers for our editor.
pub trait Renderer {
    /// Clears the viewport with blank characters.
    fn clear(&mut self) -> &mut Renderer;

    /// Indicates to the Renderer that the user is done manipulating
    /// the viewport. Useful for pooling transactions together and
    /// optimizing the amount of full-screen refreshes we might have
    /// to do.
    ///
    /// Ideally, this would only be called once by the caller in any given
    /// update loop.
    fn done(&mut self);

    /// Move the cursor to the given position.
    fn move_cursor(&mut self, u16, u16) -> &mut Renderer;

    /// Write characters onto the screen at the cursor's position.
    fn write(&mut self, &str) -> &mut Renderer;

    /// Get the current size of the rendering context in rows and columns.
    fn size(&mut self) -> Point;
}

pub fn render_thread(
    receiver : mpsc::Receiver<threaded::RenderMessage>,
    size   : Arc<Mutex<Point>>) {

    let mut term = terminal::TermRenderer::new();
    {
        let current_size = term.size();
        let mut mutex_size = size.lock()
                                 .unwrap();
        mutex_size.row = current_size.row;
        mutex_size.col = current_size.col;
    }

    loop {
        // TODO: add error handling for this
        let data = receiver
                    .recv()
                    .unwrap();

        // TODO: Improve this someday so that transactions are pooled
        // together
        match data {
            threaded::RenderMessage::Clear => term.clear().done(),
            threaded::RenderMessage::Move(row, col) => term.move_cursor(row, col).done(),
            threaded::RenderMessage::Write(out) => term.write(out.as_str()).done(),
        }
    }
}
