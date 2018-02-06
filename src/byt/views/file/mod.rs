//! byt - views::file
//!
//! The FileView offers operations on PieceFiles, which store all of the edits made to a file in a
//! tidy data structure. The idea behind the FileView is that it provides an API for plugins to do
//! common manipulations of state in a text editor. Things like inserting, deleting, movement, and
//! search are all handled by the FileView. In a sense, that makes the struct seem rather
//! monolithic, but the whole purpose is that users of FileViews never have to thing about the
//! underlying representation on disk or otherwise. Since the goal of byt is extensibility, having
//! a common base of functionality implemented in optimized Rust makes things a lot easier.

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
use termion;

// SUBMODULES
mod tests;

// LOCAL INCLUDES
use byt::io::binds::Keymaster;
use byt::io::file::PieceFile;
use byt::render;
use byt::editor::{
    Action,
    Actionable
};
use byt::io::binds::KeyInput;

#[derive(Debug, Clone)]
/// Stores information about a line of text in the file.
pub struct Line {
    /// The line number. Starts at 1.
    number : usize,
    /// The line's offset in the file.
    offset : usize,
    /// The length of the line in bytes without its line ending characters.
    content_length : usize,
    /// The number of bytes occupied by line ending characters. This can be
    /// 0 (for end of file,) 1 (for \n,) or 2 (for \r\n.)
    line_ending_length : usize,
}

impl Line {
    /// Check whether an offset falls within this line.
    pub fn contains(&self, offset : usize) -> bool {
        offset >= self.start() && offset < self.end()
    }

    /// Get the offset of the end of the line.
    pub fn content_end(&self) -> usize {
        self.offset + self.content_length
    }

    /// Get the offset of the end of the line.
    pub fn end(&self) -> usize {
        self.offset + self.len()
    }

    /// Get the size of the line ending characters.
    pub fn end_size(&self) -> usize {
        self.line_ending_length
    }

    /// Get the effective length of the line in bytes.
    pub fn len(&self) -> usize {
        self.content_length + self.line_ending_length
    }

    /// Get the line's number.
    pub fn number(&self) -> usize {
        self.number
    }

    /// Get the offset of the start of the line.
    pub fn start(&self) -> usize {
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
    /// The number of rows the viewport had the last time
    /// this FileView was rendered.
    viewport_rows: usize,

    /// A Vec of the locations of line ending characters.
    /// Regenerated after insertion or deletion.
    lines : Vec<Line>,

    /// Whether or not the file's lines should be rendered after
    /// the next event.
    render_lines : bool,
    /// Whether or not the cursor has moved. We often don't need
    /// to rerender the lines, just the cursor, so this saves us
    /// some work.
    render_cursor : bool,

    /// Stores and interprets keybindings for this buffer
    /// in particular.
    keys : Keymaster,
}

impl FileView {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################
    /// Rebuild self.lines to have the proper line locations.
    /// May only need to be called upon file load.
    fn regenerate_lines(&mut self) {
        // Read the whole piece file.
        let length = self.file.len();
        let text   = self.file.read_at(0, length).unwrap();

        self.lines.clear();

        let mut line_number : usize      = 1;
        let mut offset : usize           = 0;
        let mut line_offset : usize      = 0;
        let mut num_chars : usize        = 0;
        let mut num_ending_chars : usize = 0;

        for (index, byte) in text.char_indices() {
            match byte {
                // TODO make sure these are sequential
                '\r' => {
                    num_ending_chars += 1;
                },
                '\n' => {
                    num_ending_chars += 1;

                    self.lines.push(Line {
                        number             : line_number,
                        offset             : line_offset,
                        content_length     : num_chars,
                        line_ending_length : num_ending_chars,
                    });

                    line_offset      = offset + num_ending_chars;
                    line_number     += 1;
                    num_chars        = 0;
                    num_ending_chars = 0;
                },
                _ => {
                    num_chars += 1;
                }
            }

            offset += 1;
        }

        self.lines.push(Line {
            number             : line_number,
            offset             : line_offset,
            content_length     : num_chars,
            line_ending_length : 0,
        });
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

        self.file.delete(self.cursor_offset - 1, 1);
        self.regenerate_lines();
        self.move_cursor_left();
        self.render_lines = true;
    }

    /// Get the current line and its index.
    pub fn current_line(&self) -> &Line {
        let offset = self.cursor_offset;

        let mut index = 0;
        // TODO make this binary search
        for line in self.lines.iter() {
            if line.contains(offset) ||
               offset == line.content_end() {
                return &line;
            }

            index += 1;
        }

        return &self.lines[0];
    }

