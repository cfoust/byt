/// Tests for all of the fun editor stuff.
#[cfg(test)]

use super::*;
use super::mutator::Scope;

#[test]
fn it_uses_a_rust_closure() {
    let mut bar = 0;
    let mut rust = mutator::RustMutator::new();

    rust.register("foo", |a| {
        *a = 2;
    });

    rust.call("foo", &mut bar);
    assert_eq!(bar, 2);
}
