//! byt - io
//!
//! Implements a piece table to abstract over accessing and
//! modifying a file on disk.

// EXTERNS

// LIBRARY INCLUDES
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Empty;
use std::io::BufReader;
use std::io::Read;
use std::io;

// SUBMODULES

// LOCAL INCLUDES

#[derive(Debug, Copy, Clone, PartialEq)]
enum SourceFile {
    /// The original file on disk.
    Original,
    /// The append file, held in memory.
    Append
}

/// A Piece stores some information about a part of the
/// modified file.
#[derive(Clone)]
struct Piece {
    /// The file this piece refers to.
    file : SourceFile,
    /// The offset (in bytes) of the text in this Piece in its file.
    file_offset : u32,
    /// The length of the text in this Piece.
    length : u32,
    /// The logical offset of the Piece.
    logical_offset : u32,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{} ({}){})", self.file, self.file_offset, self.logical_offset, self.length)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Insert,
    Delete
}

/// Records a particular modification of the text.
struct Action {
    op     : Operation,
    offset : u32,
    pieces : Vec<Piece>,
    length : u32
}

/// Implements logical operations on a file that are not written
/// until asked to.
pub struct PieceFile {
    /// Stores all mutation operations of the file so that we can
    /// undo.
    actions : Vec<Action>,
    /// The in-memory append string. All edits will refer to bytes
    /// stored here.
    append_file : String,
    /// The total size (in bytes) of the PieceFile.
    length : u32,
    /// The path of the file this PieceFile refers to.
    path : String,
    /// Stores all current Pieces.
    piece_table : Vec<Piece>,
    /// The seekable file reader.
    reader : Option<BufReader<File>>,
}

impl PieceFile {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################
    /// Get a Piece at a particular offset.
    fn get_at_offset(&self, offset : u32) -> Option<usize> {
        let mut logical_length = 0;

        // This has to be O(N) for the time being unless we want
        // to update all logical offsets when the table changes.
        // On modern hardware this will hardly be a bottleneck,
        // but we'll see.
        for index in 0..self.piece_table.len() {
            let piece = &self.piece_table[index];
            logical_length += piece.length;

            if offset > logical_length {
                continue;
            }

            return Some(index);
        }

        None
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################
    /// Open a new PieceFile. If the file doesn't exist, it is created.
    pub fn open(path : String) -> io::Result<PieceFile> {
        let file = OpenOptions::new()
            .read(true)
            .open(path.clone()).unwrap();

        // Get the length of the file to initialize the first Piece
        let size = file.metadata().unwrap().len() as u32;

        let mut piece_file = PieceFile {
            actions     : Vec::new(),
            append_file : String::new(),
            length      : size,
            path,
            piece_table : Vec::new(),
            reader      : Some(BufReader::new(file)),
        };

        piece_file.piece_table.push(Piece {
            file : SourceFile::Original,
            file_offset : 0,
            length : size as u32,
            logical_offset : 0,
        });

        Ok(piece_file)
    }

    /// Create a new empty PieceFile.
    pub fn empty() -> io::Result<PieceFile> {
        let mut piece_file = PieceFile {
            actions     : Vec::new(),
            append_file : String::new(),
            length      : 0,
            path        : String::from(""),
            piece_table : Vec::new(),
            reader      : None,
        };

        piece_file.piece_table.push(Piece {
            file : SourceFile::Append,
            file_offset : 0,
            length : 0,
            logical_offset : 0,
        });

        Ok(piece_file)
    }

    /// Insert some text.
    pub fn insert(&mut self, text : &str, offset : u32) {
        let length = text.len() as u32;
        let append_offset = self.append_file.len() as u32;

        self.append_file += text;
        self.length      += length;

        let mut piece = Piece {
            file   : SourceFile::Append,
            file_offset : append_offset,
            logical_offset : 0, // Unknown right now
            length,
        };

        let action = Action {
            op     : Operation::Insert,
            pieces : Vec::new(),
            offset,
            length,
        };

        self.actions.push(action);

        // There are edge cases if you do an insert
        // at the beginning or end of the file.
        if offset == 0 {
            self.piece_table.insert(0, piece);
            return;
        }
        else if offset + length == self.length {
            piece.logical_offset = self.length - length;
            self.piece_table.push(piece);
            return;
        }

        // The insertion may create as many as three pieces. It can
        // split a piece that already exists into two parts and then
        // goes in between them.
        let split_index = match self.get_at_offset(offset) {
            Some(v) => v,
            None => return // TODO: Handle this better
        };
        let split_piece = self.piece_table.remove(split_index);

        let lower_size  = offset - split_piece.logical_offset;
        let upper_size  = (split_piece.logical_offset + split_piece.length) - offset;

        if upper_size > 0 {
            self.piece_table.insert(split_index, Piece {
                file           : split_piece.file,
                file_offset    : split_piece.file_offset + lower_size,
                length         : upper_size,
                logical_offset : split_piece.logical_offset + lower_size + length,
            });
        }

        self.piece_table.insert(split_index, piece);

        if lower_size > 0 {
            self.piece_table.insert(split_index, Piece {
                file           : split_piece.file,
                file_offset    : split_piece.file_offset,
                length         : lower_size,
                logical_offset : split_piece.logical_offset,
            });
        }
    }

    /// Delete some text.
    pub fn delete(&mut self, offset : u32, length : u32) {
        let start_offset = offset;
        let end_offset   = offset + length;
        let start_index  = self.get_at_offset(start_offset).unwrap();
        let end_index    = self.get_at_offset(end_offset).unwrap();
        let delete_size  = end_index - start_index;
        let num_pieces   = (end_index - start_index) + 1;

        let mut action = Action {
            length,
            offset,
            op     : Operation::Delete,
            pieces : Vec::new()
        };

        // Deletes can affect multiple pieces if the user wants to delete
        // across piece boundaries. We have to start at the bottom index
        // and move up.
        for index in start_index..end_index {
            let mut piece          = &self.piece_table[index];
            let piece_start_offset = piece.logical_offset;
            let piece_end_offset   = piece.logical_offset + piece.length;

            // Edge case : delete is embedded WITHIN a single piece
            if start_offset > piece_start_offset && end_offset < piece_end_offset {
                self.piece_table.remove(index);
                let upper_size = piece_end_offset - end_offset;
                let lower_size = start_offset - piece_start_offset;

                action.pieces.push(Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset + (start_offset - piece.logical_offset),
                    length         : delete_size,
                    logical_offset : start_offset,
                });

                // We insert the upper part first
                self.piece_table.insert(index, Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset + lower_size + delete_size,
                    length         : upper_size,
                    logical_offset : start_offset,
                });
                // Then the lower part
                self.piece_table.insert(index, Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset,
                    length         : lower_size,
                    logical_offset : piece.logical_offset,
                });

                return
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_inserts() {
        let mut file = PieceFile::empty().unwrap();
        assert_eq!(file.piece_table.len(), 1);

        file.insert("foo", 0);
        file.insert("bar", 0);

        let piece_table = &file.piece_table;
        assert_eq!(piece_table.len(), 2);

        let first_element  = &piece_table[0];
        let second_element = &piece_table[1];

        assert_eq!(first_element.file_offset, 3);
        assert_eq!(first_element.length, 3);

        assert_eq!(second_element.file_offset, 0);
        assert_eq!(second_element.length, 3);

        let action = &file.actions[0];
        assert_eq!(action.op, Operation::Insert);

        // TODO: when read() is implemented check that this says "barfoo"
    }
}
