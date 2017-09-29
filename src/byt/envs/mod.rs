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

// Used for *nix terminals. Allows us to intercept raw input
// instead of waiting for a return.
pub enum TermMode {
    Raw,
    Cooked,
}
