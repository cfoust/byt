//! Houses all of the neat little tests for vym.
//!
//! These are mostly integration since that's all we really
//! care about.
#[cfg(test)]

use byt::editor::mutator::*;
use byt::views::file::FileView;

use super::*;

/// Make a FileView with vym already injected.
fn make_file() -> MutatePair<FileView> {
    let mut file = MutatePair::new(FileView::empty().unwrap());
    file.register_mutator(Box::new(Vym::new()));
    file
}

#[test]
fn it_enters_insert_mode() {
    let mut file = make_file();
}
