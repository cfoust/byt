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
use std::sync::mpsc::channel;
use std::thread;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion;

// SUBMODULES
mod events;
mod io;
mod render;

// LOCAL INCLUDES
use self::events::*;

/// Initialize and start byt.
pub fn init() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut screen = AlternateScreen::from(stdout);

    // Make some kind of title screen.
    {
        // Get the size of the terminal window.
        let (rows, cols) = termion::terminal_size().unwrap();

        // For now we just move to the center
        write!(screen, "{}", Goto(rows / 2, cols / 2));
        write!(screen, "BYT");

        screen.flush().unwrap();
    }

    let (sender, receiver) = channel::<Event>();

    let mut key_handler = io::binds::Keymaster::new();

    {
        let mut table = io::binds::BindingTable::new();

        table.add_binding(io::binds::Binding::new(
                Key::Char('q'), 
                io::binds::Action::Function(String::from("quit")),
                ));

        table.add_binding(io::binds::Binding::new(
                Key::Char('a'), 
                io::binds::Action::Function(String::from("test")),
                ));

        key_handler.add_table(table);
    }

    // One thread just reads from user input and makes
    // events from whatever it gets.
    let key_sender = sender.clone();
    thread::spawn(move|| {
        let stdin = stdin();

        for c in stdin.keys() {
            let key = c.unwrap();
            key_sender.send(Event::KeyPress(key)).unwrap();
        }
    });

    loop {
        let event = receiver.recv().unwrap();
        let sender = sender.clone();

        if let Event::KeyPress(key) = event {
            let result = key_handler.consume(key);

            if result.is_none() {
                continue;
            }

            sender.send(Event::Function(result.unwrap()));
        }

        if let Event::Function(name) = event {
            if name == "quit" {
                break;
            }

            write!(screen, "{}", name.as_str());
            screen.flush().unwrap();
        }
    }
}
