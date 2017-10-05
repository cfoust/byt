//! byt - io
//!
//! Implements a piece table to abstract over accessing and
//! modifying a file on disk.

// EXTERNS

// LIBRARY INCLUDES
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
use std::io;

// SUBMODULES

// LOCAL INCLUDES

enum SourceFile {
    /// The original file on disk.
    Original,
    /// The append file, held in memory.
    Append
}

/// A Piece stores some information about a part of the
/// modified file.
struct Piece {
    /// The file this piece refers to.
    file : SourceFile,
    /// The offset (in bytes) of the text in this Piece.
    offset : u32,
    /// The length of the text in this Piece.
    length : u32
}

/// Implements logical operations on a file that are not written
/// until asked to.
pub struct PieceFile {
    /// The in-memory append string. All edits will refer to bytes
    /// stored here.
    append_file : String,
    /// The underlying file descriptor on the filesystem.
    //file : File,
    /// The path of the file this PieceFile refers to.
    path : String,
    /// Stores all current Pieces.
    piece_table : Vec<Piece>,
    /// The seekable file reader.
    reader : BufReader<File>,
}

impl PieceFile {
    // ###############################
    // P U B L I C  F U N C T I O N S
    // ###############################
    /// Open a new PieceFile. If the file doesn't exist, it is created.
    pub fn open(path : String) -> io::Result<PieceFile> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.clone()).unwrap();

        // Get the length of the file to initialize the first Piece
        let size = file.metadata().unwrap().len();

        let mut piece_file = PieceFile {
            append_file : String::new(),
            path,
            piece_table : Vec::new(),
            reader : BufReader::new(file)
        };

        piece_file.piece_table.push(Piece {
            file : SourceFile::Original,
            offset : 0,
            length : size as u32,
        });

        Ok(piece_file)
    }

    /// Insert some text.
    pub fn insert(&mut self, text : String, offset : u32) {
    }
}
