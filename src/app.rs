extern crate famine;

use famine::{Application, WindowType};

pub struct App<Window: WindowType> {
    pub window: Window,
    pub basic_shader: Window::Shader,
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

        App {
            window,
            basic_shader,
        }
    }

    fn update(&self) {
        self.window.clear(1.0, 1.0, 0.0, 1.0);
        self.window.use_shader(&self.basic_shader);
    }

    fn get_window(&self) -> &Window {
        &self.window
    }
}