/// Tests for all of the fun editor stuff.
#[cfg(test)]

use termion::event::Key;
use super::*;
use byt::mutators::*;

#[test]
fn it_uses_a_rust_closure() {
    let mut bar = 0;
    let mut rust = RustScope::new(0);

    rust.register("foo", |state, target, c| {
        *target = 2;
    });

    rust.call("foo", &mut bar, Key::Char('a'));
    assert_eq!(bar, 2);
}

#[test]
fn it_uses_a_rust_closure_with_state() {
    let mut bar = 0;
    let mut rust = RustScope::new(false);

    rust.register("foo", |state, target, c| {
        *state = true;
    });

    rust.call("foo", &mut bar, Key::Char('a'));
    assert!(*rust.state());
}
