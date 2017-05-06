//! byt - render
//!
//! This module provides a simple trait for drawing characters on a given
//! viewport. The goal is that this can be abstracted to render to something
//! like X if in the future that becomes desirable. For the time being we just
//! want to 'render' to the terminal window.

/// 'terminal' implements the Renderer for standard ANSI-compliant terminals.
pub mod terminal;

/// Describes a position in the rendering context in terms of rows and columns.
pub struct Point {
    pub row : u16,
    pub col : u16,
}

pub trait Renderer {
    /// Do any necessary initialization for this renderer.
    fn init(&self);

    /// Clears the viewport with blank characters.
    fn clear(&self) -> &Renderer;

    /// Indicates to the Renderer that the user is done manipulating
    /// the viewport. Useful for pooling transactions together and
    /// optimizing the amount of full-screen refreshes we might have
    /// to do.
    ///
    /// Ideally, this would only be called once by the caller in any given
    /// update loop.
    fn done(&self);

    /// "Draws" a sequence of utf-8 characters onto the screen in the given
    /// position.
    ///
    /// Positions are given in the ROW, COLUMN format.
    fn draw(&self, Point, &str) -> &Renderer;

    /// Get the current size of the rendering context in rows and columns.
    fn size(&self) -> Point;
}
