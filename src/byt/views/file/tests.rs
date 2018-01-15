#[cfg(test)]

use super::*;
use byt::views::file::FileView;

/// Make a FileView.
fn make_file() -> FileView {
    FileView::empty().unwrap()
}

#[test]
fn it_inserts_a_character() {
    let mut file = make_file();
    file.insert('a');
    assert_eq!(file.cursor_offset, 1);
    assert_eq!(file.len(), 1);
    assert_eq!(file.lines.len(), 1);

    {
        let line = &file.lines[0];
        assert_eq!(line.start(), 0);
        assert_eq!(line.end(), 1);
        assert_eq!(line.len(), 1);
    }
}

#[test]
fn it_inserts_a_string() {
    let mut file = make_file();
    file.insert_str("a");
    assert_eq!(file.cursor_offset, 1);
    assert_eq!(file.len(), 1);
    assert_eq!(file.lines.len(), 1);

    {
        let line = &file.lines[0];
        assert_eq!(line.start(), 0);
        assert_eq!(line.end(), 1);
        assert_eq!(line.len(), 1);
    }
}

#[test]
fn it_moves_down() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    assert_eq!(file.cursor_offset, 7);
    file.set_cursor(0);
    file.move_cursor_down();
    assert_eq!(file.cursor_offset, 4);
}

#[test]
fn it_moves_down_to_CR() {
    let mut file = make_file();
    file.insert_str("foo\n");
    file.set_cursor(3);
    file.move_cursor_down();
    assert_eq!(file.cursor_offset, 4);
}

#[test]
fn it_doesnt_move_down_past_end() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    file.set_cursor(4);
    file.move_cursor_down();
    assert_eq!(file.cursor_offset, 4);
}

#[test]
fn it_moves_up() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    assert_eq!(file.cursor_offset, 7);
    file.set_cursor(4);
    file.move_cursor_up();
    assert_eq!(file.cursor_offset, 0);
}

#[test]
fn it_moves_right() {
    let mut file = make_file();
    file.insert_str("fo");
    file.set_cursor(0);
    file.move_cursor_right();
    assert_eq!(file.cursor_offset, 1);
}

#[test]
fn it_gets_the_current_line() {
    let mut file = make_file();
    file.insert_str("this is a test line");
    assert_eq!(file.current_line().number(), 1);
}

#[test]
fn it_gets_the_current_line_of_many() {
    let mut file = make_file();
    file.insert_str("a\nb");
    file.set_cursor(3);
    assert_eq!(file.current_line().number(), 2);
}

#[test]
fn it_moves_to_line_start() {
    let mut file = make_file();
    file.insert_str("this is a test line");
    file.goto_line_start();
    assert_eq!(file.cursor_offset, 0);
}

#[test]
fn it_moves_left() {
    let mut file = make_file();
    file.insert_str("fo");
    file.set_cursor(1);
    file.move_cursor_left();
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
fn it_deletes_a_lower_line() {
    let mut file = make_file();

    file.insert_str("foo\nbar\nfoobar");
    file.set_cursor(4);
    assert_eq!(file.current_line().number(), 2);
    assert_eq!(file.len(), 14);

    file.delete_current_line();
    assert_eq!(file.len(), 10);

    let read = file.file.read_at(0, 10).unwrap();
    assert_eq!(read.len(), 10);
    assert_eq!(read.as_str(), "foo\nfoobar");

    assert_eq!(file.cursor_offset, 4);
    assert_eq!(file.len(), 10);
}

#[test]
fn it_deletes_this_line_in_empty_file() {
    let mut file = make_file();

    file.insert_str("h");
    assert_eq!(file.len(), 1);
    assert_eq!(file.lines.len(), 1);

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

#[test]
fn it_deletes_the_last_empty_line() {
    let mut file = make_file();

    file.insert_str("foo\nbar\n");
    file.delete_current_line();
    let read = file.file.read_at(0, 7).unwrap();
    assert_eq!(read.as_str(), "foo\nbar");
}

#[test]
fn it_moves_viewport_down() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    file.set_cursor(0);
    file.move_viewport(1);
    assert_eq!(file.viewport_top, 2);
}

#[test]
fn it_moves_viewport_up() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    file.set_viewport_top(2);
    file.move_viewport(-1);
    assert_eq!(file.viewport_top, 1);
}

#[test]
fn it_doesnt_fail_to_move_with_invalid_delta() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    file.set_viewport_top(2);
    file.move_viewport(-3);
    assert_eq!(file.viewport_top, 1);
}

#[test]
fn it_moves_the_cursor_too() {
    let mut file = make_file();
    file.insert_str("foo\nbar");
    file.set_cursor(0);
    file.move_viewport(1);
    assert_eq!(file.cursor_offset, 4);
}

#[test]
fn it_doesnt_add_fake_newlines() {
    let mut file = FileView::new("testfiles/no_line_ending.txt").unwrap();
    assert_eq!(file.lines.len(), 1);
}

#[test]
fn it_goes_to_the_end_of_the_file() {
    let mut file = make_file();
    file.insert_str("foo\nbar\nfoobar");
    file.set_cursor(0);
    file.move_cursor_to_end();
    assert_eq!(file.cursor_offset, 8);
}

#[test]
fn it_goes_to_the_start_of_the_file() {
    let mut file = make_file();
    file.insert_str("foo\nbar\nfoobar");
    file.set_cursor(8);
    file.move_cursor_to_start();
    assert_eq!(file.cursor_offset, 0);
}
