//! byt - views::file
//!
//! The FileView offers operations on PieceFiles that it can render.

// EXTERNS

// LIBRARY INCLUDES
use std::fs::File;
use std::fs;
use std::io::{BufReader, ErrorKind, Error, Result};

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
    pub fn new(path : &str) -> Result<FileView> {
        Ok(FileView {
            path : Option::Some(String::from(path)),
            file : PieceFile::open(path).unwrap(),
        })
    }

    /// Make a new FileView with an empty, in-memory PieceFile.
    pub fn empty() -> Result<FileView> {
        Ok(FileView {
            path : Option::None,
            file : PieceFile::empty().unwrap(),
        })
    }
}

impl render::Renderable for FileView {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> Result<()> {
        let (rows, cols) = size;
        /// The maximum number of characters we could display.
        let num_characters = (rows * cols) as u64;

        let text = self.file.read(num_characters).unwrap();

        let mut counter = 1;

        for line in text.lines() {
            renderer.move_cursor(counter, 1);
            renderer.write(line);
            counter += 1;
        }

        Ok(())
    }

    fn should_render(&self) -> bool {
        true
    }
}