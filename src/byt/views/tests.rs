#[cfg(test)]

use super::*;

#[test]
fn it_uses_a_rust_closure() {
    let mut bar = 0;
    let mut rust = mutator::RustScope::new(0);

    rust.register("foo", |state, target, c| {
        *target = 2;
    });

    rust.call("foo", &mut bar, Key::Char('a'));
    assert_eq!(bar, 2);
}
