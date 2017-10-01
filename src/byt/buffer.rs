//! byt - buffer
//!

// EXTERNS

// LIBRARY INCLUDES
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;

// SUBMODULES

// LOCAL INCLUDES

pub struct Buffer {
    name  : String,
    lines : Vec<String>,
    cursor_row : u16,
    cursor_col : u16,
}

impl Buffer {
    /// Create a new, empty buffer.
    pub fn new(name : String) -> Buffer {
        Buffer {
            name,
            lines : Vec::new(),
            cursor_row : 0,
            cursor_col : 0,
        }
    }

    /// Create a new buffer from a file.
    pub fn from_file(filename : String) -> io::Result<Buffer> {
        let mut buffer = Buffer::new(filename.clone());

        let file = match File::open(filename) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents)?;

        for line in contents.lines() {
            buffer.lines.push(String::from(line));
        }

        Ok(buffer)
    }
}
