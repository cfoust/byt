mod render;

use render::*;

fn main() {
    let x = render::terminal::TermRenderer {
        size : Point {
            row : 20,
            col : 20
        }
    };

    x
        .clear()
        .draw(Point {
            row : 20,
            col : 20
        }, "Hello")
        .done();
}
