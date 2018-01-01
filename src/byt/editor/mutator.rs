//! byt - editor::mutator
//!
//! Provides abstractions over making modifications to editor state.
//! See the documentation for `Mutator`.

// EXTERNS

// LIBRARY INCLUDES
use std::collections::HashMap;
use termion::event::Key;
use std::io::{
    Error,
    ErrorKind
};
use std::io;

// SUBMODULES

// LOCAL INCLUDES
use byt::io::binds::KeyInput;
use byt::editor::Actionable;

/// Defines a way of calling some function by its identifier
/// within a given scope. The closure is given a mutable reference
/// to something of the Scope's type.
pub trait Scope<T> {
    /// Whether or not this scope has a function for a certain name.
    fn has_function(&self, name : &str) -> bool;

    /// Perform the function referred to by `name` on the mutable target.
    /// Will error if the name has no association.
    fn call(&mut self, name : &str, target : &mut T) -> io::Result<()>;
}

/// Stores and allows the invocation of any procedures defined in Rust.
/// Each closure is given a mutable reference to some kind of state storage
/// (usually a struct or even the mutator itself) and the closure's target,
/// which would be something like the editor or a pane.
pub struct RustScope<'a, S, T> {
    map : HashMap<String, Box<Fn(&mut S, &mut T) + 'a>>,
    state : S,
}

impl<'a, S, T> RustScope<'a, S, T> {
    pub fn new(state : S) -> RustScope<'a, S, T> {
        RustScope {
            map : HashMap::new(),
            state
        }
    }

    /// Get a reference to the stored state.
    pub fn get_state(&self) -> &S {
        &self.state
    }

    /// Register a closure with a name.
    pub fn register<F: Fn(&mut S, &mut T) + 'a, N: AsRef<str>>(&mut self, name: N, closure: F) {
        self.map.insert(String::from(name.as_ref()), Box::new(closure) as Box<Fn(&mut S, &mut T) + 'a>);
    }
}

impl<'a, S, T> Scope<T> for RustScope<'a, S, T> {
    fn has_function(&self, name : &str) -> bool {
        self.map.get(name).is_some()
    }

    fn call(&mut self, name : &str, target : &mut T) -> io::Result<()> {
        let closure = self.map.get(name);

        if closure.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Closure not found for name"));
        }

        closure.unwrap()(&mut self.state, target);

        Ok(())
    }
}

/// A Mutator describes a set of bindings, actions, and hooks that manipulate a n instance of a
/// type in some way.
pub trait Mutator<T>: KeyInput + Actionable + Scope<T> {}

/// A characteristic of an entity that allows state manipulation with mutators.
/// The degree to which mutators are used is up to the implementer.
pub trait Mutatable<T> {
    /// Register a mutator with this entity.
    fn register_mutator(&mut self, mutator : Box<Mutator<T>>) -> io::Result<()>;
}

pub struct MutatePair<T> 
    where T: KeyInput + Actionable + Scope<T> {
    mutators : Vec<Box<Mutator<T>>>,
    target    : T,
}

impl<T> MutatePair<T> 
    where T: KeyInput + Actionable + Scope<T> {
    pub fn new(target : T) -> MutatePair<T> {
        MutatePair {
            mutators : Vec::new(),
            target,
        }
    }
}

impl<T> KeyInput for MutatePair<T> 
    where T: KeyInput + Actionable + Scope<T> {
    fn consume(&mut self, key : Key) -> Option<()> {
        for mutator in self.mutators.iter_mut() {
            if mutator.consume(key).is_some() {
                return Some(());
            }
        }

        self.target.consume(key)
    }
}
