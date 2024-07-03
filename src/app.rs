extern crate famine;

use famine::{Application, Mesh, WindowType};

pub struct App<Window: WindowType> {
    pub window: Window,
    pub basic_shader: Window::Shader,
    pub texture: Window::Texture,
    pub mesh: Mesh
}    

impl<Window: WindowType> Application<Window> for App<Window> {

    fn new() -> Self {
        let vert_src = 
            r##"#version 300 es
        
            in vec4 v_position;
            in vec2 v_uv;

            out vec2 f_uv;
            out vec3 f_pos;
        
            void main() {
                gl_Position = v_position;
                f_uv = v_uv;
                f_pos = v_position.xyz;
            }
            "##;
        
        let frag_src =
            r##"#version 300 es
            precision highp float;

            uniform sampler2D uTexture;

            in vec2 f_uv;
            in vec3 f_pos;
        
            out vec4 outColor;
            
            void main() {
                outColor = texture(uTexture, f_uv);
            }
            "##;

        let window = Window::new("ZA APP", 640, 480);
        let basic_shader = window.new_shader(vert_src, frag_src);
        
        let texture = window.new_data_texture(3, 3, vec![
            255, 0, 0, 255,
            0, 0, 255, 255,
            255, 0, 0, 255,
            0, 255, 0, 255,
            255, 255, 0, 255,
            0, 255, 0, 255,
            255, 0, 0, 255,
            0, 0, 255, 255,
            255, 0, 0, 255
        ]);
        
        let mesh = Mesh::sphere(0.5, 15, 15).unwrap();

        App {
            window,
            basic_shader,
            texture,
            mesh,
        }
    }

    fn update(&mut self) {
        self.window.clear(0.02, 0.05, 0.2, 1.0);
        self.window.use_texture(&mut self.texture);
        self.window.use_shader(&self.basic_shader);
        self.window.draw_mesh(&self.mesh);
    }

    fn get_window(&self) -> &Window {
        &self.window
    }
}