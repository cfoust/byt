//! byt - views::file
//!
//! The FileView offers operations on PieceFiles that it can render.

// EXTERNS

// LIBRARY INCLUDES
use std::cmp;
use std::fs::File;
use std::fs;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufReader, ErrorKind, Error, Result};
use std::io;
use termion::event::Key;

// SUBMODULES

// LOCAL INCLUDES
use byt::io::binds::Keymaster;
use byt::io::file::PieceFile;
use byt::render;
use byt::editor::{
    Action,
    Actionable,
    mutator
};
use byt::io::binds::KeyInput;

#[derive(Debug, Clone)]
/// Stores information about a line of text in the file.
struct Line {
    /// The line's offset in the file.
    offset : usize,
    /// The length of the line in bytes without its line ending characters.
    content_length : usize,
    /// The number of bytes occupied by line ending characters. This can be
    /// 0 (for end of file,) 1 (for \n,) or 2 (for \r\n.)
    line_ending_length : usize,
}

impl Line {
    /// Get the offset of the end of the line.
    fn end(&self) -> usize {
        self.offset + self.len()
    }

    /// Get the effective length of the line in bytes.
    fn len(&self) -> usize {
        self.content_length + self.line_ending_length
    }

    /// Get the offset of the start of the line.
    fn start(&self) -> usize {
        self.offset
    }
}

/// Analogous to a buffer in vim. Offers abstractions
/// over byt's PieceFile type.
pub struct FileView {
    /// The path to the file this FileView references.
    path : Option<String>,
    file : Box<PieceFile>,

    /// The location of the cursor in the file
    cursor_offset : usize,

    /// The line number of the top of the viewport.
    /// Line numbers are zero-indexed.
    viewport_top : usize,

    /// A Vec of the locations of line ending characters.
    /// Regenerated after insertion or deletion.
    lines : Vec<Line>,

    /// Whether or not this view should be rendered after
    /// the next event.
    _should_render : bool,

    /// Stores and interprets keybindings for this buffer
    /// in particular.
    keys : Keymaster,

    /// Stores an insertion as it happens so that the caller
    /// doesn't have to worry about batching together single
    /// keys.
    insertion : String,
    /// The start of the current insertion in the file, in bytes.
    /// Taken from cursor_offset.
    insert_start : usize,
}

impl FileView {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################

    /// Calculate the size of the viewport.
    fn calculate_viewport(&mut self, size : (u16, u16)) -> (usize, usize) {
        let (rows, cols) = size;
        // TODO actually enable scrolling
        let viewport_loc = 0;

        // We'll come back and add support for multibyte characters
        // at some point. For the time being I don't feel like
        // implementing it.
        (viewport_loc, cmp::min((rows * cols) as usize, self.file.len()))
    }

    /// Get the current line.
    fn current_line(&self) -> &Line {
        let offset = self.cursor_offset;

        for line in self.lines.iter() {
            if offset >= line.start() && offset < line.end() {
                return &line;
            }
        }

        return &self.lines[0];
    }

