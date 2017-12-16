//! byt
//!
//! This library provides a way to initialize an instance of byt.
//!
//! In the future it'd be good if this included a way to change the
//! default file descriptors for STDOUT/STDERR. Then you could run
//! automated tests just by piping in a file.

// EXTERNS

// LIBRARY INCLUDES
use std::env;
use std::io::{Write, stdout, stdin};
use std::process;
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

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        screen.flush().unwrap();
    }
}
