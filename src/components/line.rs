use crate::{impl_canvas_component_parent, CanvasComponent, CanvasComponentGroup};
use web_sys::CanvasRenderingContext2d;

#[derive(Debug)]
pub struct Line {
    x: f64,
    y: f64,
    parent: Option<*const dyn CanvasComponentGroup>,
}

impl Line {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, parent: None }
    }
}

impl CanvasComponent for Line {
    fn draw(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let (x, y) = self.get_parent_location();
        ctx.line_to(self.x + x, self.y + y);
    }
    impl_canvas_component_parent!();
}
