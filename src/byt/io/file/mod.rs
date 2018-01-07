//! byt - io
//!
//! Implements a piece table to abstract over accessing and
//! modifying a file on disk.

// EXTERNS

// LIBRARY INCLUDES
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{
    BufReader,
    Error,
    ErrorKind,
    Read,
    Result,
    Seek,
    SeekFrom,
    Write
};
use std::io;
use std::str;
use std::time;

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
    /// Convert a logical offset, which is piece-table global, to an offset
    /// inside the Piece's file.
    pub fn logical_to_file(&self, offset : u64) -> u64 {
        return (offset - self.logical_offset) + self.file_offset;
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file={:?}[{}] logical_offset={} length={}",
               self.file,
               self.file_offset,
               self.logical_offset,
               self.length)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Insert,
    Delete
}

/// Records a particular modification of the text.
#[derive(Clone)]
struct Action {
    /// The operation performed
    op     : Operation,
    /// The logical offset of the beginning of the operation
    offset : u64,
    /// For a delete operation, contains the pieces deleted
    /// For an insert operation, contains the piece that was inserted
    pieces : Vec<Piece>,
    /// The length (in bytes) of the operation
    length : u64,
    /// These two fields are indicators that the pieces inside can be
    /// merged downwards or upwards when the action is undone.
    merge_down : bool,
    merge_up   : bool,
    /// The instant this Action was initialized.
    timestamp : time::Instant,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "op={:?} {}+{} down={} up={}\n",
               self.op,
               self.offset,
               self.length,
               self.merge_down,
               self.merge_up);
        write!(f, "action pieces\n");
        for piece in &self.pieces {
            write!(f, "{}\n", piece);
        }
        write!(f, "end of action pieces\n")
    }
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
    /// The number of actions currently undone. Needed so
    /// we can track `redo`.
    history_offset : usize,
    /// The total size (in bytes) of the PieceFile.
    length : u64,
    /// The current offset for reads.
    offset : u64,
    /// Stores all current Pieces.
    piece_table : Vec<Piece>,
    /// The seekable file reader.
    reader : Option<BufReader<File>>,
}

impl fmt::Display for PieceFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PieceFile\n");
        write!(f, "history_offset={}\n", self.history_offset);
        write!(f, "length={}\n", self.length);
        write!(f, "offset={}\n", self.offset);
        write!(f, "piece_table len({})\n", self.piece_table.len());
        for piece in &self.piece_table {
            write!(f, "{}\n", piece);
        }
        write!(f, "end piece table")
        //write!(f, "actions len({})\n", self.actions.len());
        //for action in &self.actions {
            //write!(f, "{}\n", action);
        //}
        //write!(f, "end actions")
    }
}

impl PieceFile {
    // #################################
    // P R I V A T E  F U N C T I O N S
    // #################################
    /// Delete some text. Returns an action that represents
    /// the deletion.
    fn _delete(&mut self, offset : u64, length : u64) -> Action {
        let start_offset = offset;
        let end_offset   = offset + length;
        let start_index  = self.get_at_offset(start_offset);
        let end_index    = self.get_at_offset(end_offset);
        let delete_size  = ((end_index - start_index) as u64) + 1;
        let num_pieces   = (end_index - start_index) + 1;

        let mut action = Action {
            length,
            offset,
            op         : Operation::Delete,
            pieces     : Vec::new(),
            merge_down : false,
            merge_up   : false,
            timestamp  : time::Instant::now(),
        };

        // TODO: ensure we don't overflow
        self.length -= length;

        // Edge case : delete is embedded WITHIN a single piece
        if num_pieces == 1 {
            let piece              = self.piece_table[start_index].clone();
            let piece_start_offset = piece.logical_offset;
            let piece_end_offset   = piece_start_offset + piece.length;
            let upper_size         = piece_end_offset - end_offset;
            let lower_size         = start_offset - piece_start_offset;

            action.pieces.push(Piece {
                file           : piece.file,
                file_offset    : piece.file_offset + lower_size,
                length         : length,
                logical_offset : start_offset,
            });

            action.merge_up   = upper_size > 0;
            action.merge_down = lower_size > 0;

            self.piece_table.remove(start_index);

            // We insert the upper part first
            if upper_size > 0 {
                self.piece_table.insert(start_index, Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset + lower_size + length,
                    length         : upper_size,
                    logical_offset : start_offset,
                });
            }

            // Then the lower part
            if lower_size > 0 {
                self.piece_table.insert(start_index, Piece {
                    file           : piece.file,
                    file_offset    : piece.file_offset,
                    length         : lower_size,
                    logical_offset : piece.logical_offset,
                });
            }

            self.update_offsets(start_index);

            return action;
        }

