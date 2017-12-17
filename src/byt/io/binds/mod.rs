//! byt - io
//!
//! Structs and methods for the purpose of handling user input.

// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES
mod tests;

// LOCAL INCLUDES

/// Stores a reference to the action identifier or the
/// next binding table.
#[derive(Clone)]
pub enum Next<'b> {
    /// The string referring to a method e.g `next-line`.
    Action(String),
    /// A reference to another table.
    Table(&'b BindingTable<'b>),
    /// Go back to the root table.
    Root,
    /// Stay in the current table and do nothing.
    Noop
}

/// The association of a key to some action.
pub struct Binding<'a> {
    // The key that will yield the result.
    key    : Key,
    // Either an action or a table of new bindings.
    result : Next<'a>,
}

/// A table of bindings.
pub struct BindingTable<'a> {
    bindings : Vec<Binding<'a>>,
    /// Describes what happens when a key matches nothing in the list 
    /// of bindings.  If `wildcard` is an Action, it is invoked with
    /// the key.
    wildcard : Next<'a>
}

impl<'a> BindingTable<'a> {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################
    /// Make sure that a binding does not conflict with other bindings
    /// in the table.
    fn ensure_unique(&self, key : Key) {
        for binding in self.bindings.iter() {
            if key != binding.key {
                continue;
            }

            panic!("Binding already exists for key");
        }
    }
    
    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    /// Make a new BindingTable without anything in it.
    pub fn new() -> BindingTable<'a> {
        BindingTable {
            bindings : Vec::new(),
            wildcard : Next::Noop
        }
    }

    /// Add a binding that runs an action.
    pub fn add_action(&mut self, key : Key, action : String) {
        self.ensure_unique(key);
        self.bindings.push(Binding {
            key    : key.clone(),
            result : Next::Action(action)
        });
    }

    /// Link a binding that leads to another table.
    pub fn add_table(&mut self, key : Key, table : &'a BindingTable<'a>) {
        self.ensure_unique(key);
        self.bindings.push(Binding {
            key    : key.clone(),
            result : Next::Table(table)
        });
    }
}

/// Takes in keys and returns actions or tables.
pub struct BindHandler<'b> {
    current_table : &'b BindingTable<'b>,
    root_table    : &'b BindingTable<'b>,
    next_action   : String,
    has_action    : bool,
}

impl<'a> BindHandler<'a> {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################

    /// Interpret the result of a Next enum.
    fn handleNext(&mut self, next : &Next<'a>) {
        match *next {
            Next::Action(ref action) => {
                self.next_action = action.clone();
                self.has_action  = true;
            },
            Next::Table(table) => {
                self.current_table = table;
            },
            Next::Root => {
                self.current_table = self.root_table;
            }
            Next::Noop => {
            }
        }
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    pub fn new(table : &'a BindingTable<'a>) -> BindHandler<'a> {
        BindHandler {
            current_table : table,
            root_table    : table,
            next_action   : String::new(),
            has_action    : false
        }
    }

    /// Handle a key of new user input.
    pub fn consume(&mut self, key : Key) {
        let mut next = &self.current_table.wildcard;

        for binding in self.current_table.bindings.iter() {
            if key != binding.key {
                continue;
            }

            next = &binding.result;
            break;
        }

        self.handleNext(next);
    }

    /// Check whether there's an action that can be consumed.
    pub fn has_action(&self) -> bool {
        self.has_action
    }

    /// Get the action waiting to be taken.
    pub fn pop_action(&mut self) -> String {
        self.has_action = false;
        return self.next_action.clone();
    }
}
