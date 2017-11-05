/// Tests for all buffer operations.
///
/// Many of these are closer to integration
/// than unit. Frankly, the idea is to test the
/// PieceFile's API and whether it performs operations
/// correctly, not to rigidly test its implementation.
#[cfg(test)]

use super::*;

#[test]
fn it_inserts() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("foo", 0);
    file.insert("bar", 0);

    assert_eq!(file.length, 6);

    let piece_table = &file.piece_table;
    assert_eq!(piece_table.len(), 2);

    let first_element  = &piece_table[0];
    let second_element = &piece_table[1];

    assert_eq!(first_element.file_offset, 3);
    assert_eq!(first_element.logical_offset, 0);
    assert_eq!(first_element.length, 3);

    assert_eq!(second_element.file_offset, 0);
    assert_eq!(second_element.logical_offset, 3);
    assert_eq!(second_element.length, 3);

    let action = &file.actions[0];
    assert_eq!(action.op, Operation::Insert);
    assert_eq!(action.length, 3);
    assert_eq!(action.offset, 0);
}

#[test]
fn it_inserts_inside_piece() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("aa", 0);
    file.insert("b", 1);

    assert_eq!(file.length, 3);

    let piece_table = &file.piece_table;
    assert_eq!(piece_table.len(), 3);

    let first_piece  = &piece_table[0];
    let second_piece = &piece_table[1];
    let third_piece  = &piece_table[2];

    assert_eq!(first_piece.file_offset, 0);
    assert_eq!(first_piece.logical_offset, 0);
    assert_eq!(first_piece.length, 1);

    assert_eq!(second_piece.file_offset, 2);
    assert_eq!(second_piece.logical_offset, 1);
    assert_eq!(second_piece.length, 1);

    assert_eq!(third_piece.file_offset, 1);
    assert_eq!(third_piece.logical_offset, 2);
    assert_eq!(third_piece.length, 1);

    let action = &file.actions[0];
    assert_eq!(action.op, Operation::Insert);
    assert_eq!(action.length, 2);
    assert_eq!(action.offset, 0);
}

#[test]
fn it_deletes_inside_piece() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("foo", 0);
    file.delete(1, 1);

    let piece_table = &file.piece_table;
    assert_eq!(piece_table.len(), 2);
    assert_eq!(piece_table[0].length, 1);
    assert_eq!(piece_table[1].length, 1);

    let action = &file.actions[1];
    assert_eq!(action.op, Operation::Delete);
    assert_eq!(action.length, 1);
    assert_eq!(action.offset, 1);

    let piece = &action.pieces[0];
    assert_eq!(piece.file, SourceFile::Append);
    assert_eq!(piece.length, 1);
    assert_eq!(piece.file_offset, 1);
    assert_eq!(piece.logical_offset, 1);
}

#[test]
fn it_deletes_across_two_pieces() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.insert("foo", 0);
    file.delete(2,2);

    assert_eq!(file.length, 4);

    let piece_table = &file.piece_table;
    assert_eq!(piece_table.len(), 2);
    assert_eq!(piece_table[0].length, 2);
    assert_eq!(piece_table[1].length, 2);

    let action = &file.actions[2];
    assert_eq!(action.op, Operation::Delete);
    assert_eq!(action.length, 2);
    assert_eq!(action.offset, 2);

    let first_piece = &action.pieces[0];
    assert_eq!(first_piece.file, SourceFile::Append);
    assert_eq!(first_piece.length, 1);
    assert_eq!(first_piece.file_offset, 0);
    assert_eq!(first_piece.logical_offset, 2);

    let second_piece = &action.pieces[1];
    assert_eq!(second_piece.file, SourceFile::Append);
    assert_eq!(second_piece.length, 1);
    assert_eq!(second_piece.file_offset, 5);
    assert_eq!(second_piece.logical_offset, 2);
}

