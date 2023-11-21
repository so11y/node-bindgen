use crate::Component;

pub trait ComponentGroup: Component {
    fn add_child(&mut self, child: Box<dyn Component>);
    // fn set_translate(&mut self, x: f64, y: f64);
    // fn get_translate(&self) -> (f64, f64);
}
