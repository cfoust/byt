//! byt - views
//!
//! Byt operates by handling a set of Views that refer to either a file or some other kind of UI
//! component. The term `View" is kind of disingenous as they are really more akin to both the
//! "controller" and "view" in MVC parlance, but I didn't think that was the right abstraction for
//! the job. Ultimately, a View just implements and follows the Renderable trait. The rest is up
//! for debate.

// EXTERNS

// LIBRARY INCLUDES

// SUBMODULES
pub mod file;

// LOCAL INCLUDES
