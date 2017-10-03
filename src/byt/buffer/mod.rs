//! byt - buffer
//!
//! Buffers manipulate and store information in a body of text. They're
//! the bread and butter (buffer?) of the editor.

// EXTERNS
extern crate regex;

// LIBRARY INCLUDES
use self::regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io;

// SUBMODULES

// LOCAL INCLUDES

enum SymbolType {
    Whitespace,
    LineEnding,
    Other,
    EOF,
}

struct Symbol {
    offset : u32,
    data   : String,
    kind   : SymbolType,
}

/// Akin to a vim buffer. Stores text and defines operations over
/// that text.
pub struct Buffer {
    name  : String,
    lines : Vec<Vec<Symbol>>,
    cursor_offset : u32,
}

impl Buffer {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################

    /// Convert the text into Symbols and store them in the buffer.
    fn scan_text(&mut self, text : String) {
        // I'm a bit embarrassed by this whole method, but at least
        // it works and does exactly what it is designed to do.
        let mut offset = 0;
        let mut symbols = 0;

        let space = Regex::new(r"^[\s]+").unwrap();
        let line_ending = Regex::new(r"^(\r\n|\r|\n)").unwrap();
        let other = Regex::new(r"^([\w]+|[~!@#$%^&*()_+{}\-=\|;:'`,<.>/?])").unwrap();

        let mut current_line = Vec::new();
        loop {
            if offset >= text.len() {
                return;
            }

            // TODO: in the future, make this less ugly and repetitive.
            match other.find(&text[offset..]) {
                Some(x) => {
                    current_line.push(Symbol {
                        offset : offset as u32,
                        data : String::from(x.as_str()),
                        kind : SymbolType::Other,
                    });
                    offset += x.end() - x.start();
                    symbols += 1;
                    continue;
                },
                None => {},
            };

            match line_ending.find(&text[offset..]) {
                Some(x) => {
                    current_line.push(Symbol {
                        offset : offset as u32,
                        data : String::from(x.as_str()),
                        kind : SymbolType::LineEnding,
                    });
                    offset += x.end() - x.start();
                    symbols += 1;

                    // Push this line and start a new one
                    self.lines.push(current_line);
                    current_line = Vec::new();
                    continue;
                },
                None => {},
            };

            match space.find(&text[offset..]) {
                Some(x) => {
                    current_line.push(Symbol {
                        offset : offset as u32,
                        data : String::from(x.as_str()),
                        kind : SymbolType::Whitespace,
                    });
                    offset += x.end() - x.start();
                    symbols += 1;
                    continue;
                },
                None => {},
            };
        }
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################
    /// Create a new, empty buffer.
    pub fn new(name : String) -> Buffer {
        Buffer {
            name,
            lines : Vec::new(),
            cursor_offset : 0,
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

        // TODO: Come back and fix this so we don't have to read the whole file at once.
        buf_reader.read_to_string(&mut contents)?;
        buffer.scan_text(contents);

        Ok(buffer)
    }
}
