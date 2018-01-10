#[cfg(test)]

use super::*;
use byt::views::file::FileView;

/// Make a FileView.
fn make_file() -> FileView {
    FileView::empty().unwrap()
}

#[test]
fn it_moves_down() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    assert_eq!(file.cursor_offset, 7);
    file.set_cursor(0);
    file.move_down();
    assert_eq!(file.cursor_offset, 4);
}

#[test]
fn it_moves_up() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    assert_eq!(file.cursor_offset, 7);
    file.set_cursor(4);
    file.move_up();
    assert_eq!(file.cursor_offset, 0);
}

#[test]
fn it_moves_right() {
    let mut file = make_file();
    file.insert_str("fo");
    file.set_cursor(0);
    file.move_right();
    assert_eq!(file.cursor_offset, 1);
}

#[test]
fn it_moves_left() {
    let mut file = make_file();
    file.insert_str("fo");
    file.set_cursor(1);
    file.move_left();
    assert_eq!(file.cursor_offset, 0);
}

#[test]
fn it_deletes_this_line() {
    let mut file = make_file();
    file.insert_str("foo\nbar\nfoobar");
    file.set_cursor(0);
    file.delete_current_line();
    assert_eq!(file.len(), 10);
}

#[test]
fn it_deletes_this_line_in_empty_file() {
    let mut file = make_file();

    file.insert_str("h");
    file.delete_current_line();

    assert_eq!(file.len(), 0);
    assert_eq!(file.lines.len(), 1);

    {
        let line = &file.lines[0];
        assert_eq!(line.start(), 0);
        assert_eq!(line.end(), 0);
        assert_eq!(line.len(), 0);
    }
}
