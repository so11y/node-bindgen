
//trunk serve --open
use canvas_rust::{
    canvas_context, CanvasComponent, CanvasComponentGroup, CanvasOptions, Line, Lines,
};

// use web_sys::console;

fn main() {
    let mut ctx = canvas_context("canvas", CanvasOptions::default());

    let mut liens = Lines::new(0.0, 0.0);

    liens.add_child(Box::new(Line::new(20.0, 0.0)));
    liens.add_child(Box::new(Line::new(20.0, 20.0)));
    liens.add_child(Box::new(Line::new(0.0, 20.0)));
    liens.add_child(Box::new(Line::new(0.0, 0.0)));
    liens.draw(&mut ctx);

    // liens.move_container(50.0, 50.0, &mut ctx);
}


