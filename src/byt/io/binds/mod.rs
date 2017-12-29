//! byt - io
//!
//! Structs and methods for the purpose of handling user input.
// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES
mod tests;

// LOCAL INCLUDES

/// Stores some information about what should be done when
/// the state machine reaches this state. It's called an arrow
/// because it refers to the next state, even if that involves
/// sending an action up to the editor as a result.
pub enum Arrow<'a> {
    /// Triggers some kind of action within the editor.
    /// In the future this will be a reference to a closure, probably.
    /// For now we just use strings for ease of testing.
    Function(String),
    Table(&'a mut BindingTable<'a>),
    /// Stay in the current table and do nothing.
    Nothing
}

/// The association of a key to some action.
pub struct Binding<'a> {
    // The key that will yield the result.
    key    : Key,
    // Either an action or a table of new bindings.
    result : Arrow<'a>,
}

impl<'a> Binding<'a> {
    pub fn new(key : Key, result : Arrow<'a>) -> Binding<'a> {
        Binding {
            key,
            result
        }
    }
}

/// A table of bindings.
pub struct BindingTable<'a> {
    bindings : Vec<Binding<'a>>,
    /// Describes what happens when a key matches nothing in the list
    /// of bindings. If `wildcard` is an Arrow, it is invoked with
    /// the key.
    wildcard : Arrow<'a>
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
            wildcard : Arrow::Nothing
        }
    }

    /// Add a binding to the table.
    pub fn add_binding(&mut self, binding : Binding<'a>) {
        self.ensure_unique(binding.key);
        self.bindings.push(binding);
    }

    /// Set the wildcard action.
    pub fn set_wildcard(&mut self, action : Arrow<'a>) {
        self.wildcard = action;
    }

    /// Look through the table for any entries that match
    /// the given key or the wildcard otherwise. Return
    /// that key's action if so.
    pub fn search_key(&self, key : Key) -> Option<&'a Arrow> {
        let entry = self.bindings
            .iter()
            .find(|ref x| x.key == key);

        if entry.is_none() {
            if let Arrow::Nothing = self.wildcard {
                return None
            }

            return Some(&self.wildcard);
        }

        Some(&entry.unwrap().result)
    }
}

/// Takes in keys and returns actions or tables.
pub struct Keymaster<'a> {
    tables : Vec<BindingTable<'a>>
}

impl<'a> Keymaster<'a> {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################
    /// Pop a table from the table stack.
    fn pop_table(&mut self) {
        self.tables.pop();
    }

    /// Search through the tables in the Keymaster for a binding
    /// that matches a key. 
    fn search_key(&mut self, key : Key) -> Option<&'a Arrow> {
        let mut action : Option<&Arrow> = Option::None;

        // Start from the top of the stack and go down.
        for table in self.tables.iter().rev() {
            action = table.search_key(key);

            if action.is_some() {
                break
            }
        }

        action
    }

    /// Interpret the result of a Arrow enum. If a function
    /// was called, return its identifier.
    fn handle_action(&mut self, next : &Arrow) -> Option<String> {
        match *next {
            Arrow::Function(ref action) => {},
            Arrow::Table(ref table) => {},
            Arrow::Nothing => {}
        }

        None
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################
    /// Create a new Keymaster and return it.
    /// Initially there are nothing in its bindings.
    pub fn new() -> Keymaster<'a> {
        Keymaster {
            tables : Vec::new()
        }
    }

    /// Handle a key of new user input. 
    pub fn consume(&mut self, key : Key) -> Option<String> {
        let mut action : Arrow = Arrow::Nothing;

        return self.handle_action(&action);
    }
}
