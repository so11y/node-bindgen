#[macro_export]
macro_rules! canvas_component_parent_macro {
    () => {
        fn set_parent(&mut self, parent: Option<*const dyn CanvasComponentGroup>) {
            self.parent = parent;
        }

        fn get_parent(&self) -> Option<&dyn CanvasComponentGroup> {
            if self.parent.is_none() {
                return None;
            }
            unsafe {
                let parent = self.parent;
                return Some(&*parent.unwrap());
            }
        }
    };
}
