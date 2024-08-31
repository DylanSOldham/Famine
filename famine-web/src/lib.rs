use std::{alloc::{alloc, dealloc, Layout}, ptr, sync::Arc};


use famine_application::App;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys, HtmlCanvasElement, HtmlImageElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlTexture};
use famine::{shaders::font_shader::{FONT_FRAG_SHADER, FONT_VERT_SHADER}, Application, ContextType, Mesh};
use famine::linalg::Mat4;

#[wasm_bindgen]
pub struct WebContext {
    gl: web_sys::WebGl2RenderingContext,
    font_texture: Option<WebTexture>,
    font_shader: Option<WebShader>,
}

pub struct WebShader {
    program: WebGlProgram,
}

pub enum TextureData {
    ImageData(HtmlImageElement),
    RawData(i32, i32, Vec<u8>),
}

pub struct WebTexture {
    pub data: TextureData,
    pub gl_texture: WebGlTexture,
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

impl ContextType for WebContext {
    type Shader = WebShader;
    type Texture = WebTexture;

    fn new(title: &str, _width: usize, _height: usize) -> WebContext {
        let window: web_sys::Window = web_sys::window().expect("Failed to get global window!");
        let document: web_sys::Document = window.document().expect("Failed to get the document!");

        let canvas: HtmlCanvasElement = document.query_selector("canvas").expect("Failed to find a canvas!")
            .unwrap().dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to cast element as a canvas!");
        canvas.set_width((canvas.client_width() as f64 * window.device_pixel_ratio()) as u32);
        canvas.set_height((canvas.client_height() as f64 * window.device_pixel_ratio()) as u32);

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

        gl.enable(WebGl2RenderingContext::CULL_FACE);
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

        WebContext { gl, font_texture: None, font_shader: None }
    }

    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    fn draw_mesh(&self, mesh: &Mesh) {
        let data = unsafe { js_sys::Float32Array::view(&mesh.vertices.as_slice()) };
        self.gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data, WebGl2RenderingContext::STATIC_DRAW);
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (mesh.vertices.len() / 5) as i32);
    }

    fn new_shader(&self, vert_src: &str, frag_src: &str) -> Self::Shader {
        let vert_shader = compile_shader(&self.gl, WebGl2RenderingContext::VERTEX_SHADER, vert_src);
        let frag_shader = compile_shader(&self.gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src);
        let program = link_program(&self.gl, &vert_shader, &frag_shader);

        WebShader {
            program,
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
        let position_attribute_location: u32 = self.gl.get_attrib_location(&shader.program, "v_position") as u32;
        let uv_attribute_location: u32 = self.gl.get_attrib_location(&shader.program, "v_uv") as u32;
        self.gl.bind_vertex_array(Some(&vao));
        self.gl.vertex_attrib_pointer_with_i32(position_attribute_location, 3,
             WebGl2RenderingContext::FLOAT, false, 20, 0);
        self.gl.vertex_attrib_pointer_with_i32(uv_attribute_location, 2,
            WebGl2RenderingContext::FLOAT, false, 20, 12);
        self.gl.enable_vertex_attrib_array(position_attribute_location);
        self.gl.enable_vertex_attrib_array(uv_attribute_location);
    }
    
    fn set_uniform_vec4(&self, shader: &Self::Shader, uniform_name: &str, value: &famine::linalg::Vec4) {
        let location = self.gl.get_uniform_location(&shader.program, uniform_name);
        self.gl.uniform4fv_with_f32_array(location.as_ref(), &value.data);
    }
    
    fn set_uniform_mat4(&self, shader: &Self::Shader, uniform_name: &str, value: &Mat4) {
        let location = self.gl.get_uniform_location(&shader.program, uniform_name);
        self.gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &value.data);
    }

    async fn new_image_texture(&self, name: &str) -> Self::Texture {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(format!("pkg/assets/{}", name).as_str());
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let callback = Closure::<dyn Fn()>::new(move || {
                resolve.call0(&JsValue::NULL).unwrap();
            });
    
            if let Err(err)= image.add_event_listener_with_callback("load", callback.as_ref().unchecked_ref()) {
                Self::log(&format!("Error adding image loading callback: {}", err.as_string().get_or_insert("Unknown Error".into())));
            }
            callback.forget();
        });
        
        if let Err(err) = JsFuture::from(promise).await {
            Self::log(&format!("Error with image load callback: {}", err.as_string().get_or_insert("Unknown Error".into())));
        }

        let gl_texture: WebGlTexture = match self.gl.create_texture() {
            None => {
                Self::log("Failed to create texture");
                panic!()
            }
            Some(t) => t
        };

        self.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&gl_texture));
        self.gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D, 0, 
            WebGl2RenderingContext::RGBA as i32, WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE, &image
        ).unwrap();
        self.gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        WebTexture {
            data: TextureData::ImageData(image),
            gl_texture,
        }
    }

    fn new_data_texture(&self, width: i32, height: i32, data: Vec<u8>) -> Self::Texture {
        let gl_texture: WebGlTexture = match self.gl.create_texture() {
            None => {
                console::log_1(&"Failed to create texture".into());
                panic!()
            }
            Some(t) => t
        };
        self.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&gl_texture));

        self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D, 0, WebGl2RenderingContext::RGBA as i32, 
            width, height, 0, WebGl2RenderingContext::RGBA, WebGl2RenderingContext::UNSIGNED_BYTE, 
            Some(&data)
        ).unwrap();
        self.gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        WebTexture {
            data: TextureData::RawData(width, height, data),
            gl_texture,
        }
    }

    fn use_texture(&self, texture: &Self::Texture) {
        self.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture.gl_texture));
    }

    fn display_width(&self) -> i32 {
        self.gl.drawing_buffer_width()
    }

    fn display_height(&self) -> i32 {
        self.gl.drawing_buffer_height()
    }

    fn log(text: &str) {
        console::log_1(&text.into());
    }
    
    fn draw_text(&self, text: &str, x: f32, y: f32, width: f32, height: f32, color: famine::Color) {
        if self.font_texture.is_none() {
            Self::log("Famine Warning: Context is missing font texture.");
            return
        }

        self.use_shader(self.font_shader.as_ref().unwrap());
        self.use_texture(self.font_texture.as_ref().unwrap());
        self.set_uniform_vec4(self.font_shader.as_ref().unwrap(), "u_color", &color.as_vec4());

        let char_w: f32 = width / text.len() as f32;
        let u_per_column: f32 = 1.0 / 16.0;
        let v_per_row: f32 = 1.0 / 8.0;
        for (i, c) in text.chars().enumerate() {

            let row: f32 = ((c as u32 - '!' as u32) as f32 / 16.0).floor();
            let column: f32 = ((c as u32 - '!' as u32) % 16) as f32;

            let char_x: f32 = x + i as f32 * char_w;
            let char_u1: f32 = u_per_column * column;
            let char_u2: f32 = char_u1 + u_per_column;
            let char_v1: f32 = v_per_row * (row + 1.0);
            let char_v2: f32 = v_per_row * row;
            let mesh = Mesh { vertices: vec![
                char_x,          y + height, 0.0, char_u1, char_v2,
                char_x,          y,          0.0, char_u1, char_v1,
                char_x + char_w, y + height, 0.0, char_u2, char_v2,
                char_x,          y,          0.0, char_u1, char_v1,
                char_x + char_w, y,          0.0, char_u2, char_v1,
                char_x + char_w, y + height, 0.0, char_u2, char_v2,
            ]};
            self.draw_mesh(&mesh);
        }

    }
    
    fn set_font_texture(&mut self, texture: Self::Texture) {
        if self.font_shader.is_none() {
            let font_shader = self.new_shader(FONT_VERT_SHADER, FONT_FRAG_SHADER);
            self.font_shader = Some(font_shader);
        }

        self.font_texture = Some(texture);
    }
}

#[wasm_bindgen]
pub async fn web_startup() -> *mut App<WebContext> {
    let layout = Layout::new::<App<WebContext>>();
    let app_ptr = unsafe { alloc(layout) as *mut App<WebContext> };

    if app_ptr.is_null() {
        console::log_1(&"Failed to allocate memory for the applicaiton.".into());
        return ptr::null_mut();
    }
    
    unsafe { ptr::write(app_ptr, App::<WebContext>::new().await) }
    
    console::log_1(&"Successful application startup.".into());

    app_ptr
}

#[wasm_bindgen]
pub async fn web_update(application: *mut App<WebContext>) {
    unsafe { &mut *application }.update().await;
}

#[wasm_bindgen]
pub fn web_shutdown(application: *mut App<WebContext>) {
    unsafe { dealloc(application as *mut u8, Layout::new::<App<WebContext>>()) }
}