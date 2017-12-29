//! byt - views::file
//!
//! The FileView offers operations on PieceFiles that it can render.

// EXTERNS

// LIBRARY INCLUDES
use std::cmp;
use std::fs::File;
use std::fs;
use std::io::{BufReader, ErrorKind, Error, Result};
use std::io::Seek;
use std::io::SeekFrom;

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

    /// Set the line that is the top of the viewport. Lines
    /// are zero-indexed.
    pub fn set_viewport_top(&mut self, line : u64) -> Result<()> {
        // TODO: input validation
        self.viewport_top = cmp::min(line, (self.lines.len() - 1) as u64);

        Ok(())
    }

    /// Calculate the size of the viewport.
    fn calculate_viewport(&mut self, size : (u16, u16)) -> (u64, u64) {
        let (rows, cols) = size;
        let viewport_loc = self.lines[self.viewport_top as usize];

        // We'll come back and add support for multibyte characters
        // at some point. For the time being I don't feel like 
        // implementing it.
        (viewport_loc, cmp::min((rows * cols) as u64, self.file.len()))
    }

    pub fn move_right(&mut self) {
        let current = self.cursor_loc;
        self.cursor_loc = cmp::min(current + 1, self.file.len());

        // TODO: Only need to rerender if the viewport has changed
        // If only the cursor moves then it's fine
        self._should_render = true;
    }

    pub fn move_left(&mut self) {
        let current = self.cursor_loc;

        if current == 0 {
            return;
        }

        self.cursor_loc = current - 1;

        self._should_render = true;
    }
}

impl render::Renderable for FileView {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> Result<()> {
        let (rows, cols) = size;

        // We can assume that self.cursor_loc and self.viewport_top are
        // valid because of the validation done by other methods.
        let (start_offset, length) = self.calculate_viewport(size);

        let text = self.file.read_at(start_offset, length).unwrap();

        let cursor_loc = self.cursor_loc - start_offset;

        let mut counter          = 1;
        let mut loc              = 0;
        let mut cursor_row : u16 = 1;
        let mut cursor_col : u16 = 1;

        for line in text.lines() {
            if cursor_loc >= loc {
                cursor_row = counter;
                cursor_col = (cursor_loc - loc + 1) as u16;

                if cursor_col == (line.len() + 1) as u16 {
                    cursor_col -= 1;
                }
            }

            renderer.move_cursor(counter as u16, 1);
            renderer.write(line);

            counter += 1;
            loc += (line.len() + 1) as u64;
        }

        renderer.move_cursor(cursor_row, cursor_col);

        self._should_render = false;

        Ok(())
    }

    fn should_render(&self) -> bool {
        self._should_render
    }
}
