mod components;

pub use components::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d};

pub struct CanvasOptions {
    width: u32,
    height: u32,
}

impl Default for CanvasOptions {
    fn default() -> Self {
        Self {
            width: 300,
            height: 300,
        }
    }
}
impl CanvasOptions {
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub trait CanvasComponent {
    fn draw(&mut self, ctx: &mut CanvasRenderingContext2d);
    fn set_parent(&mut self, parent: Option<*const dyn CanvasComponentGroup>);
    fn get_parent(&self) -> Option<&dyn CanvasComponentGroup>;
    fn get_children(&self) -> Option<&Vec<Box<dyn CanvasComponent>>> {
        return None;
    }
    fn get_parent_location(&self) -> (f64, f64) {
        let parent = self.get_parent();
        if parent.is_none() {
            return (0 as f64, 0 as f64);
        } else {
            return parent.unwrap().get_translate();
        }
    }
}

pub trait CanvasComponentGroup: CanvasComponent {
    fn add_child(&mut self, child: Box<dyn CanvasComponent>);
    fn set_translate(&mut self, x: f64, y: f64);
    fn get_translate(&self) -> (f64, f64);
}

pub fn canvas_context(dom_name: &str, canvas_options: CanvasOptions) -> CanvasRenderingContext2d {
    //  获取document
    let document = window().unwrap().document().unwrap();
    //获取canvas
    let canvas = document.get_element_by_id(dom_name).unwrap();

    //转换为HtmlCanvasElement
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(canvas_options.width);
    canvas.set_height(canvas_options.height);
    //获取canvas的context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    return context;
}
