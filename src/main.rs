//trunk serve --open
use canvas_rust::{
    canvas_context, CanvasComponent, CanvasComponentGroup, CanvasOptions, Container, Line, Lines,
};

fn main() {
    let mut ctx = canvas_context("canvas", CanvasOptions::default());

    let mut lines = Lines::new(0.0, 0.0);

    let mut container = Container::new(50.0, 50.0, 50.0, 50.0);

    lines.add_child(Box::new(Line::new(20.0, 0.0)));
    lines.add_child(Box::new(Line::new(20.0, 20.0)));
    lines.add_child(Box::new(Line::new(0.0, 20.0)));
    lines.add_child(Box::new(Line::new(0.0, 0.0)));
    lines.set_translate(210.0, 20.0);

    container.add_child(Box::new(lines));

    container.draw(&mut ctx);
}
