mod components;
mod macro_rules;
mod traits;

pub use components::*;
pub use macro_rules::*;
pub use traits::*;

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
