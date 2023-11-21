use web_sys::CanvasRenderingContext2d;

use crate::{ComponentGroup, Constraint};

pub trait Component {
    // fn layout(&self, constraint: Constraint)->;
    fn draw(&mut self, ctx: &mut CanvasRenderingContext2d);
    // fn set_parent(&mut self, parent: Option<*const dyn ComponentGroup>);
    // fn get_parent(&self) -> Option<&dyn ComponentGroup>;
    fn get_children(&self) -> Option<&Vec<Box<dyn Component>>> {
        return None;
    }
    // fn set_constraint(&mut self, constraint: Constraint);
    // fn get_constraint(&self) -> &Constraint;
}
