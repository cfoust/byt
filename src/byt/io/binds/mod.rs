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
pub enum Next {
    /// The string referring to a method e.g `next-line`.
    Action(String),
    /// A special action for inserting whatever key was typed.
    Insert,
    /// Pop a table of bindings off of the stack.
    Pop,
    /// Stay in the current table and do nothing.
    Nothing
}

/// The association of a key to some action.
pub struct Binding {
    // The key that will yield the result.
    key    : Key,
    // Either an action or a table of new bindings.
    result : Next,
}

/// A table of bindings.
pub struct BindingTable {
    bindings : Vec<Binding>,
    /// Describes what happens when a key matches nothing in the list 
    /// of bindings.  If `wildcard` is an Action, it is invoked with
    /// the key.
    wildcard : Next
}

impl BindingTable {
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
    pub fn new() -> BindingTable {
        BindingTable {
            bindings : Vec::new(),
            wildcard : Next::Nothing
        }
    }

    /// Add a binding to the table.
    pub fn add_binding(&mut self, binding : Binding) {
        self.ensure_unique(binding.key);
        self.bindings.push(binding);
    }
}

/// Takes in keys and returns actions or tables.
pub struct Keymaster {
    tables : Vec<BindingTable>
}

impl Keymaster {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################

    /// Interpret the result of a Next enum.
    fn handleNext(&mut self, next : Next) {
        match next {
            Next::Action(ref action) => {
            },
            Next::Pop => {
            },
            Next::Insert => {
            },
            Next::Nothing => {
            }
        }
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    pub fn new() -> Keymaster {
        Keymaster {
            tables : Vec::new()
        }
    }

    /// Handle a key of new user input.
    pub fn consume(&mut self, key : Key) {
        //let mut next = &self.current_table.wildcard;

        //for binding in self.current_table.bindings.iter() {
            //if key != binding.key {
                //continue;
            //}

            //next = &binding.result;
            //break;
        //}

        //self.handleNext(next);
    }
}
