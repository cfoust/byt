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
use std::io::Seek;
use std::io::SeekFrom;
use std::io;
use std::str;

// SUBMODULES
mod tests;

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
    file_offset : u64,
    /// The length of the text in this Piece.
    length : u64,
    /// The logical offset of the Piece.
    logical_offset : u64,
}

impl Piece {
    /// Convert a logical offset, which is piece-table global, to an offset inside
    /// the Piece's file.
    pub fn logical_to_file(&self, offset : u64) -> u64 {
        return (offset - self.logical_offset) + self.file_offset;
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{} ({}:{})", self.file, self.file_offset, self.logical_offset, self.length)
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
    offset : u64,
    pieces : Vec<Piece>,
    length : u64
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
    length : u64,
    /// The current offset for reads.
    offset : u64,
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
    fn get_at_offset(&self, offset : u64) -> Option<usize> {
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

    /// Update the logical offsets starting at a certain index.
    fn update_offsets(&mut self, start_index : usize) {
        // Don't do anything if this is the last index
        if start_index == self.piece_table.len() - 1 {
            return;
        }

        let mut offset = self.piece_table[start_index].logical_offset +
                         self.piece_table[start_index].length;

        for index in start_index + 1 .. self.piece_table.len() {
            let piece = &mut self.piece_table[index];
            piece.logical_offset = offset;
            offset += piece.length;
        }
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
        let size = file.metadata().unwrap().len() as u64;

        let mut piece_file = PieceFile {
            actions     : Vec::new(),
            append_file : String::new(),
            length      : size,
            offset      : 0,
            path,
            piece_table : Vec::new(),
            reader      : Some(BufReader::new(file)),
        };

        piece_file.piece_table.push(Piece {
            file : SourceFile::Original,
            file_offset : 0,
            length : size as u64,
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
            offset      : 0,
            path        : String::from(""),
            piece_table : Vec::new(),
            reader      : None,
        };

        Ok(piece_file)
    }

    /// Insert some text.
    pub fn insert(&mut self, text : &str, offset : u64) {
        let length = text.len() as u64;
        let append_offset = self.append_file.len() as u64;

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
            self.update_offsets(0);
            return;
        }
        else if offset + length == self.length {
            piece.logical_offset = self.length - length;
            self.piece_table.push(piece);
            // No need to update offsets, this is the last piece
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
        self.update_offsets(split_index);
    }

    /// Delete some text.
    pub fn delete(&mut self, offset : u64, length : u64) {
        let start_offset = offset;
        let end_offset   = offset + length;
        let start_index  = self.get_at_offset(start_offset).unwrap();
        let end_index    = self.get_at_offset(end_offset).unwrap();
        let delete_size  = ((end_index - start_index) as u64) + 1;
        let num_pieces   = (end_index - start_index) + 1;

        let mut action = Action {
            length,
            offset,
            op     : Operation::Delete,
            pieces : Vec::new()
        };

        // TODO: ensure we don't overflow
        self.length -= length;

        // Deletes can affect multiple pieces if the user wants to delete
        // across piece boundaries. We have to start at the bottom index
        // and move up. The code below doesn't result in a net positive number
        // of pieces in the piece table. We do any deletion necessary in a second
        // pass.
        for index in start_index .. end_index + 1 {
            let piece              = self.piece_table[index].clone();
            let piece_start_offset = piece.logical_offset;
            let piece_end_offset   = piece.logical_offset + piece.length;

            // Edge case : delete is embedded WITHIN a single piece
            if start_offset > piece_start_offset && end_offset < piece_end_offset {
                self.piece_table.remove(index);
                let upper_size = piece_end_offset - end_offset;
                let lower_size = start_offset - piece_start_offset;

                action.pieces.push(Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset + lower_size,
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
                break
            }

            // The deletion area starts in this piece and goes on to another
            if start_offset > piece_start_offset && end_offset >= piece_end_offset {
                let piece      = &mut self.piece_table[index];
                let upper_size = piece_end_offset - start_offset;
                let lower_size = start_offset - piece_start_offset;

                action.pieces.push(Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset + lower_size,
                    length         : upper_size,
                    logical_offset : piece_start_offset + lower_size,
                });

                piece.length = lower_size;
                continue;
            }

            // The deletion area finishes in this piece
            if start_offset < piece_start_offset && end_offset <= piece_end_offset {
                let piece      = &mut self.piece_table[index];
                let upper_size = piece_end_offset - end_offset;
                let lower_size = end_offset - piece_start_offset;

                action.pieces.push(Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset + lower_size,
                    length         : lower_size,
                    logical_offset : piece_start_offset - lower_size,
                });

                piece.file_offset += lower_size;
                piece.length       = upper_size;
                continue;
            }

            // The deletion area is outside the bounds of this piece
            if start_offset <= piece_start_offset && end_offset >= piece_end_offset {
                let piece = &mut self.piece_table[index];
                action.pieces.push(piece.clone());
                piece.length = 0;
            }
        }

        self.actions.push(action);

        // It's possible that we left zero-length pieces above. We should delete them.
        self.piece_table.retain(|ref v| v.length > 0);
        self.update_offsets(start_index);
    }

    /// Read characters from the buffer. The result is guaranteed to contain only
    /// whole UTF-8 graphemes, but the result is guaranteed to have at MOST
    /// `num_bytes`. If taking the provided number of bytes would result in a panic
    /// (i.e if it falls on a UTF8 grapheme boundary) then it takes one grapheme
    /// fewer.
    pub fn read(&mut self, num_bytes : u64) -> io::Result<Box<String>> {
        let mut result = Box::new(String::new());

        let start_offset = self.offset;
        let start_index  = self.get_at_offset(start_offset).unwrap();
        let end_offset   = self.offset + num_bytes;
        let end_index    = self.get_at_offset(end_offset).unwrap();
        let num_pieces   = end_index - start_index + 1;

        // Same as delete. There is an edge case where we read solely inside
        // of a piece.
        if num_pieces == 1 {
            let piece = self.piece_table[start_index].clone();
            self.read_piece(piece, start_offset, num_bytes, &mut result);
            return Ok(result);
        }

        // 1. Handle the piece the read starts in.
        {
            let piece = self.piece_table[start_index].clone();
            let piece_end_offset = piece.logical_offset + piece.length;
            let piece_read_bytes = piece_end_offset - start_offset;

            self.read_piece(piece, start_offset, piece_read_bytes, &mut result);
        }

        // 2. Handle all of the pieces between the first and the last.
        if (num_pieces > 2) {
            let mut piece_read_bytes   : u64;
            let mut piece_start_offset : u64;

            for index in start_index + 1 .. end_index {
                // TODO move this allocation out?
                let piece = self.piece_table[index].clone();

                piece_read_bytes   = piece.length;
                piece_start_offset = piece.logical_offset;

                self.read_piece(piece, piece_start_offset, piece_read_bytes, &mut result);
            }
        }

        // 3. Handle the piece the read ends in.
        {
            let piece = self.piece_table[end_index].clone();
            let piece_read_bytes   = end_offset - piece.logical_offset;
            let piece_start_offset = piece.logical_offset;

            self.read_piece(piece, piece_start_offset, piece_read_bytes, &mut result);
        }

        Ok(result)
    }

    /// Reads characters from a piece into a destination string. The `offset` refers to
    /// logical offset in the whole piece table, not file-specific offset.
    fn read_piece(&mut self, piece : Piece, offset : u64, num_bytes : u64, dest : &mut String) {
        let piece_start_offset = piece.logical_offset;
        let piece_end_offset   = piece_start_offset + piece.length - 1;
        let mut buf = vec![0 as u8; num_bytes as usize];

        match piece.file {
            SourceFile::Append => {
                let append_start_offset   = piece.logical_to_file(offset);
                let mut append_end_offset = append_start_offset + num_bytes;
                let append_bytes          = self.append_file.as_bytes();

                let append_slice = append_bytes
                    .get(append_start_offset as usize ..
                         append_end_offset as usize)
                    .unwrap();

                buf.clone_from_slice(append_slice);
            },
            SourceFile::Original => {
                let mut reader = self.reader.as_mut().unwrap();
                reader.read(buf.as_mut_slice());
            },
        }

        let mut slice = buf.as_slice();
        let mut index = buf.len();
        let mut converted = str::from_utf8(&buf[0..index]);

        // It's possible we'll stumble upon a boundary. If we do, reduce the number of
        // bytes until all is well.
        while converted.is_err() {
            index -= 1;
            converted = str::from_utf8(&buf[0..index]);
        }

        dest.push_str(converted.unwrap());
    }
}

impl Seek for PieceFile {
    fn seek(&mut self, pos : SeekFrom) -> io::Result<u64> {
        let newOffset = match pos {
            SeekFrom::Start(v)   => v as i64,
            SeekFrom::End(v)     => (self.length as i64) + v,
            SeekFrom::Current(v) => (self.offset as i64) + v,
        };

        if newOffset < 0 {
            panic!("Seek offset less than 0");
        }

        Ok(newOffset as u64)
    }
}
