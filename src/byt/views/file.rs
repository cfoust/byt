//! byt - views::file
//!
//! The FileView offers operations on PieceFiles that it can render.

// EXTERNS

// LIBRARY INCLUDES
use std::cmp;
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
    cursor_loc : u64,

    /// The line number of the top of the viewport.
    /// Line numbers are zero-indexed.
    viewport_top : u64,

    /// A Vec of the locations of line ending characters.
    /// Regenerated after insertion or deletion.
    lines : Vec<u64>,

    /// Whether or not this view should be rendered after
    /// the next event.
    _should_render : bool,
}

impl FileView {
    /// Rebuild self.lines to have the proper line locations.
    fn regenerate_lines(&mut self) {
        // Read the whole piece file.
        let length = self.file.len();
        let text   = self.file.read(length).unwrap();
        let lines  = text.lines();

        self.lines.clear();

        let mut loc : u64 = 0;
        for line in lines {
            self.lines.push(loc);
            // This is terrible. Never do this. It assumes we never
            // have to deal with a line that has \r\n. There's no way
            // for us to discern whether we have one with this hacky
            // approach.
            loc += (line.len() + 1) as u64;
        }
    }

    /// Make a new FileView with a predefined path. Does not attempt to open the file
    /// corresponding to the path.  You must call open() on the returned instance to do so.
    pub fn new(path : &str) -> Result<FileView> {
        let mut view = FileView {
            path : Option::Some(String::from(path)),
            file : PieceFile::open(path).unwrap(),
            cursor_loc : 0,
            viewport_top : 0,
            lines : Vec::new(),
            _should_render : true,
        };

        view.regenerate_lines();

        Ok(view)
    }

    /// Make a new FileView with an empty, in-memory PieceFile.
    pub fn empty() -> Result<FileView> {
        Ok(FileView {
            path : Option::None,
            file : PieceFile::empty().unwrap(),
            cursor_loc : 0,
            viewport_top : 0,
            lines : Vec::new(),
            _should_render : true,
        })
    }

    /// Set the cursor's location in the file.
    pub fn set_cursor(&mut self, loc : u64) -> Result<()> {
        self.cursor_loc = cmp::min(loc, self.file.len());

        Ok(())
    }
}

impl render::Renderable for FileView {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> Result<()> {
        let (rows, cols) = size;

        // We can assume that self.cursor_loc and self.viewport_top are
        // valid because of the validation done by other methods.
        let (start_offset, length) = self.calculate_viewport(size);

        self.file.seek(SeekFrom::Start(start_offset));

        let text = self.file.read(length).unwrap();

        let (cursor_row, cursor_col) = self.calculate_cursor_pos(size);

        for line in text.lines() {
            if counter >= rows {
                break;
            }

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
