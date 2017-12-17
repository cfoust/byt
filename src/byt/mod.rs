//! byt
//!
//! This library provides a way to initialize an instance of byt.
//!
//! In the future it'd be good if this included a way to change the
//! default file descriptors for STDOUT/STDERR. Then you could run
//! automated tests just by piping in a file.

// EXTERNS

// LIBRARY INCLUDES
use std::io::{Write, stdout, stdin};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion;

// SUBMODULES
mod io;

// LOCAL INCLUDES

/// Initialize and start byt.
pub fn init() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut screen = AlternateScreen::from(stdout);

    let mut table = io::binds::BindingTable::new();
    table.add_action(Key::Char('a'), String::from("foo"));
    table.add_action(Key::Char('s'), String::from("bar"));
    table.add_action(Key::Char('d'), String::from("foobar"));

    let mut handler = io::binds::BindHandler::new(&table);

    // Get the size of the terminal window.
    let (rows, cols) = termion::terminal_size().unwrap();

    // For now we just move to the center
    write!(screen, "{}", Goto(rows / 2, cols / 2));
    write!(screen, "BYT");
    screen.flush().unwrap();

    for c in stdin.keys() {
        write!(screen,
               "{}{}",
               Goto(1, 1),
               termion::clear::CurrentLine)
            .unwrap();

        let key = c.unwrap();

        handler.consume(key);

        if handler.has_action() {
            let action = handler.pop_action();
            write!(screen, "{}", action);
        }

        match key {
            Key::Char('q') => break,
            _ => {}
        }
        screen.flush().unwrap();
    }
}