    /// Rebuild self.lines to have the proper line locations.
    fn regenerate_lines(&mut self) {
        // Read the whole piece file.
        let length = self.file.len();
        let text   = self.file.read(length).unwrap();

        self.lines.clear();

        let mut offset : usize           = 0;
        let mut num_chars : usize        = 0;
        let mut num_ending_chars : usize = 0;

        for (index, byte) in text.char_indices() {
            match byte {
                '\r' => {
                    num_ending_chars += 1;
                },
                '\n' => {
                    num_ending_chars += 1;

                    self.lines.push(Line {
                        offset,
                        content_length     : num_chars,
                        line_ending_length : num_ending_chars,
                    });

                    num_chars        = 0;
                    num_ending_chars = 0;
                },
                _ => {
                    num_chars += 1;
                }
            }

            offset += 1;
        }
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    /// Delete the character before the cursor. Works whether or not
    /// you are currently in an insertion.
    pub fn backspace(&mut self) {
        if self.cursor_offset == 0 {
            return;
        }

        if self.insertion.len() > 0 {
            self.insertion.pop();
        } else if self.cursor_offset > 0 {
            self.file.delete(self.cursor_offset - 1, 1);
        }

        self.move_left();
    }

    /// Make a new FileView with an empty, in-memory PieceFile.
    pub fn empty() -> Result<FileView> {
        Ok(FileView {
            path : Option::None,
            file : PieceFile::empty().unwrap(),
            cursor_offset : 0,
            viewport_top : 0,
            lines : Vec::new(),
            _should_render : true,
            keys  : Keymaster::new(),
            insertion : String::new(),
            insert_start : 0,
        })
    }

    /// Flush inserted characters to the underlying PieceFile.
    pub fn done_inserting(&mut self) {
        if self.insertion.len() == 0 {
            return;
        }

        self.file.insert(self.insertion.as_str(), self.insert_start);
        self.cursor_offset = self.insert_start + (self.insertion.len() as usize);

        self.insertion.clear();

        self._should_render = true;
    }

    /// Get a reference to the view's PieceFile.
    pub fn file(&self) -> &PieceFile {
        &self.file
    }

    /// Get a mutable reference to the view's PieceFile. Only call this
    /// if you're sure you know what you're doing.
    pub fn file_mut(&mut self) -> &mut PieceFile {
        &mut self.file
    }

    /// Insert a character into the PieceFile at the offset of the cursor. Does NOT actually insert
    /// the character into the underlying PieceFile until you call done_inserting().  Note that
    /// this is making a bit of a gratuitous assumption that all inserts will be done continuously
    /// in single pieces. This is true in cases like vim where there is a rigid separation between
    /// modes, but it's bad for editors that always want to be "live".
    pub fn insert(&mut self, c : char) {
        if self.insertion.len() == 0 {
            self.insert_start = self.cursor_offset;
        }

        self.insertion += &c.to_string();

        self.move_right();

        self._should_render = true;
    }

    /// Move the cursor right one.
    pub fn move_right(&mut self) {
        let max = self.file.len() + (self.insertion.len() as usize);
        self.cursor_offset = cmp::min(self.cursor_offset + 1, max);

        // TODO: Only need to rerender if the viewport has changed
        // If only the cursor moves then it's fine
        self._should_render = true;
    }

    /// Move the cursor left one.
    pub fn move_left(&mut self) {
        let current = self.cursor_offset;

        if current == 0 {
            return;
        }

        self.cursor_offset = current - 1;

        self._should_render = true;
    }

    /// Move the cursor down one.
    pub fn move_down(&mut self) {

        self._should_render = true;
    }

    /// Make a new FileView with a predefined path. Does not attempt to open the file
    /// corresponding to the path.  You must call open() on the returned instance to do so.
    pub fn new(path : &str) -> Result<FileView> {
        let mut view = FileView {
            path : Option::Some(String::from(path)),
            file : PieceFile::open(path).unwrap(),
            cursor_offset : 0,
            viewport_top : 0,
            lines : Vec::new(),
            _should_render : true,
            keys  : Keymaster::new(),
            insertion : String::new(),
            insert_start : 0,
        };

        view.regenerate_lines();

        Ok(view)
    }

    /// Set the cursor's location in the file.
    pub fn set_cursor(&mut self, loc : usize) -> Result<()> {
        self.cursor_offset = cmp::min(loc, self.file.len());

        Ok(())
    }

    /// Set the line that is the top of the viewport. Lines
    /// are zero-indexed.
    pub fn set_viewport_top(&mut self, line : usize) -> Result<()> {
        // TODO: input validation
        self.viewport_top = cmp::min(line, (self.lines.len() - 1) as usize);

        Ok(())
    }
}

impl KeyInput for FileView {
    fn consume(&mut self, key : Key) -> Option<()> {
        None
    }
}

impl Actionable for FileView {
    fn actions(&mut self) -> Vec<Action> {
        Vec::new()
    }
}

impl render::Renderable for FileView {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> Result<()> {
        let (rows, cols) = size;
        let (start_offset, length) = self.calculate_viewport(size);

        let mut text = self.file.read_at(start_offset, length).unwrap();

        if self.insertion.len() > 0 {
            text.insert_str((self.insert_start - start_offset) as usize, self.insertion.as_str());
        }

        let mut line_number = 1;

        // The offset of the current line in viewport-local space.
        let mut line_offset  = 0;

        // The offset of the cursor within the current line
        let mut cursor_line_offset = 0;

        // The file-global cursor offset. This may fall within an imaginary location in
        // the current insertion.
        let mut cursor_offset = self.cursor_offset - start_offset;
        let mut cursor_placed = false;
        // The calculated screen position of the cursor
        let mut cursor_row : u16 = 1;
        let mut cursor_col : u16 = 1;

        for line in text.lines() {
            if !cursor_placed {
                cursor_line_offset = cursor_offset - line_offset;

                if cursor_line_offset <= (line.len() as usize) {
                    cursor_row = line_number;
                    cursor_col = (cursor_line_offset + 1) as u16;
                    cursor_placed = true;
                }

                // I can't believe this fucking works flawlessly.
                // After beating my head against this problem, here we are.
                if cursor_line_offset == (line.len() + 1) as usize {
                    cursor_row = line_number + 1;
                    cursor_col = 1;
                    cursor_placed = true;
                }
            }

            renderer.move_cursor(line_number as u16, 1);
            renderer.write(line);

            line_number += 1;

            // Once again we are assuming only \n and not \r\n.
            line_offset += (line.len() + 1) as usize;
        }

        renderer.move_cursor(cursor_row, cursor_col);

        self._should_render = false;

        Ok(())
    }

    fn should_render(&self) -> bool {
        self._should_render
    }
}
