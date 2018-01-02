/// Tests for all of the fun editor stuff.
#[cfg(test)]

use super::*;
use super::mutator::Scope;

#[test]
fn it_uses_a_rust_closure() {
    let mut bar = 0;
    let mut rust = mutator::RustScope::new(0);

    rust.register("foo", |state, target| {
        *target = 2;
    });

    rust.call("foo", &mut bar);
    assert_eq!(bar, 2);
}

#[test]
fn it_uses_a_rust_closure_with_state() {
    let mut bar = 0;
    let mut rust = mutator::RustScope::new(false);

    rust.register("foo", |state, target| {
        *state = true;
    });

    rust.call("foo", &mut bar);
    assert!(*rust.get_state());
}
