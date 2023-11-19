use web_sys::{CanvasRenderingContext2d};

use crate::{impl_canvas_component_parent, CanvasComponent, CanvasComponentGroup};



pub struct Lines {
    x: f64,
    y: f64,
    move_x: f64,
    move_y: f64,
    parent: Option<*const dyn CanvasComponentGroup>,
    children: Vec<Box<dyn CanvasComponent>>,
}

impl Drop for Lines {
    fn drop(&mut self) {
        self.children.iter_mut().for_each(|child| {
            child.set_parent(None);
        });
    }
}

impl Lines {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            move_x: 0.0,
            move_y: 0.0,
            parent: None,
            children: Vec::new(),
        }
    }
}

impl CanvasComponent for Lines {
    fn draw(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let (x, y) = self.get_parent_location();

        ctx.begin_path();
        ctx.move_to(self.x + x + self.move_x, self.y + y + self.move_y);

        let current_ref = self as *const dyn CanvasComponentGroup;

        self.children.iter_mut().for_each(|child| {
            child.set_parent(Some(current_ref));
            child.draw(ctx);
        });
        ctx.stroke();
        ctx.close_path();
    }
    fn get_local_move_x(&self) -> f64 {
        return self.move_x;
    }
    fn get_local_move_y(&self) -> f64 {
        return self.move_y;
    }

    impl_canvas_component_parent!();
}

impl CanvasComponentGroup for Lines {
    fn add_child(&mut self, child: Box<dyn CanvasComponent>) {
        self.children.push(child);
    }
    fn move_container(&mut self, x: f64, y: f64, ctx: &mut CanvasRenderingContext2d) {
        self.move_x = x;
        self.move_y = y;
        self.draw(ctx)
    }
}
