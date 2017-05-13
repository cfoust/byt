//! byt - envs
//!
//! The modules here are included depending on the platform
//! we're targetting. They implement a standard set of 
//! functionality for text-based terminals.

// EXTERNS

// LIBRARY INCLUDES

// SUBMODULES
pub mod os_unix;

// LOCAL INCLUDES

pub enum TermMode {
    Raw,
    Cooked,
}
