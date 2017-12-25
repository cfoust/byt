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
pub enum Action {
    /// The string referring to a method e.g `next-line`.
    /// The function will be given the key that was pressed.
    Function(String),
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
    result : Action,
}

impl Binding {
    pub fn new(key : Key, result : Action) -> Binding {
        Binding {
            key,
            result
        }
    }
}

/// A table of bindings.
pub struct BindingTable {
    bindings : Vec<Binding>,
    /// Describes what happens when a key matches nothing in the list
    /// of bindings.  If `wildcard` is an Action, it is invoked with
    /// the key.
    wildcard : Action
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
            wildcard : Action::Nothing
        }
    }

    /// Add a binding to the table.
    pub fn add_binding(&mut self, binding : Binding) {
        self.ensure_unique(binding.key);
        self.bindings.push(binding);
    }

    /// Set the wildcard action.
    pub fn set_wildcard(&mut self, action : Action) {
        self.wildcard = action;
    }

    /// Look through the table for any entries that match
    /// the given key or the wildcard otherwise. Return
    /// that key's action if so.
    pub fn search_key(&self, key : Key) -> Option<&Action> {
        let entry = self.bindings
            .iter()
            .find(|ref x| x.key == key);

        if entry.is_none() {
            if let Action::Nothing = self.wildcard {
                return None
            }

            return Some(&self.wildcard);
        }

        Some(&entry.unwrap().result)
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
    /// Pop a table from the table stack.
    fn pop_table(&mut self) {
        self.tables.pop();
    }

    /// Search through the tables in the Keymaster for a binding
    /// that matches a key. 
    fn search_key(&mut self, key : Key) -> Option<&Action> {
        let mut action : Option<&Action> = Option::None;

        // Start from the top of the stack and go down.
        for table in self.tables.iter().rev() {
            action = table.search_key(key);

            if action.is_some() {
                break
            }
        }

        action
    }

    /// Interpret the result of a Action enum. If a function
    /// was called, return its identifier.
    fn handle_action(&mut self, next : &Action) -> Option<String> {
        match *next {
            Action::Function(ref action) => {
                // TODO: Fix this shitty string clone
                return Some(action.clone());
            },
            Action::Pop => {
                self.pop_table();
            },
            Action::Nothing => {}
        }

        None
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################
    /// Create a new Keymaster and return it.
    /// Initially there are nothing in its bindings.
    pub fn new() -> Keymaster {
        Keymaster {
            tables : Vec::new()
        }
    }

    /// Add a table of bindings to the Keymaster.
    pub fn add_table(&mut self, table : BindingTable) {
        self.tables.push(table);
    }

    /// Get a mutable reference to the table on the top of the stack.
    pub fn peek_table(&self) -> Option<&BindingTable> {
        if self.tables.len() == 0 {
            return None
        }

        // Holy shit, Rust.
        Some(&self.tables
            .iter()
            .peekable()
            .peek()
            .unwrap())
    }

    /// Handle a key of new user input. 
    pub fn consume(&mut self, key : Key) -> Option<String> {
        let mut action : Action = Action::Nothing;

        {
            let result = self.search_key(key);

            if result.is_none() {
                return None;
            }

            action = result.unwrap().clone();
        }

        return self.handle_action(&action);
    }
}
