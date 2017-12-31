//! byt - editor::mutator
//!
//! Provides abstractions over making modifications to editor state.
//! See the documentation for `Mutator`.

// EXTERNS

// LIBRARY INCLUDES
use std::collections::HashMap;
use std::io::{
    Error,
    ErrorKind
};
use std::io;

// SUBMODULES

// LOCAL INCLUDES

/// Defines a way of calling some function by its identifier
/// within a given scope. The closure is given a mutable reference
/// to something of the Scope's type.
pub trait Scope<S, T> {
    /// Perform the function referred to by `name` on the mutable target.
    /// Will error if the name has no association.
    fn call(&self, name : &str, state : &mut S, target : &mut T) -> io::Result<()>;
}

/// Stores and allows the invocation of any procedures defined in Rust.
/// Each closure is given a mutable reference to some kind of state storage
/// (usually a struct or even the mutator itself) and the closure's target,
/// which would be something like the editor or a pane.
pub struct RustMutator<'a, S, T> {
    map : HashMap<String, Box<Fn(&mut S, &mut T) + 'a>>
}

impl<'a, S, T> RustMutator<'a, S, T> {
    pub fn new() -> RustMutator<'a, S, T> {
        RustMutator {
            map : HashMap::new()
        }
    }

    /// Register a closure with a name.
    pub fn register<F: Fn(&mut S, &mut T) + 'a, N: AsRef<str>>(&mut self, name: N, closure: F) {
        self.map.insert(String::from(name.as_ref()), Box::new(closure) as Box<Fn(&mut S, &mut T) + 'a>);
    }
}

impl<'a, S, T> Scope<S, T> for RustMutator<'a, S, T> {
    fn call(&self, name : &str, state : &mut S, target : &mut T) -> io::Result<()> {
        let closure = self.map.get(name);

        if closure.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Closure not found for name"));
        }

        closure.unwrap()(state, target);

        Ok(())
    }
}
