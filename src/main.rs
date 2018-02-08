// EXTERNS
#[macro_use]
extern crate hlua;
extern crate libc;
extern crate termion;

// LIBRARY INCLUDES

// SUBMODULES
mod byt;

// LOCAL INCLUDES

fn main() {
    byt::init();
}