#[test]
fn it_deletes_across_three_pieces() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("cc", 0);
    file.insert("bb", 0);
    file.insert("aa", 0);
    file.delete(1, 4);

    assert_eq!(file.length, 2);

    let piece_table = &file.piece_table;
    assert_eq!(piece_table.len(), 2);

    let first_piece  = &piece_table[0];
    let second_piece = &piece_table[1];

    assert_eq!(first_piece.file_offset, 4);
    assert_eq!(first_piece.logical_offset, 0);
    assert_eq!(first_piece.length, 1);

    assert_eq!(second_piece.file_offset, 1);
    assert_eq!(second_piece.logical_offset, 1);
    assert_eq!(second_piece.length, 1);

    // TODO: check the action's steps
}

#[test]
fn it_reads_inside_piece() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("foobar", 0);
    let read = file.read(6).unwrap();
    assert_eq!(read.as_str(), "foobar");
}

#[test]
fn it_reads_across_two_pieces() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.insert("foo", 0);
    let read = file.read(6).unwrap();
    assert_eq!(read.as_str(), "foobar");
}

#[test]
fn it_reads_across_three_pieces() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.insert("foo", 0);
    file.insert("car", 0);

    let read = file.read(9).unwrap();
    assert_eq!(read.as_str(), "carfoobar");
}

#[test]
fn it_undoes_an_insert() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.undo();

    assert_eq!(file.piece_table.len(), 0);
}

#[test]
fn it_undoes_a_delete_in_a_single_piece() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.delete(1, 1);
    file.undo();

    assert_eq!(file.piece_table.len(), 1);

    let read = file.read(3).unwrap();
    assert_eq!(read.as_str(), "bar");
}

// It turns out there's a nasty edge case here where
// deletes will mess up if they're at the beginning
// or an end of a piece. This is because the undo
// function expects to be able to insert at a specific
// index but does not account for the absence of that
// index.

#[test]
fn it_undoes_a_delete_at_the_end_of_a_single_piece() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.delete(1, 2);
    file.undo();

    assert_eq!(file.piece_table.len(), 1);
    assert_eq!(file.piece_table[0].length, 3);
    assert_eq!(file.length, 3);

    let read = file.read(3).unwrap();
    assert_eq!(read.as_str(), "bar");
}

#[test]
fn it_undoes_a_delete_at_the_start_of_a_single_piece() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.delete(0, 2);
    file.undo();

    assert_eq!(file.piece_table.len(), 1);
    assert_eq!(file.piece_table[0].length, 3);
    assert_eq!(file.length, 3);

    let read = file.read(3).unwrap();
    assert_eq!(read.as_str(), "bar");
}

#[test]
fn it_undoes_a_delete_across_two_pieces() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.insert("foo", 0);
    file.delete(0, 6);
    assert_eq!(file.piece_table.len(), 0);
    file.undo();
    assert_eq!(file.piece_table.len(), 2);

    let read = file.read(6).unwrap();
    assert_eq!(read.as_str(), "foobar");
}

#[test]
fn it_undoes_a_delete_across_three_pieces() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.insert("foo", 0);
    file.insert("car", 0);
    file.delete(0, 9);
    file.undo();

    assert_eq!(file.piece_table.len(), 3);

    let read = file.read(9).unwrap();
    assert_eq!(read.as_str(), "carfoobar");
}

#[test]
fn it_redoes_an_insert() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("bar", 0);
    file.undo();
    assert_eq!(file.piece_table.len(), 0);
    file.redo();
    assert_eq!(file.piece_table.len(), 1);
    let read = file.read(3).unwrap();
    assert_eq!(read.as_str(), "bar");
}

#[test]
fn it_redoes_a_delete() {
    let mut file = PieceFile::empty().unwrap();
    assert_eq!(file.piece_table.len(), 0);

    file.insert("foobar", 0);
    assert_eq!(file.piece_table.len(), 1);

    file.delete(2, 4);
    assert_eq!(file.piece_table.len(), 1);
    assert_eq!(file.length, 2);

    file.undo();
    assert_eq!(file.piece_table.len(), 1);
    assert_eq!(file.length, 6);

    file.redo();
    assert_eq!(file.length, 2);
    assert_eq!(file.piece_table.len(), 1);

    let read = file.read(2).unwrap();
    assert_eq!(read.as_str(), "fo");
}
