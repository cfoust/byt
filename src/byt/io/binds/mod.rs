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
        // Note: in the future it might be good to ensure that only
        // one binding exists for a given key in the table.
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

/// Takes in keys and returns actions or tables.
pub struct BindHandler<'b> {
    current_table : &'b BindingTable<'b>,
    root_table    : &'b BindingTable<'b>,
    next_action   : &'b str,
    has_action    : bool,
}

impl<'a> BindHandler<'a> {
    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    pub fn new(table : &'a BindingTable<'a>) -> BindHandler<'a> {
        BindHandler {
            current_table : table,
            root_table    : table,
            next_action   : "",
            has_action    : false
        }
    }

    /// Handle a key of new user input.
    pub fn consume(&mut self, key : Key) {
        for binding in self.current_table.bindings.iter() {
            if key != binding.key {
                continue;
            }

            match binding.result {
                Next::Action(ref action) => {
                    self.next_action = action.as_str();
                    self.has_action  = true;
                    return;
                },
                Next::Table(table) => {
                    self.current_table = table;
                    return;
                },
                Next::Root => {
                    self.current_table = self.root_table;
                    return;
                }
            }
        }
    }

    /// Check whether an action can be consumed.
}
