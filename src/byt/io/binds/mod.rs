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
use byt::editor::{Action, Actionable};

/// Acts as a transition arrow between states of the state machine.
#[derive(Clone, PartialEq, Debug)]
pub enum Arrow {
    /// Triggers some kind of action within the editor.
    /// In the future this will be a reference to a closure, probably.
    /// For now we just use strings for ease of testing.
    Function(Action),

    /// Refers to a table within the Keymaster
    Table(usize),

    /// Go back to the root table.
    Root,

    /// Stay in the current table and do nothing.
    Nothing
}

pub trait KeyInput {
    /// Handle a key of new user input. If the key got consumed
    /// (i.e resulted in a state transition of some sort) then
    /// this method will return Some.
    fn consume(&mut self, key : Key) -> Option<()>;
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
    // TODO Why the heck isn't this a HashMap??
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

    /// Remove a binding from the table.
    pub fn unbind(&mut self, key : Key) -> io::Result<()> {
        let index = self.bindings
            .iter()
            .position(|val| key == val.key)
            .unwrap();

        self.bindings.remove(index);

        Ok(())
    }

    /// Get the number of bindings in this table.
    pub fn len(&self) -> usize {
        self.bindings.len()
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

    // TODO: Make the Keymaster use a HashMap instead.
    // The current approach works more or less without problems,
    // but it's effectively a reimplementation of a hash map except
    // with monotonically nondecreasing ids assigned to every
    // new table. Just because it works and is simple doesn't mean
    // it's good.
    /// All other binding tables. They are referred to by
    /// their id.
    tables : Vec<BindingTable>,

    /// Stores any action that we've generated but hasn't
    /// been consumed yet.
    actions : Vec<Action>,
}

impl Keymaster {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################

    /// Get a binding table according to a prefix of keys, which
    /// are evaluated starting at the root. Will only return Some
    /// if the keys evaluate to a state that is a table.
    fn get_prefix<T: AsRef<[Key]>>(&mut self, sequence : T) -> Option<&mut BindingTable> {
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
    fn handle_action(&mut self, next : &Arrow) -> Option<()> {
        match *next {
            Arrow::Function(ref action) => {
                self.actions.push(action.clone());
                self.to_root();
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
            Arrow::Nothing => {
                return None
            }
        }

        Some(())
    }

    /// Make a new table at the end of a prefix of keys. If the arrows
    /// to reach this table from the root do not already exist, they
    /// will be created.
    ///
    /// If you give this function a slice consisting of just Key::Char('a')
    /// and add a binding to the returned table, you can use that binding
    /// by typing 'a' then that binding.
    /// Returns the usize id of the table in this Keymaster, which you
    /// can query for with get_table_by_id().
    fn make_prefix<T: AsRef<[Key]>>(&mut self, prefix : T)
        -> io::Result<usize>
        {
            // The id of the table that does not have the prefix
            // binding.
            let mut max_id      = 0;
            let mut max_index   = 0;
            let mut should_make = false;

            if prefix.as_ref().len() == 0 {
                return Ok(max_id);
            }

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

    /// Make an arrow that runs a mutator action.
    pub fn mutator_action(&self, action : &str) -> Arrow {
        Arrow::Function(Action::Mutator(String::from(action)))
    }

    /// Bind some an action to a key sequence. Intermediate binding
    /// tables are created automatically. Will return Err if the sequence
    /// does not resolve to a table that can be created. This might happen
    /// if you try to bind `Ctrl+a b` to something and `Ctrl+a` is already
    /// bound to something that's not a table.
    pub fn bind<T: AsRef<[Key]>>(&mut self, sequence : T, action : Arrow) -> io::Result<()> {
        let sequence       = sequence.as_ref();
        let (prefix, last) = sequence.split_at(sequence.len() - 1);
        let table          = self.make_prefix(prefix)?;

        self.get_table_by_id(table).unwrap().bind(last[0], action)
    }

    /// Bind a mutator action to a sequence.
    pub fn bind_action<T: AsRef<[Key]>>(&mut self, sequence : T, action : &str) -> io::Result<()> {
        let action = self.mutator_action(action);
        self.bind(sequence, action)
    }

    /// Set a wildcard to some action in particular.
    pub fn bind_wildcard<T: AsRef<[Key]>>(&mut self, sequence : T, action : &str) -> io::Result<()> {
        let sequence       = sequence.as_ref();
        let (prefix, last) = sequence.split_at(sequence.len() - 1);
        let table          = self.make_prefix(prefix)?;
        let action         = self.mutator_action(action);

        self.get_table_by_id(table).unwrap().set_wildcard(action);
        Ok(())
    }

    /// Remove the action and any tables that reference it. Any of the action's
    /// parents back to the root table will be destroyed if they only contained
    /// this binding.
    pub fn unbind<T: AsRef<[Key]>>(&mut self, sequence : T) -> io::Result<()> {
        let sequence       = sequence.as_ref();

        for start_index in (0..sequence.len()).rev() {
            let (prefix, _) = sequence.split_at(start_index);
            let key         = sequence[start_index];

            // If the prefix doesn't have any keys, we have to use
            // the root table instead.
            let table       = if prefix.len() > 0 {
                self.get_prefix(prefix)
            } else {
                Option::Some(self.get_root())
            };

            if table.is_none() {
                return Err(Error::new(ErrorKind::InvalidInput, "Table not found for sequence"));
            }

            let table = table.unwrap();

            table.unbind(key);

            // Don't continue messing with tables and deleting
            // references if they contain other things.
            if table.len() > 0 {
                break;
            }
        }

        // TODO clear out empty tables

        Ok(())
    }

    /// Get an arrow (a binding) from a sequence of keys.
    /// We say `arrow` here because this might not be a "leaf node",
    /// or an action that results in returning to the root table.
    pub fn get_arrow<T: AsRef<[Key]>>(&mut self, sequence : T) -> Option<&Arrow> {
        let sequence       = sequence.as_ref();
        let (prefix, last) = sequence.split_at(sequence.len() - 1);
        let table          = self.get_prefix(prefix);

        if table.is_none() {
            return None;
        }

        table.unwrap().search_key(last[0])
    }

    /// Get the root binding table.
    pub fn get_root(&mut self) -> &mut BindingTable {
        self.get_table_by_id(0).unwrap()
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
            id_counter : 1,
            actions : Vec::new()
        }
    }
}

impl Actionable for Keymaster {
    fn actions(&mut self) -> Vec<Action> {
        self.actions.drain(..).collect()
    }
}

impl KeyInput for Keymaster {
    fn consume(&mut self, key : Key) -> Option<()> {
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

        self.handle_action(&action)
    }
}
