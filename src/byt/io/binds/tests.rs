/// Tests for all binding-related logic.
#[cfg(test)]

use super::*;

mod tables {
    use super::*;

    #[test]
    fn it_returns_no_empty_wildcard() {
        let mut table = BindingTable::new();
        assert!(table.search_key(Key::Char('b')).is_none());
    }

    #[test]
    fn it_returns_the_wildcard() {
        let mut table = BindingTable::new();

        table.set_wildcard(Action::Function(String::from("foobar")));

        assert!(table.search_key(Key::Char('b')).is_some());
    }

    #[test]
    fn it_finds_a_binding() {
        let mut table = BindingTable::new();

        table.add_binding(Binding {
            key : Key::Char('a'),
            result : Action::Nothing
        });

        assert!(table.search_key(Key::Char('a')).is_some());
    }
}

#[test]
fn it_finds_a_binding() {
    let mut master = Keymaster::new();
    let mut table = BindingTable::new();

    table.add_binding(Binding {
        key : Key::Char('a'),
        result : Action::Nothing
    });

    master.add_table(table);

    assert!(master.search_key(Key::Char('a')).is_some());
}

#[test]
fn it_finds_a_lower_binding() {
    let mut master = Keymaster::new();
    let mut global = BindingTable::new();

    global.add_binding(Binding {
        key : Key::Char('a'),
        result : Action::Nothing
    });

    master.add_table(global);
    
    // We want to ensure the master checks tables lower
    // in the stack if it doesn't find anything.
    master.add_table(BindingTable::new());

    assert!(master.search_key(Key::Char('a')).is_some());
}

#[test]
fn it_finds_a_wildcard() {
    let mut master = Keymaster::new();

    let mut table = BindingTable::new();
    table.set_wildcard(Action::Function(String::from("foobar")));

    master.add_table(table);

    assert!(master.search_key(Key::Char('a')).is_some());
}

#[test]
fn it_pops_a_table() {
    let mut master = Keymaster::new();

    let mut first = BindingTable::new();
    first.add_binding(Binding {
        key : Key::Char('a'),
        result : Action::Nothing
    });
    master.add_table(first);

    let mut second = BindingTable::new();
    second.add_binding(Binding {
        key : Key::Char('a'),
        result : Action::Pop
    });
    master.add_table(second);

    master.consume(Key::Char('a'));
    assert!(master.tables.len() == 1);
}
