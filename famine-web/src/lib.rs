use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use famine::Window;


#[wasm_bindgen]
pub struct WebWindow {
    gl: web_sys::WebGl2RenderingContext,
}

impl Window for WebWindow {
    fn new(title: &str, _width: usize, _height: usize) -> WebWindow {
        let window: web_sys::Window = web_sys::window().expect("Failed to get global window!");
        let document: web_sys::Document = window.document().expect("Failed to get the document!");
        let canvas: HtmlCanvasElement = document.query_selector("canvas").expect("Failed to find a canvas!")
            .unwrap().dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast element as a canvas!");
        let gl: WebGl2RenderingContext = canvas.get_context("webgl2")
            .expect("Failed to get rendering context!").unwrap()
            .dyn_into::<WebGl2RenderingContext>().expect("Failed to cast rendering context!");

        document.set_title(title);

        WebWindow { gl }
    }

    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r * 255.0, g * 255.0, b * 255.0, a * 255.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}

#[wasm_bindgen]
pub fn web_update(window: &WebWindow) {
    famine_application::update(window)
}

#[wasm_bindgen]
pub fn web_startup() -> WebWindow {
    famine_application::startup::<WebWindow>()
}