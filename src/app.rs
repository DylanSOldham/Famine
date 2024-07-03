extern crate famine;

use famine::{linalg::Mat4, Application, Mesh, WindowType};

pub struct App<Window: WindowType> {
    pub window: Window,
    pub basic_shader: Window::Shader,
    pub vmp_matrix: Mat4,
    pub texture: Window::Texture,
    pub mesh: Mesh,
    pub rotation: f32,
}    

impl<Window: WindowType> Application<Window> for App<Window> {

    fn new() -> Self {
        let vert_src = 
            r##"#version 300 es
        
            in vec4 v_position;
            in vec2 v_uv;

            out vec2 f_uv;
            out vec3 f_pos;

            uniform mat4 u_ViewModelProjection;
        
            void main() {
                gl_Position = u_ViewModelProjection * v_position;
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

        let vmp_matrix: Mat4 = Mat4::identity();
        
        let texture = window.new_data_texture(3, 3, vec![
            255, 0, 0,   255,
            255, 255, 0, 255,
            255, 0, 0,   255,
            0, 255, 0,   255,
            0, 0, 255,   255,
            0, 255, 0,   255,
            255, 0, 0,   255,
            255, 255, 0, 255,
            255, 0, 0,   255
        ]);
        
        let mesh = Mesh::sphere(0.5, 30, 30).unwrap();

        App {
            window,
            basic_shader,
            vmp_matrix,
            texture,
            mesh,
            rotation: 0.0,
        }
    }

    fn update(&mut self) {
        let rotation_matrix_0 = Mat4::rotate_y(self.rotation);
        let rotation_matrix_1 = Mat4::rotate_x(self.rotation);
        let scale_matrix = Mat4::scale(self.window.height() as f32 / self.window.width() as f32, 1.0, 1.0);
        self.vmp_matrix = rotation_matrix_1.mul(&rotation_matrix_0).mul(&scale_matrix);

        Window::log(format!("{}", self.window.height()).as_str());
        self.rotation += 0.02;

        self.window.clear(0.02, 0.05, 0.2, 1.0);
        self.window.use_texture(&mut self.texture);
        self.window.use_shader(&self.basic_shader);
        self.window.set_uniform_mat4(&self.basic_shader, "u_ViewModelProjection", &self.vmp_matrix);
        self.window.draw_mesh(&self.mesh);
    }

    fn get_window(&self) -> &Window {
        &self.window
    }
}