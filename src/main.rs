mod render;

use render::*;

fn main() {
    let mut x = render::terminal::TermRenderer::new();
    print!("\x1B[20;0f");
    x
        .clear()
        .draw(Point { row : 5, col : 20, }, "welcome to byt")
        .done();
        //.done();
    loop {}
}
