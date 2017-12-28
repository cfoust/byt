//! byt - render
//!
//! This module provides a simple trait for drawing characters on a given
//! viewport. The goal is that this can be abstracted to render to something
//! like X if in the future that becomes desirable. For the time being we just
//! want to 'render' to the terminal window.

// EXTERNS

// LIBRARY INCLUDES
use std::io::Result;

// SUBMODULES
pub mod terminal;

// LOCAL INCLUDES

/// Describes a position in the rendering context in eerms of rows and columns.
pub struct Point {
    pub row : u16,
    pub col : u16,
}

/// Describes a struct that can be rendered in text.
pub trait Renderable {
    /// Does everything necessary to update the visible contents.
    fn render(&self, renderer : &Renderer, size : (u16, u16)) -> Result<()>;

    /// Whether or not this entity should be rendered on this frame.
    fn should_render(&self) -> bool;
}

/// Trait for some simple methods to create renderers for our editor.
pub trait Renderer {
    // TODO: Return an error if you try to move the cursor outside of size().
    /// Move the cursor to the given position.
    fn move_cursor(&mut self, u16, u16) -> Result<()>;

    /// Write characters onto the screen at the cursor's position.
    fn write(&mut self, &str) -> Result<()>;

    /// Get the current size of the rendering context in rows and columns.
    fn size(&mut self) -> Result<(u16, u16)>;
}
