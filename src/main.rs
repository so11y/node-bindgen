
//trunk serve --open
use canvas_rust::{
    canvas_context, CanvasComponent, CanvasComponentGroup, CanvasOptions, Line, Lines,
};

fn main() {
    let mut ctx = canvas_context("canvas", CanvasOptions::default());

    let mut lines = Lines::new(0.0, 0.0);

    lines.add_child(Box::new(Line::new(20.0, 0.0)));
    lines.add_child(Box::new(Line::new(20.0, 20.0)));
    lines.add_child(Box::new(Line::new(0.0, 20.0)));
    lines.add_child(Box::new(Line::new(0.0, 0.0)));
    lines.set_translate(20.0, 20.0);
    lines.draw(&mut ctx);
}

