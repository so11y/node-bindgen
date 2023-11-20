use derives::{GroupDropMacro, GroupMacro};
use web_sys::CanvasRenderingContext2d;

use crate::{canvas_component_parent_macro, CanvasComponent, CanvasComponentGroup};

#[derive(GroupDropMacro, GroupMacro)]
pub struct Container {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    translate: (f64, f64),
    parent: Option<*const dyn CanvasComponentGroup>,
    children: Vec<Box<dyn CanvasComponent>>,
}
impl Container {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            translate: (0.0, 0.0),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl CanvasComponent for Container {
    fn draw(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let current_ref = self as *const dyn CanvasComponentGroup;
        let (x, y) = self.get_parent_location();
        let (tx, ty) = self.get_translate();
        ctx.set_stroke_style(&"red".into());
        ctx.rect(self.x + x + tx, self.y + y + ty, self.width, self.height);
        ctx.stroke();
        self.children.iter_mut().for_each(|child| {
            child.set_parent(Some(current_ref));
            child.draw(ctx);
        });
    }
    canvas_component_parent_macro!();
}
