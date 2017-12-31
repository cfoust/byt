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
pub trait Scope<T> {
    /// Perform the function referred to by `name` on the mutable target.
    /// Will error if the name has no association.
    fn call(&self, name : &str, target : &mut T) -> io::Result<()>;
}

/// Stores and allows the invocation of any procedures defined in Rust.
/// The lifetime here allows the closures to use things in their parent
/// stacks.
pub struct RustMutator<'a, T> {
    map : HashMap<String, Box<Fn(&mut T) + 'a>>
}

impl<'a, T> RustMutator<'a, T> {
    pub fn new() -> RustMutator<'a, T> {
        RustMutator {
            map : HashMap::new()
        }
    }

    /// Register a closure with a name.
    pub fn register<F: Fn(&mut T) + 'a, S: AsRef<str>>(&mut self, name: S, closure: F) {
        self.map.insert(String::from(name.as_ref()), Box::new(closure) as Box<Fn(&mut T) + 'a>);
    }
}

impl<'a, T> Scope<T> for RustMutator<'a, T> {
    fn call(&self, name : &str, target : &mut T) -> io::Result<()> {
        let closure = self.map.get(name);

        if closure.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Closure not found for name"));
        }

        closure.unwrap()(target);

        Ok(())
    }
}
