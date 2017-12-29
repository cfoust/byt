//! byt - io
//!
//! Structs and methods for the purpose of handling user input.
// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;

// SUBMODULES
//mod tests;

// LOCAL INCLUDES

/// Stores some information about what should be done when
/// the state machine reaches this state. It's called an arrow
/// because it refers to the next state, even if that involves
/// sending an action up to the editor as a result.
#[derive(Clone)]
pub enum Arrow {
    /// Triggers some kind of action within the editor.
    /// In the future this will be a reference to a closure, probably.
    /// For now we just use strings for ease of testing.
    Function(String),

    /// Refers to a table within the Keymaster
    Table(usize),

    /// Go back to the root table.
    Root,

    /// Stay in the current table and do nothing.
    Nothing
}

/// The association of a key to some action.
struct Binding {
    // The key that will yield the result.
    key    : Key,
    // Either an action or a table of new bindings.
    result : Arrow,
}

impl Binding {
    pub fn new(key : Key, result : Arrow) -> Binding {
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
    /// of bindings. If `wildcard` is an Arrow, it is invoked with
    /// the key.
    wildcard : Arrow,

    /// Unique id within the Keymaster
    id : usize,
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

    /// Add a binding to the table.
    pub fn add_binding(&mut self, binding : Binding) {
        self.ensure_unique(binding.key);
        self.bindings.push(binding);
    }

    /// Get this BindingTable's unique id.
    pub fn get_id(&self) -> usize {
        self.id
    }

    /// Make a new BindingTable without anything in it.
    pub fn new(id : usize) -> BindingTable {
        BindingTable {
            bindings : Vec::new(),
            wildcard : Arrow::Nothing,
            id,
        }
    }

    /// Look through the table for any entries that match
    /// the given key or the wildcard otherwise. Return
    /// that key's action if so.
    pub fn search_key(&self, key : Key) -> Option<&Arrow> {
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

    /// Set the wildcard action.
    pub fn set_wildcard(&mut self, action : Arrow) {
        self.wildcard = action;
    }
}

/// Takes in keys and returns actions or tables.
pub struct Keymaster {
    /// The id of the current state (binding table).
    current_table : usize,

    /// Specifies the id of the next binding table that's
    /// created.
    id_counter : usize,

    /// The root table with id 0.
    root_table : BindingTable,

    /// All other binding tables. They are referred to by
    /// their id.
    tables : Vec<BindingTable>,
}

impl Keymaster {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################

    /// Get the current state (i.e the current binding table) of
    /// the Keymaster. If the current table has not been set, it
    /// will be set to the root table.
    fn get_state(&self) -> &BindingTable {
        self.get_table(self.current_table).unwrap()
    }

    /// Get a reference to a table given its id.
    fn get_table(&self, id : usize) -> Option<&BindingTable> {
        if id == 0 {
            return Some(&self.root_table)
        }

        self.tables.iter().find(|ref x| x.id == id)
    }

    /// Interpret the result of a Arrow enum. If a function
    /// was called, return its identifier.
    fn handle_action(&mut self, next : &Arrow) -> Option<String> {
        match *next {
            Arrow::Function(ref action) => {
                return Some(action.clone());
            },
            // Move to the table's state.
            Arrow::Table(ref table) => {
                let id = *table;

                if let Some(_) = self.get_table(id) {
                    self.current_table = id;
                }
            },
            Arrow::Root => {
                self.go_home();
            },
            Arrow::Nothing => {}
        }

        None
    }

    /// Make a new table with an assigned id.
    fn new_table(&mut self) -> &BindingTable {
        self.tables.push(BindingTable::new(self.id_counter));
        self.id_counter += 1;
        self.get_table(self.id_counter - 1).unwrap()
    }

    /// Attempt to get an action for a key if it is
    /// evaluated in the current binding table.
    fn search_key(&self, key : Key) -> Option<&Arrow> {
        self.get_state().search_key(key)
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    /// Handle a key of new user input. If an action got fired
    /// this method will return it.
    pub fn consume(&mut self, key : Key) -> Option<String> {
        let mut action : Arrow;

        // I kind of hate the fact that I have to clone
        // the result, but it's a product of Rust's sensible
        // rules wherein you can't have multiple mutable/immutable
        // references at once.
        {
            let result = self.search_key(key);

            if result.is_none() {
                return None
            }

            action = (*result.unwrap()).clone();
        }

        return self.handle_action(&action);
    }

    /// Return to the initial (root) state.
    pub fn go_home(&mut self) {
        self.current_table = 0;
    }

    /// Create a new Keymaster and return it.
    /// Initially there are nothing in its bindings.
    pub fn new() -> Keymaster {
        Keymaster {
            root_table : BindingTable::new(0),
            current_table : 0,
            tables : Vec::new(),
            id_counter : 1
        }
    }
}
