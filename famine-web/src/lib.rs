use std::{alloc::{alloc, dealloc, Layout}, ptr};

use famine_application::App;
use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};
use famine::{Application, Mesh, WindowType};

#[wasm_bindgen]
pub struct WebWindow {
    gl: web_sys::WebGl2RenderingContext,
}

pub struct WebShader {
    program: WebGlProgram,
}

fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> WebGlShader {
    let shader = context
        .create_shader(shader_type)
        .expect("Failed to create shader.");
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if !context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        console::log_1(&"Failed to compile shader.".into())
    }

    shader
}

pub fn link_program(context: &WebGl2RenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> WebGlProgram {
    let program = match context.create_program() {
        Some(p) => p,
        None => {
            console::log_1(&"Unable to create shader program".into());
            panic!("");
        }
    };

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if !context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let error_msg = context.get_program_info_log(&program).unwrap_or_else(|| "Unkown error".into()); 
        console::log_1(&format!("Error creating shader program: {}", error_msg).into());
    }

    program
}

impl WindowType for WebWindow {
    type Shader = WebShader;

    fn new(title: &str, _width: usize, _height: usize) -> WebWindow {
        let window: web_sys::Window = web_sys::window().expect("Failed to get global window!");
        let document: web_sys::Document = window.document().expect("Failed to get the document!");

        let canvas: HtmlCanvasElement = document.query_selector("canvas").expect("Failed to find a canvas!")
            .unwrap().dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast element as a canvas!");

        console::log_1(&format!("DA{}", canvas.width()).into());

        canvas.set_width((canvas.client_width() as f64 * window.device_pixel_ratio()) as u32);
        canvas.set_height((canvas.client_height() as f64 * window.device_pixel_ratio()) as u32);
        
        console::log_1(&format!("{}", canvas.width()).into());

        let gl: WebGl2RenderingContext = canvas.get_context("webgl2")
            .expect("Failed to get rendering context!").unwrap()
            .dyn_into::<WebGl2RenderingContext>().expect("Failed to cast rendering context!");

        document.set_title(title);

        let buffer = match gl.create_buffer() {
            Some(b) => b,
            None => {
                console::log_1(&"Failed to create buffer".into());
                panic!()
            },
        };
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        WebWindow { gl }
    }

    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    fn draw_mesh(&self, mesh: &Mesh) {
        let data = unsafe { js_sys::Float32Array::view(&mesh.vertices.as_slice()) };
        self.gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data, WebGl2RenderingContext::STATIC_DRAW);
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (mesh.vertices.len() / 3) as i32);
    }

    fn new_shader(&self, vert_src: &str, frag_src: &str) -> Self::Shader {
        let vert_shader = compile_shader(&self.gl, WebGl2RenderingContext::VERTEX_SHADER, vert_src);
        let frag_shader = compile_shader(&self.gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src);
        let program = link_program(&self.gl, &vert_shader, &frag_shader);

        WebShader {
            program
        }
    }

    fn use_shader(&self, shader: &Self::Shader) {
        self.gl.use_program(Some(&shader.program));

        let vao = match self.gl.create_vertex_array() {
            Some(va) => va,
            None => {
                console::log_1(&"Failed to create vao".into());
                panic!()
            }
        };
        let position_attribute_location: u32 = self.gl.get_attrib_location(&shader.program, "position") as u32;
        self.gl.bind_vertex_array(Some(&vao));
        self.gl.vertex_attrib_pointer_with_i32(position_attribute_location, 3,
             WebGl2RenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(position_attribute_location);
    }
}

#[wasm_bindgen]
pub fn web_startup() -> *mut App<WebWindow> { 
    let layout = Layout::new::<App<WebWindow>>();
    let app_ptr = unsafe { alloc(layout) as *mut App<WebWindow> };

    if app_ptr.is_null() {
        console::log_1(&"Failed to allocate memory for the applicaiton.".into());
        return ptr::null_mut();
    }
    
    unsafe { ptr::write(app_ptr, App::<WebWindow>::new()) }
    
    console::log_1(&"Successful application startup.".into());

    app_ptr
}

#[wasm_bindgen]
pub fn web_update(application: *mut App<WebWindow>) {
    unsafe { &*application }.update();
}

#[wasm_bindgen]
pub fn web_shutdown(application: *mut App<WebWindow>) {
    unsafe { dealloc(application as *mut u8, Layout::new::<App<WebWindow>>()) }
}