//! byt - views::file
//!
//! The FileView offers operations on PieceFiles that it can render.

// EXTERNS

// LIBRARY INCLUDES
use std::fs::File;
use std::fs;
use std::io::{BufReader, ErrorKind, Error, Result};
use std::io::{Read, Seek, SeekFrom};

// SUBMODULES

// LOCAL INCLUDES
use byt::io::file::PieceFile;
use byt::render;

pub struct FileView {
    /// The path to the file this FileView references.
    path : Option<String>,
    file : Box<PieceFile>
}

impl FileView {
    /// Make a new FileView with a predefined path. Does not attempt to open the file
    /// corresponding to the path.  You must call open() on the returned instance to do so.
    pub fn new(path : &str) -> FileView {
        FileView {
            path : Option::Some(String::from(path)),
            file : PieceFile::open(path).unwrap(),
        }
    }

    /// Make a new FileView with an empty, in-memory PieceFile. When you set a path and call
    /// `open()` the file.
    pub fn empty() -> FileView {
        FileView {
            path : Option::None,
            file : PieceFile::empty().unwrap(),
        }
    }
}

impl render::Renderable for FileView {
    fn render(&self, renderer : &render::Renderer, size : (u16, u16)) -> Result<()> {
        let (rows, cols) = size;

        Ok(())
    }

    fn should_render(&self) -> bool {
        false
    }
}
