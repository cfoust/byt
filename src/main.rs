// EXTERNS
extern crate libc;

// LIBRARY INCLUDES

// SUBMODULES
mod render;

// LOCAL INCLUDES
use render::*;

fn main() {
    let mut x = render::terminal::TermRenderer::new();
    let size = x.size();
    println!("{}, {}", size.row, size.col);
}