        // Deletes can affect multiple pieces if the user wants
        // to delete across piece boundaries.The code below
        // doesn't result in a net positive number of pieces
        // in the piece table. We do any deletion necessary
        // in a second pass.

        // 1. Handle the piece the delete starts in.
        {
            let piece              = &mut self.piece_table[start_index];
            let piece_start_offset = piece.logical_offset;
            let piece_end_offset   = piece_start_offset + piece.length;
            let upper_size         = piece_end_offset - start_offset;
            let lower_size         = start_offset - piece_start_offset;
            piece.length = lower_size;

            if upper_size > 0 {
                action.merge_down = true;
            }

            action.pieces.push(Piece {
                file           : piece.file,
                file_offset    : piece.file_offset + lower_size,
                length         : upper_size,
                logical_offset : piece_start_offset + lower_size,
            });
        }

        // 2. Handle any pieces in between. They are deleted.
        if num_pieces > 2 {
            for index in start_index + 1 .. end_index {
                let piece = &mut self.piece_table[index];
                action.pieces.insert(0, piece.clone());
                piece.length = 0;
            }
        }

        // 3. Handle the piece the delete ends in.
        {
            let piece              = &mut self.piece_table[end_index];
            let piece_start_offset = piece.logical_offset;
            let piece_end_offset   = piece_start_offset + piece.length;
            let upper_size         = piece_end_offset - end_offset;
            let lower_size         = end_offset - piece_start_offset;

            if lower_size > 0 {
                action.merge_up = true;
            }

            action.pieces.insert(0, Piece {
                file           : piece.file,
                file_offset    : piece.file_offset,
                length         : lower_size,
                logical_offset : piece_start_offset - lower_size,
            });

            piece.file_offset += lower_size;
            piece.length       = upper_size;
        }

        // It's possible that we left zero-length pieces above. We should delete them.
        self.piece_table.retain(|ref v| v.length > 0);
        self.update_offsets(start_index);

