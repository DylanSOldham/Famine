extern crate famine;

use famine::{Application, Mesh, WindowType};

pub struct App<Window: WindowType> {
    pub window: Window,
    pub basic_shader: Window::Shader,
    pub mesh: Mesh
}    

impl<Window: WindowType> Application<Window> for App<Window> {

    fn new() -> Self {
        let vert_src = 
            r##"#version 300 es
        
            in vec4 position;
        
            void main() {
            
                gl_Position = position;
            }
            "##;
        
        let frag_src =
            r##"#version 300 es
        
            precision highp float;
            out vec4 outColor;
            
            void main() {
                outColor = vec4(1, 1, 1, 1);
            }
            "##;

        let window = Window::new("ZA APP", 640, 480);
        let basic_shader = window.new_shader(vert_src, frag_src);
        let mesh = Mesh::new(vec![
            -0.5,  0.5, 0.0,
             0.5,  0.5, 0.0,
             0.0, -0.5, 0.0,
        ]);

        App {
            window,
            basic_shader,
            mesh,
        }
    }

    fn update(&self) {
        self.window.clear(0.02, 0.05, 0.2, 1.0);
        self.window.use_shader(&self.basic_shader);
        self.window.draw_mesh(&self.mesh);
    }

    fn get_window(&self) -> &Window {
        &self.window
    }
}