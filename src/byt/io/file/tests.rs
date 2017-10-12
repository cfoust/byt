/// Tests for all buffer operations.
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
    assert_eq!(first_piece.file_offset, 5);
    assert_eq!(first_piece.logical_offset, 2);

    let second_piece = &action.pieces[1];
    assert_eq!(second_piece.file, SourceFile::Append);
    assert_eq!(second_piece.length, 1);
    assert_eq!(second_piece.file_offset, 1);
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

