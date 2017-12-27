//! byt - views::file
//!
//! The FileView offers operations on PieceFiles that it can render.

// EXTERNS

// LIBRARY INCLUDES
use std::fs::File;
use std::io::{BufReader, ErrorKind, Error, Result};

// SUBMODULES

// LOCAL INCLUDES
use byt::io::file::{PieceFile, Empty, empty};
use byt::render;

struct FileView<'a> {
    /// The path to the file this FileView references.
    path : Option<String>,

    /// These two feel kind of hacky right now, but they work.
    file : Option<PieceFile<'a, BufReader<File>>>,
    empty_file : Option<PieceFile<'a, Empty>>,
}

impl<'a> FileView<'a> {
    /// Make a new FileView with a predefined path. Does not attempt to open the file
    /// corresponding to the path.  You must call open() on the returned instance to do so.
    pub fn new(path : String) -> FileView<'a> {
        FileView {
            path : Option::Some(path),
            file : Option::None,
            empty_file : Option::None,
        }
    }

    /// Make a new FileView with an empty, in-memory PieceFile. When you set a path and call
    /// `open()` the file.
    pub fn empty() -> FileView<'a> {
        FileView {
            path : Option::None,
            file : Option::None,
            empty_file : Option::Some(empty().unwrap()),
        }
    }
}

impl<'a> render::Renderable for FileView<'a> {
    fn render(&self, renderer : &render::Renderer, size : (u16, u16)) -> Result<()> {
        let (rows, cols) = size;

        Ok(())
    }

    fn should_render(&self) -> bool {
        false
    }
}
