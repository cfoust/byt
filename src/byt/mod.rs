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
mod editor;
mod events;
mod io;
mod mutators;
mod render;
mod views;

// LOCAL INCLUDES
use byt::editor::{Action, Actionable, Editor};
use byt::editor::mutator::*;
use byt::io::binds::KeyInput;
use byt::io::file;
use byt::render::Renderable;
use byt::mutators::vym::Vym;
use self::events::*;

pub fn render(mut screen : &mut Write, editor : &mut MutatePair<Editor>) {
    let size = termion::terminal_size().unwrap();

    // Clear the screen before rendering
    write!(screen, "{}", termion::clear::All);

    {
        let mut renderer = render::terminal::TermRenderer::new(&mut screen);
        editor.render(&mut renderer, size);
    }

    screen.flush().unwrap();
}

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

    let mut editor = MutatePair::new(editor::Editor::new());
    editor.target_mut().open_empty();

    editor
        .target_mut()
        .current_file()
        .unwrap()
        .register_mutator(Box::new(Vym::new()));

    render(&mut screen, &mut editor);

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
            // Just for now while I mess with other things
            if key == Key::Char('q') {
                break;
            }

            let result = editor.consume(key);

            if result.is_none() {
                continue;
            }

            for action in editor.actions() {
                if let Action::Mutator(name) = action {
                    editor.call_action(name.as_str());
                }
            }
        }

        // Check if we should render
        if !editor.should_render() {
            continue;
        }

        render(&mut screen, &mut editor);
    }
}
