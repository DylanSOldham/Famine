extern crate famine;

use famine::{linalg::Mat4, Application, Color, ContextType, Mesh};

pub struct App<Context: ContextType> {
    pub ctx: Context,
    pub basic_shader: Context::Shader,
    pub vmp_matrix: Mat4,
    pub texture: Context::Texture,
    pub mesh: Mesh,
    pub rotation: f32,
}    

impl<Context: ContextType> Application<Context> for App<Context> {

    async fn new() -> Self {
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

        let mut ctx = Context::new("ZA APP", 640, 480);
        let font_texture = ctx.new_image_texture("font.png").await;
        ctx.set_font_texture(font_texture);

        let basic_shader = ctx.new_shader(vert_src, frag_src);
        let vmp_matrix: Mat4 = Mat4::identity();
        
        let texture = ctx.new_data_texture(3, 3, vec![
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
            ctx,
            basic_shader,
            vmp_matrix,
            texture,
            mesh,
            rotation: 0.0,
        }
    }

    async fn update(&mut self) {
        let rotation_matrix_0 = Mat4::rotate_y(self.rotation);
        let rotation_matrix_1 = Mat4::rotate_x(self.rotation);
        let scale_matrix = Mat4::scale(self.ctx.display_height() as f32 / self.ctx.display_width() as f32, 1.0, 1.0);
        self.vmp_matrix = rotation_matrix_1.mul(&rotation_matrix_0).mul(&scale_matrix);

        self.rotation += 0.02;

        self.ctx.clear(0.02, 0.05, 0.2, 1.0);
        self.ctx.use_texture(&mut self.texture);
        self.ctx.use_shader(&self.basic_shader);
        self.ctx.set_uniform_mat4(&self.basic_shader, "u_ViewModelProjection", &self.vmp_matrix);
        self.ctx.draw_mesh(&self.mesh);

        let rot_str = format!("{}", self.rotation);
        let text_w = rot_str.len() as f32 * 0.05;
        self.ctx.draw_text(rot_str.as_str(), -1.0, -1.0, text_w, 0.2, Color { r: 0, g: 255, b: 0, a: 255 });
    }

    fn get_window(&self) -> &Context {
        &self.ctx
    }
}