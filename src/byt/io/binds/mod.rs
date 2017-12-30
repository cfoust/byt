//! byt - io
//!
//! Structs and methods for the purpose of handling user input.
// EXTERNS

// LIBRARY INCLUDES
use termion::event::Key;
use std::io::{
    Error,
    ErrorKind
};
use std::io;

// SUBMODULES
mod tests;

// LOCAL INCLUDES

/// Stores some information about what should be done when
/// the state machine reaches this state. It's called an arrow
/// because it refers to the next state, even if that involves
/// sending an action up to the editor as a result.
#[derive(Clone, PartialEq, Debug)]
enum Arrow {
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
struct BindingTable {
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
    fn ensure_unique(&self, key : Key) -> io::Result<()> {
        for binding in self.bindings.iter() {
            if key != binding.key {
                continue;
            }

            return Err(Error::new(ErrorKind::InvalidInput, "Not unique"));
        }

        Ok(())
    }

    /// Get this BindingTable's unique id.
    fn get_id(&self) -> usize {
        self.id
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    /// Bind some an action to a key.
    pub fn bind(&mut self, key : Key, action : Arrow)
        -> io::Result<()> {
        self.ensure_unique(key)?;

        self.bindings.push(Binding {
            key,
            result : action.clone()
        });

        Ok(())
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
    fn get_state(&mut self) -> &mut BindingTable {
        let id = self.current_table;
        self.get_table_by_id(id).unwrap()
    }

    /// Get a reference to a table given its id.
    fn get_table_by_id(&mut self, id : usize) -> Option<&mut BindingTable> {
        if id == 0 {
            return Some(&mut self.root_table)
        }

        self.tables.iter_mut().find(|ref x| x.id == id)
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

                if let Some(_) = self.get_table_by_id(id) {
                    self.current_table = id;
                }
            },
            Arrow::Root => {
                self.to_root();
            },
            Arrow::Nothing => {}
        }

        None
    }

    /// Make a new table with an assigned id.
    fn new_table(&mut self) -> &mut BindingTable {
        let id = self.id_counter;
        self.tables.push(BindingTable::new(id));
        self.id_counter += 1;
        self.get_table_by_id(id - 1).unwrap()
    }

    /// Attempt to get an action for a key if it is
    /// evaluated in the current binding table.
    fn search_key(&mut self, key : Key) -> Option<&Arrow> {
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

    /// Get the root binding table.
    pub fn get_root(&mut self) -> &mut BindingTable {
        self.get_table_by_id(0).unwrap()
    }

    /// Get a binding table according to a prefix of keys, which
    /// are evaluated starting at the root. Will only return Some
    /// if the keys evaluate to a state that is a table.
    pub fn get_prefix<T: AsRef<[Key]>>(&mut self, sequence : T) -> Option<&mut BindingTable> {
        let mut id = 0;

        for key in sequence.as_ref().iter() {
            let mut table = self.get_table_by_id(id).unwrap();
            let binding = table.search_key(*key);

            if binding.is_none() {
                return None;
            }

            match *binding.unwrap() {
                Arrow::Table(ref index) => {
                    id = *index;
                },
                _ => {
                    return None;
                }
            }
        }

        if id == 0 {
            return None;
        }

        self.get_table_by_id(id)
    }

    /// Make a new table at the end of a prefix of keys. If the arrows
    /// to reach this table from the root do not already exist, they
    /// will be created.
    /// 
    /// If you give this function a slice consisting of just Key::Char('a')
    /// and add a binding to the returned table, you can use that binding
    /// by typing 'a' then that binding.
    pub fn make_prefix<T: AsRef<[Key]>>(&mut self, prefix : T) 
        -> io::Result<usize> 
    {
        // The id of the table that does not have the prefix
        // binding.
        let mut max_id      = 0;
        let mut max_index   = 0;
        let mut should_make = false;

        // We need to find where we need to start making tables.

        for key in prefix.as_ref().iter() {
            let mut table = self.get_table_by_id(max_id).unwrap();
            let binding = table.search_key(*key);

            if binding.is_none() {
                should_make = true;
                break;
            }

            match *binding.unwrap() {
                Arrow::Table(ref index) => {
                    max_id = *index;
                },
                _ => {
                    return Err(Error::new(ErrorKind::InvalidInput, "Binding already exists"));
                }
            }

            max_index += 1;
        }

        if should_make {
            let (_, rest)   = prefix.as_ref().split_at(max_index);
            let mut last    = max_id;
            let mut current = self.id_counter;

            for key in rest.iter() {
                self.new_table();
            }

            for key in rest.iter() {
                // jenni is a qt and I am so tired of working on this
                let mut table = self.get_table_by_id(last).unwrap();
                table.bind(*key, Arrow::Table(current));

                last    = current;
                current = last + 1;
            }

            max_id = last;
        }

        Ok(max_id)
    }

    /// Return to the initial (root) state.
    pub fn to_root(&mut self) {
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
