use derives::{GroupDropMacro, GroupMacro};
use web_sys::CanvasRenderingContext2d;

use crate::{component_macro, Component, ComponentGroup, Constraint};

#[derive(GroupDropMacro, GroupMacro)]
pub struct Container {
    width: f64,
    height: f64,
    constraint: Option<Constraint>,
    // translate: (f64, f64),
    // parent: Option<*const dyn ComponentGroup>,
    children: Vec<Box<dyn Component>>,
}
impl Container {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            constraint: None,
            // parent: None,
            children: Vec::new(),
        }
    }
}

impl Component for Container {
    // fn layout(&mut self, constraint: Constraint) {
    //     if constraint.maybe_constraint(self.width, self.height) {

    //     }
    // }
    fn draw(&mut self, ctx: &mut CanvasRenderingContext2d) {
        // let current_ref = self as *const dyn CanvasComponentGroup;
        // let (tx, ty) = self.get_translate();
        // self.children.iter_mut().for_each(|child| {
        //     child.set_parent(Some(current_ref));
        //     child.draw(ctx);
        // });
        // ctx.rect(x + tx, y + ty, self.width, self.height);
    }
    component_macro!();
}
