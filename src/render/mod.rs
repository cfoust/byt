//! byt - render
//!
//! This module provides a simple trait for drawing characters on a given
//! viewport. The goal is that this can be abstracted to render to something
//! like X if in the future that becomes desirable. For the time being we just
//! want to 'render' to the terminal window.

// EXTERNS

// LIBRARY INCLUDES

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
    fn move_cursor(&mut self, Point) -> &mut Renderer;

    /// Write characters onto the screen at the cursor's position.
    fn write(&mut self, &str) -> &mut Renderer;

    /// Get the current size of the rendering context in rows and columns.
    fn size(&mut self) -> Point;
}

