use derives::{CanvasGroupDropMacro, CanvasGroupMacro};
use web_sys::CanvasRenderingContext2d;

use crate::{canvas_component_parent_macro, CanvasComponent, CanvasComponentGroup};

#[derive(CanvasGroupDropMacro, CanvasGroupMacro)]
pub struct Lines {
    x: f64,
    y: f64,
    translate: (f64, f64),
    parent: Option<*const dyn CanvasComponentGroup>,
    children: Vec<Box<dyn CanvasComponent>>,
}

impl Lines {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            translate: (0.0, 0.0),
            children: Vec::new(),
            parent: None,
        }
    }
}

impl CanvasComponent for Lines {
    fn draw(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let (x, y) = self.get_parent_location();
        let (tx, ty) = self.get_translate();

        ctx.begin_path();

        ctx.move_to(self.x + x + tx, self.y + y + ty);

        let current_ref = self as *const dyn CanvasComponentGroup;

        self.children.iter_mut().for_each(|child| {
            child.set_parent(Some(current_ref));
            child.draw(ctx);
        });
        ctx.stroke();
        ctx.close_path();
    }

    canvas_component_parent_macro!();
}
