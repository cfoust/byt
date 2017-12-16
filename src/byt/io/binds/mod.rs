//! byt - io
//!
//! Structs and methods for the purpose of handling user input.

// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES

// LOCAL INCLUDES

/// Stores a reference to the action identifier or the
/// next binding table.
pub enum Next<'b> {
    Action(String),
    Table(&'b BindingTable<'b>)
}

/// The association of a key to some action.
pub struct Binding<'a> {
    pub key    : Key,
    pub result : Next<'a>
}

/// A table of bindings.
pub struct BindingTable<'a> {
    pub bindings : Vec<Binding<'a>>
}

impl<'a> BindingTable<'a> {
    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    /// Make a new BindingTable without anything in it.
    pub fn new() -> BindingTable<'a> {
        BindingTable {
            bindings : Vec::new()
        }
    }

    /// Add a binding that runs an action.
    pub fn add_action(&mut self, key : Key, action : String) {
        self.bindings.push(Binding {
            key    : key.clone(),
            result : Next::Action(action)
        });
    }

    /// Link a binding that leads to another table.
    pub fn add_table(&mut self, key : Key, table : &'a BindingTable<'a>) {
        self.bindings.push(Binding {
            key    : key.clone(),
            result : Next::Table(table)
        });
    }
}