    /// Delete some text from the file.
    pub fn delete(&mut self, offset : usize, num_bytes : usize) {
        if offset < 0 ||
            offset >= self.file.len() {
            return;
        }

        // TODO handle case where offset + num_bytes is greater than
        // the file length
        let cursor = self.cursor_offset;
        if cursor > offset {
            self.set_cursor(cursor - cmp::min(num_bytes, cursor - (offset + num_bytes)));
        }

        self.file.delete(offset, num_bytes);
        self.regenerate_lines();
        self.render_lines = true;
    }

    /// Delete the current line.
    pub fn delete_current_line(&mut self) {
        let mut offset : usize;
        let mut length : usize;

        let line = self.current_line().clone();

        if line.number() == self.lines.len() &&
           line.len() == 0 {
            self.backspace();
            return;
        }

        self.delete(line.start(), line.len());
    }

    /// Make a new FileView with an empty, in-memory PieceFile.
    pub fn empty() -> Result<FileView> {
        let mut view = FileView {
            path : Option::None,
            file : PieceFile::empty().unwrap(),
            cursor_offset : 0,
            viewport_top : 1,
            viewport_rows : 26,
            lines : Vec::new(),
            render_lines : true,
            render_cursor : true,
            keys  : Keymaster::new(),
        };

        view.regenerate_lines();

        Ok(view)
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

    /// Get a line from the file. This only includes byte offsets,
    /// so don't expect to get any text from this yet.
    pub fn get_line(&self, number : usize) -> Option<&Line> {
        if number < 1 || number > self.lines.len() {
            return None;
        }

        Some(&self.lines[number - 1])
    }

    /// Move the cursor to the beginning of the line.
    pub fn goto_line_end(&mut self) {
        let offset = self.current_line().content_end();
        self.set_cursor(offset);
    }

    /// Move the cursor to the beginning of the line.
    pub fn goto_line_start(&mut self) {
        let offset = self.current_line().start();
        self.set_cursor(offset);
    }

    /// Insert a character into the PieceFile at the offset of the cursor. Does NOT actually insert
    /// the character into the underlying PieceFile until you call done_inserting().
    pub fn insert(&mut self, c : char) {
        let offset = self.cursor_offset;
        self.file.insert(c.to_string().as_str(), offset);

        self.regenerate_lines();
        self.set_cursor(offset + 1);
        self.render_lines = true;
    }

    /// Insert a string at the offset of the cursor.
    pub fn insert_str<N: AsRef<str>>(&mut self, text: N) {
        let text = text.as_ref();

        let mut offset = self.cursor_offset;
        for c in text.chars() {
            self.insert(c);
            offset += 1;
        }
    }

    /// Get the length of the file .
    pub fn len(&self) -> usize {
        self.file.len()
    }

    /// Move the cursor a number of lines according to a delta.
    /// Negative numbers move the cursor more towards the top of
    /// the screen.
    pub fn move_cursor_vertically(&mut self, delta : i64) {
        let current_start = self.current_line().start();
        // Have to subtract by one becauase line numbers are 1-indexed.
        let index         = self.current_line().number() - 1;

        // The distance of the cursor from the line's beginning.
        let current_column = self.cursor_offset - current_start;
        let num_lines      = (self.lines.len() as i64);

        // We want to allow the user to go to one past the newline
        // characters at the end of the final line if there are any so that
        // they can delete or edit past the newline at the end of the file.
        let last           = self.lines.len() - 1;
        let last_end_chars = self.lines[last].line_ending_length;
        let last_end       = self.lines[last].end();

        // Calculate the bounded result of the move.
        let dest_index  = cmp::max(0, cmp::min(num_lines - 1, (index as i64) + delta)) as usize;
        let dest_column = cmp::min(self.lines[dest_index].content_length, current_column);

        let line_start = self.lines[dest_index].start();
        self.set_cursor(dest_column + line_start);
    }

    /// Move the cursor down one.
    pub fn move_cursor_down(&mut self) {
        self.move_cursor_vertically(1);
    }

    /// Move the cursor left one.
    pub fn move_cursor_left(&mut self) {
        let current = self.cursor_offset;
        let offset  = self.current_line().start();

        if current == offset {
            return;
        }

        self.set_cursor(current - 1);
    }

    /// Move the cursor right one.
    pub fn move_cursor_right(&mut self) {
        let current = self.cursor_offset;
        let limit   = self.current_line().content_end();

        if current == limit {
            return;
        }

        self.set_cursor(current + 1);
    }

    /// Move the cursor up one.
    pub fn move_cursor_up(&mut self) {
        self.move_cursor_vertically(-1);
    }

    /// Move the cursor to the end of the file.
    pub fn move_cursor_to_start(&mut self) {
        let change =  (-1 * (self.current_line().number() as i64)) + 1;
        self.move_cursor_vertically(change as i64);
    }

    /// Move the cursor to the end of the file.
    pub fn move_cursor_to_end(&mut self) {
        let change = self.lines.len() - self.current_line().number();
        self.move_cursor_vertically(change as i64);
    }

    /// Move the viewport up and down in the file.
    pub fn move_viewport(&mut self, delta : i64) {
        let index      = self.viewport_top as i64;
        let num_lines  = self.lines.len();
        let dest_index = cmp::min(cmp::max(1, index + delta) as usize, num_lines);
        self.set_viewport_top(dest_index);
    }

    /// Make a new FileView with a predefined path. Does not attempt to open the file
    /// corresponding to the path.  You must call open() on the returned instance to do so.
    pub fn new(path : &str) -> Result<FileView> {
        let mut view = FileView {
            path : Option::Some(String::from(path)),
            file : PieceFile::open(path).unwrap(),
            cursor_offset : 0,
            viewport_top : 1,
            viewport_rows : 26,
            lines : Vec::new(),
            render_lines : true,
            render_cursor : true,
            keys  : Keymaster::new(),
        };

        view.regenerate_lines();

        Ok(view)
    }

    /// Set the cursor's location in the file.
    pub fn set_cursor(&mut self, loc : usize) -> Result<()> {
        self.cursor_offset = loc;
        self.render_cursor = true;

        // Ensure the cursor's new position falls on a line in the
        // viewport as it was at the time of the last render.
        let line           = self.current_line().number();
        let viewport_start = self.viewport_top;
        let viewport_end   = viewport_start + self.viewport_rows - 1;

        if line < viewport_start {
            self.move_viewport((line as i64) - (viewport_start as i64));
        } if line > viewport_end {
            self.move_viewport((line as i64) - (viewport_end as i64));
        }

        Ok(())
    }

    /// Set the line that is the top of the viewport. Lines are one-indexed
    /// so the top of the viewport should be at least 1. The cursor offset
    /// is clamped to the inside of the viewport.
    pub fn set_viewport_top(&mut self, line : usize) -> Result<()> {
        self.viewport_top = cmp::max(1, cmp::min(line, self.lines.len()));

        let rows    = self.viewport_rows;
        let top     = self.viewport_top;
        let bottom  = cmp::min(top + (rows as usize) - 1, self.lines.len());
        let current = self.current_line().number;

        if current < top {
            self.move_cursor_vertically((top - current) as i64);
        } else if current > bottom {
            self.move_cursor_vertically((bottom as i64) - (current as i64));
        }

        self.render_lines = true;
        Ok(())
    }
}

impl Actionable for FileView {
    fn actions(&mut self) -> Vec<Action> {
        Vec::new()
    }
}

impl KeyInput for FileView {
    fn consume(&mut self, key : Key) -> Option<()> {
        None
    }
}

impl render::Renderable for FileView {
    fn render(&mut self, renderer : &mut render::Renderer, size : (u16, u16)) -> Result<()> {
        let (cols, rows) = size;
        let top          = self.viewport_top;
        let bottom       = cmp::min(top + (rows as usize) - 1, self.lines.len());

        self.viewport_rows = rows as usize;

        let mut line_number = 1;
        let mut cursor_offset = self.cursor_offset;
        let mut cursor_placed = false;

        // The calculated screen position of the cursor
        let mut cursor_row : u16 = 1;
        let mut cursor_col : u16 = 1;

        if self.render_lines {
            renderer.write(format!("{}", termion::clear::All).as_str());
        }

        for line in self.lines.iter() {
            // Don't render anything before the top of the viewport.
            if line.number < top {
                continue;
            }

            line_number = line.number - top + 1;

            // Don't render anything past the bottom edge of the viewport.
            if line_number > (rows as usize) {
                break;
            }

            // We need to find where the cursor is on the screen by
            // checking for it in each line we render. The cursor is just
            // a byte offset in the file; this means that this may fail if
            // the cursor is in the middle of a multibyte grapheme.
            if !cursor_placed && cursor_offset >= line.start() {
                if cursor_offset <= line.content_end() {
                    cursor_row    = line_number as u16;
                    cursor_col    = (cursor_offset - line.start() + 1) as u16;
                    cursor_placed = true;
                } else if cursor_offset == line.end() {
                    cursor_row    = (line_number + 1) as u16;
                    cursor_col    = 1;
                    cursor_placed = true;
                }
            }

            if self.render_lines {
                renderer.move_cursor(line_number as u16, 1);
                let text = self.file.read_at(line.start(), line.len() - line.end_size()).unwrap();
                renderer.write(&text);
            }
        }

        let next_line = (line_number + 1) as u16;
        if self.render_lines && next_line <= rows {
            for line in next_line .. rows + 1 {
                renderer.move_cursor(line, 1);
                renderer.write("~");
            }
        }

        self.render_lines  = false;
        self.render_cursor = false;

        renderer.move_cursor(cursor_row, cursor_col);

        Ok(())
    }

    fn should_render(&self) -> bool {
        self.render_lines || self.render_cursor
    }
}