        action
    }

    /// Get a Piece at a particular offset.
    fn get_at_offset(&self, offset : u64) -> usize {
        // TODO rewrite to be binary search as we have logical
        // offsets now
        for index in 0..self.piece_table.len() {
            let piece            = &self.piece_table[index];
            let piece_end_offset = piece.logical_offset + piece.length;

            if offset >= piece_end_offset {
                continue;
            }

            return index;
        }

        if self.piece_table.len() == 0 {
            return 0;
        }

        self.piece_table.len() - 1
    }

    /// Insert some text. Returns the action corresponding
    /// to the insert.
    pub fn _insert(&mut self, text : &str, offset : u64) -> Action {
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

        let mut action = Action {
            op     : Operation::Insert,
            pieces : Vec::new(),
            offset,
            length,
            // By convention inserts always have
            // merge_down as true. An insert always
            // splits a piece unless there isn't a piece
            // above or below it.
            merge_down : false,
            merge_up   : false,
            timestamp  : time::Instant::now(),
        };

        action.pieces.push(piece.clone());

        // There are edge cases if you do an insert
        // at the beginning or end of the file.
        if offset == 0 {
            self.piece_table.insert(0, piece);
            self.update_offsets(0);
            return action;
        }
        else if offset + length == self.length {
            piece.logical_offset = self.length - length;
            self.piece_table.push(piece);

            // No need to update offsets, this is the last piece
            return action;
        }

        // The insertion may create as many as three pieces. It can
        // split a piece that already exists into two parts and then
        // goes in between them.
        let split_index = self.get_at_offset(offset);
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

        if lower_size > 0 && upper_size > 0 {
            action.merge_down = true;
        }

        self.update_offsets(0);

        action
    }

    /// Reads characters from a piece into a destination string.
    /// The `offset` refers to logical offset in the whole piece
    /// table, not file-specific offset.
    fn read_piece(&mut self, piece : Piece, offset : u64, num_bytes : u64, dest : &mut String) {
        let piece_start_offset = piece.logical_offset;
        let mut buf            = vec![0 as u8; num_bytes as usize];

        match piece.file {
            SourceFile::Append => {
                let append_start_offset = piece.logical_to_file(offset);
                let append_end_offset   = append_start_offset + num_bytes;
                let append_bytes        = self.append_file.as_bytes();

                if append_start_offset == append_end_offset {
                    let byte = append_bytes.get(append_start_offset as usize).unwrap();
                    buf[0] = *byte;
                } else {
                    let append_slice = append_bytes
                        .get(append_start_offset as usize ..
                             append_end_offset as usize)
                        .unwrap();
                    buf.copy_from_slice(append_slice);
                }
            },
            SourceFile::Original => {
                let reader = self.reader.as_mut().unwrap();
                reader.seek(SeekFrom::Start(piece.logical_to_file(offset)));
                reader.read(buf.as_mut_slice()).unwrap();
            },
        }

        let mut index = buf.len();
        let mut converted = str::from_utf8(&buf[0..index]);

        // It's possible we'll stumble upon a boundary. If we do,
        // reduce the number of bytes until all is well.
        while converted.is_err() {
            index -= 1;
            converted = str::from_utf8(&buf[0..index]);
        }

        dest.push_str(converted.unwrap());
    }

    /// Remove any edits newer than the current, historical
    /// version of the file. E.g if the user undoes a few
    /// changes, remove their corresponding actions so
    /// they cannot be redone.
    fn remove_newer_history(&mut self) {
        while self.history_offset > 0 {
            self.actions.pop();
            self.history_offset -= 1;
        }
    }

    /// Update the logical offsets starting at a certain index.
    fn update_offsets(&mut self, start_index : usize) {
        let length = self.piece_table.len();

        // Don't do anything if this is the last index
        if length == 0 || start_index >= length {
            return;
        }

        // Don't trust the logical offset of the first piece if
        // we're updating from that.
        let mut offset = if start_index == 0 {
            0
        } else {
            self.piece_table[start_index].logical_offset
        };

        for index in start_index .. self.piece_table.len() {
            let piece = &mut self.piece_table[index];
            piece.logical_offset = offset;
            offset += piece.length;
        }
    }

    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################

    /// Delete some bytes in the PieceFile.
    pub fn delete(&mut self, offset : u64, length : u64) {
        let action = self._delete(offset, length);
        self.remove_newer_history();
        self.actions.push(action);
    }

    /// Create a new empty PieceFile.
    pub fn empty() -> io::Result<Box<PieceFile>> {
        let piece_file = PieceFile {
            actions        : Vec::new(),
            append_file    : String::new(),
            history_offset : 0,
            length         : 0,
            offset         : 0,
            piece_table    : Vec::new(),
            reader         : None,
        };

        Ok(Box::new(piece_file))
    }

    /// Insert some text. Returns the action corresponding
    /// to the insert.
    pub fn insert(&mut self, text : &str, offset : u64) {
        let action = self._insert(text, offset);
        self.remove_newer_history();
        self.actions.push(action);
    }

    /// Save the PieceFile's contents to disk. Returns
    /// the number of bytes written.
    pub fn save(&mut self) -> io::Result<(u64)> {
        if self.reader.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Empty PieceFile"));
        }

        let mut text : Box<String>;
        {
            let length = self.len();
            text = self.read_at(0, length)?;
        }

        let mut file = self.reader.as_ref().unwrap().get_ref();
        file.write_all(text.as_bytes())?;

        Ok((file.metadata().unwrap().len()))
    }

    /// Save the PieceFile to disk given a filename. Return the
    /// number of bytes written.
    pub fn save_as(&mut self, filename : &str) -> io::Result<(u64)> {
        let length = self.len();
        let text   = self.read_at(0, length)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename).unwrap();

        file.write_all(text.as_bytes())?;

        Ok((file.metadata().unwrap().len()))
    }

    /// Check if this PieceFile refers to any file.
    pub fn is_empty(&self) -> bool {
        self.reader.is_none()
    }

    /// Open a new PieceFile. If the file doesn't exist, it is created.
    pub fn open(path : &str) -> io::Result<Box<PieceFile>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path).unwrap();

        // Get the length of the file to initialize the first Piece
        let size = file.metadata().unwrap().len() as u64;

        let mut piece_file = PieceFile {
            actions        : Vec::new(),
            append_file    : String::new(),
            history_offset : 0,
            length         : size,
            offset         : 0,
            piece_table    : Vec::new(),
            reader         : Some(BufReader::new(file)),
        };

        piece_file.piece_table.push(Piece {
            file : SourceFile::Original,
            file_offset : 0,
            length : size as u64,
            logical_offset : 0,
        });

        Ok(Box::new(piece_file))
    }

    pub fn len(&self) -> u64 {
        self.length
    }

    /// Read characters from the buffer. The result is guaranteed to
    /// contain only whole UTF-8 graphemes and have at MOST `num_bytes`.
    /// If taking the provided number of bytes would result in a panic
    /// (i.e if it falls on a UTF8 grapheme boundary) then it takes
    /// one grapheme fewer.
    pub fn read(&mut self, num_bytes : u64) -> io::Result<Box<String>> {
        let mut result = Box::new(String::new());

        if num_bytes == 0 {
            return Ok(result);
        }

        let start_offset = self.offset;
        let start_index  = self.get_at_offset(start_offset);
        // Often you'll see the ending offset be exclusive of the rest of
        // the selection. Here we want it to be end inclusive, so we subtract
        // one from the offset. Off-by-one errors are hard.
        let end_offset   = self.offset + num_bytes - 1;
        let end_index    = self.get_at_offset(end_offset);
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
        if num_pieces > 2 {
            let mut piece_read_bytes   : u64;
            let mut piece_start_offset : u64;

            for index in start_index + 1 .. end_index {
                // TODO move this allocation out?
                let piece = self.piece_table[index].clone();

                piece_read_bytes   = piece.length;
                piece_start_offset = piece.logical_offset;

                self.read_piece(
                    piece,
                    piece_start_offset,
                    piece_read_bytes,
                    &mut result);
            }
        }

        // 3. Handle the piece the read ends in.
        {
            let piece = self.piece_table[end_index].clone();
            // In this specific case, we need to re-add the one back in.
            // This is because read_piece does its own subtraction by one
            // so we don't need it in this specific calculation.
            // I understand why this might be confusing, but it works.
            let piece_read_bytes   = end_offset - piece.logical_offset + 1;
            let piece_start_offset = piece.logical_offset;

            self.read_piece(
                piece,
                piece_start_offset,
                piece_read_bytes,
                &mut result);
        }

        Ok(result)
    }

    /// Read bytes from an offset.
    pub fn read_at(&mut self, offset : u64, num_bytes : u64) -> io::Result<Box<String>> {
        self.seek(SeekFrom::Start(offset));
        self.read(num_bytes)
    }

    /// Attempt to merge two pieces together given the index of the lower
    /// piece.
    fn merge_pieces(&mut self, index : usize) {
        let upper_index        = index + 1;
        let upper_piece        = self.piece_table[upper_index].clone();
        let upper_start_offset = upper_piece.file_offset;

        {
            let lower_piece        = &mut self.piece_table[index];
            let lower_end_offset   = lower_piece.file_offset + lower_piece.length;

            // Something is amiss. Don't merge noncontiguous pieces.
            if lower_end_offset != upper_start_offset {
                panic!("Attempting to merge noncontiguous pieces!");
            }

            lower_piece.length += upper_piece.length;
        }

        self.piece_table.remove(upper_index);
        self.update_offsets(index);
    }

    /// Redo an undone action.
    ///
    /// Will do nothing if there is nothing to be redone.
    pub fn redo(&mut self) {
        if self.actions.len() == 0 || self.history_offset == 0 {
            return;
        }

        let action_index = self.actions.len() - self.history_offset;
        let action       = self.actions[action_index].clone();
        let index        = self.get_at_offset(action.offset);
        let last_index   = index + action.pieces.len() - 1;

        self.history_offset -= 1;

        if action.op == Operation::Insert {
            let mut insert_string = String::new();
            let piece             = action.pieces[0].clone();
            let piece_offset      = piece.logical_offset;
            let piece_length      = piece.length;

            // Unfortunately, the easiest way to do this is to read
            // from the piece and then insert it like we do below.
            self.read_piece(
                piece,
                piece_offset,
                piece_length,
                &mut insert_string);

            self._insert(insert_string.as_str(), action.offset);
        } else {
            self._delete(action.offset, action.length);
        }
    }

    /// Undo the most recent change to the buffer.
    ///
    /// In the future it might be worthwhile to make this into
    /// a tree like vim does it, but frankly I never use that feature
    /// and don't find it that useful.
    ///
    /// Will do nothing if there is nothing to be undone.
    pub fn undo(&mut self) {
        if self.actions.len() == 0 ||
           self.history_offset == self.actions.len() {
            return;
        }

        let action_index = self.actions.len() - 1 - self.history_offset;
        let action       = self.actions[action_index].clone();
        let index        = self.get_at_offset(action.offset);
        let last_index   = index + action.pieces.len() - 1;

        self.history_offset += 1;

        if action.op == Operation::Insert {
            // Inserts are simple. Just remove the piece and merge.
            self.piece_table.remove(index);
            self.length -= action.length;
        } else {
            // Delete operations should have a list of pieces that
            // they removed.

            // There is an edge case here where the delete completely
            // removed the last part of a piece. If this happens, we
            // can't just insert its pieces at the index specified
            // by the action because that's already the subsequent
            // piece, not the piece the delete occurred in. As a result
            // we have to be careful to add the action's Piece _after_
            // the Piece in `index` and then merge them together.
            let is_single_end_delete = action.pieces.len() == 1 &&
                                       !action.merge_up;

            let insert_index = {
                // Handle edge case where the delete happened at the
                // end of a single piece
                if is_single_end_delete {
                    index + 1
                } else {
                    index
                }
            };

            for piece in action.pieces {
                self.piece_table.insert(insert_index, piece);
            }

            if is_single_end_delete {
                self.merge_pieces(index);
            }

            self.length += action.length;
        }

        // We want the piece table to look EXACTLY like it did
        // before the operation was performed.
        //
        // There aren't any real advantages to this other than
        // making me feel good for the time being.

        if action.merge_up && last_index + 1 < self.piece_table.len() {
            self.merge_pieces(last_index);
        }

        if action.merge_down && index > 0 {
            self.merge_pieces(index - 1);
        }

        // TODO make this smarter. We really don't have to start
        // from zero.
        self.update_offsets(0);
    }
}

impl Seek for PieceFile {
    fn seek(&mut self, pos : SeekFrom) -> io::Result<u64> {
        let new_offset = match pos {
            SeekFrom::Start(v)   => v as i64,
            SeekFrom::End(v)     => (self.length as i64) + v,
            SeekFrom::Current(v) => (self.offset as i64) + v,
        };

        if new_offset < 0 {
            panic!("Seek offset less than 0");
        }

        self.offset = new_offset as u64;

        Ok(self.offset)
    }
}
