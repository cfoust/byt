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
    file : Box<PieceFile>,

    /// The location of the cursor in the file
    loc  : u64,

    /// Whether or not this view should be rendered after
    /// the next event.
    _should_render : bool,
}

impl FileView {
    /// Make a new FileView with a predefined path. Does not attempt to open the file
    /// corresponding to the path.  You must call open() on the returned instance to do so.
    pub fn new(path : &str) -> Result<FileView> {
        Ok(FileView {
            path : Option::Some(String::from(path)),
            file : PieceFile::open(path).unwrap(),
            loc  : 0,
            _should_render : true,
        })
    }

    /// Make a new FileView with an empty, in-memory PieceFile.
    pub fn empty() -> Result<FileView> {
        Ok(FileView {
            path : Option::None,
            file : PieceFile::empty().unwrap(),
            loc  : 0,
            _should_render : true,
        })
    }
}

impl render::Renderable for FileView {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> Result<()> {
        let (rows, cols) = size;

        // The maximum number of characters we could display.
        let num_characters = (rows * cols) as u64;

        let text = self.file.read(num_characters).unwrap();

        let mut counter = 1;

        // These track the current line we're rendering
        let mut start_loc = 0;
        let mut end_loc = 0;

        let mut cursor_loc = self.loc as usize;
        let mut cursor_row : u16 = 1;
        let mut cursor_col : u16 = 1;

        for line in text.lines() {
            end_loc += line.len() + 1; // Because of the newline char

            if cursor_loc < end_loc && cursor_loc >= start_loc {
                cursor_row = counter;
                cursor_col = (cursor_loc - start_loc + 1) as u16;

                // Can't put the cursor on a newline char. Go down
                // one. This might break if you have a file that
                // uses \r\n
                if cursor_col == (line.len() + 1) as u16 {
                    cursor_col -= 1;
                }
            }

            renderer.move_cursor(counter as u16, 1);
            renderer.write(line);

            start_loc = end_loc;
            counter += 1;
        }

        renderer.move_cursor(cursor_row as u16, cursor_col as u16);

        self._should_render = false;

        Ok(())
    }

    fn should_render(&self) -> bool {
        self._should_render
    }
}
