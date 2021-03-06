/// Tests for all binding-related logic.
#[cfg(test)]

use super::*;

mod tables {
    use super::*;

    #[test]
    fn it_returns_no_empty_wildcard() {
        let mut table = BindingTable::new(0);
        assert!(table.search_key(Key::Char('b')).is_none());
    }

    #[test]
    fn it_returns_the_wildcard() {
        let mut table = BindingTable::new(0);

        table.set_wildcard(Arrow::Function(Action::Mutator(String::from("Blah"))));

        assert!(table.search_key(Key::Char('b')).is_some());
    }

    #[test]
    fn it_finds_a_binding() {
        let mut table = BindingTable::new(0);

        table.bind(Key::Char('a'), Arrow::Nothing);

        assert!(table.search_key(Key::Char('a')).is_some());
    }
}

#[test]
fn it_finds_a_binding() {
    let mut master = Keymaster::new();

    master
        .get_root()
        .bind(Key::Char('a'), Arrow::Nothing);

    assert!(master.search_key(Key::Char('a')).is_some());
}

#[test]
fn it_finds_a_wildcard() {
    let mut master = Keymaster::new();

    master
        .get_root()
        .set_wildcard(Arrow::Function(Action::Mutator(String::from("Blah"))));

    assert!(master.search_key(Key::Char('a')).is_some());
}

#[test]
fn it_binds_a_sequence() {
    let mut master = Keymaster::new();
    let seq = [Key::Char('b'), Key::Char('a'), Key::Char('r')];

    master.bind(seq, Arrow::Table(0));

    assert!(master.consume(Key::Char('b')).is_some());
    assert!(master.consume(Key::Char('a')).is_some());
    assert!(master.consume(Key::Char('r')).is_some());
}

#[test]
fn it_unbinds_a_sequence() {
    let mut master = Keymaster::new();
    let seq = [Key::Char('b'), Key::Char('a'), Key::Char('r')];

    master.bind(seq, Arrow::Table(0));

    assert!(master.unbind(seq).is_ok());
    assert!(master.consume(Key::Char('b')).is_none());
    assert!(master.consume(Key::Char('a')).is_none());
    assert!(master.consume(Key::Char('r')).is_none());
}

#[test]
/// Ensure that the Keymaster creates all of the tables as necessary
/// and then responds to input that uses them properly when using
/// the make_prefix function.
fn it_handles_depth() {
    let mut master = Keymaster::new();

    let id = master.make_prefix([Key::Char('b'), Key::Char('a')]).unwrap();

    assert_eq!(master.tables.len(), 2);

    {
        let table = &master.root_table.bindings;
        assert_eq!(table.len(), 1);
        assert_eq!(table[0].key, Key::Char('b'));
        assert_eq!(table[0].result, Arrow::Table(1));
    }

    {
        let table = &master.tables[0].bindings;
        assert_eq!(table.len(), 1);
        assert_eq!(table[0].key, Key::Char('a'));
        assert_eq!(table[0].result, Arrow::Table(2));
    }

    assert!(master.get_table_by_id(id).is_some());

    master
        .get_table_by_id(id)
        .unwrap()
        .bind(Key::Char('r'), Arrow::Function(Action::Mutator(String::from("Blah"))));

    master.consume(Key::Char('b'));
    master.consume(Key::Char('a'));

    assert_eq!(master.current_table, 2);
    assert!(master.consume(Key::Char('r')).is_some());
}
