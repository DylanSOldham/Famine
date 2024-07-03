use std::f32::consts::PI;
use linalg::Mat4;

pub mod linalg;

pub struct Mesh {
    pub vertices: Vec<f32>
}

impl Mesh {
    pub fn new(vertices: Vec<f32>) -> Self {
        Mesh { vertices }
    }

    pub fn sphere(radius: f32, rings: u32, slices: u32) -> Option<Self> {
        if rings < 3 || slices < 3 || radius <= 0.0 {
            return None;
        }

        let mut vertices: Vec<f32> = vec![];
        
        let angle_per_ring: f32 = PI / rings as f32;
        let angle_per_slice: f32 = 2.0 * PI / slices as f32;
        for i in 1..rings+1 {
            let phi: f32 = PI/2.0 + angle_per_ring * i as f32;

            let top_y = radius * phi.sin();
            let bottom_y = radius * (phi - angle_per_ring).sin();

            for j in 1..slices+1 {
                let theta: f32 = 2.0 * PI + angle_per_slice * j as f32;

                let top_x1: f32 = radius * (theta - angle_per_slice).cos() * phi.cos();
                let top_z1: f32 = radius * (theta - angle_per_slice).sin() * phi.cos();
                let bottom_x1: f32 = radius * (theta - angle_per_slice).cos() * (phi - angle_per_ring).cos();
                let bottom_z1: f32 = radius * (theta - angle_per_slice).sin() * (phi - angle_per_ring).cos();

                let top_x2: f32 = radius * theta.cos() * phi.cos();
                let top_z2: f32 = radius * theta.sin() * phi.cos();
                let bottom_x2: f32 = radius * theta.cos() * (phi - angle_per_ring).cos();
                let bottom_z2: f32 = radius * theta.sin() * (phi - angle_per_ring).cos();

                let left_u = (theta - angle_per_slice) / (2.0 * PI);
                let right_u = theta / (2.0 * PI);

                let top_v = top_y;
                let bottom_v = bottom_y;

                vertices.append(&mut vec![
                    top_x1, top_y, top_z1, left_u, top_v,
                    top_x2, top_y, top_z2, right_u, top_v,
                    bottom_x1, bottom_y, bottom_z1, left_u, bottom_v,

                    bottom_x2, bottom_y, bottom_z2, right_u, bottom_v,
                    bottom_x1, bottom_y, bottom_z1, left_u, bottom_v,
                    top_x2, top_y, top_z2, right_u, top_v,
                ]);
            }
        }


        Some(Mesh { vertices })
    }
}

pub trait WindowType {
    type Shader;
    type Texture;

    fn new(title: &str, width: usize, height: usize) -> Self;
    fn clear(&self, r: f32, g: f32, b: f32, a: f32);
    fn new_shader(&self, vert_src: &str, frag_str: &str) -> Self::Shader;
    fn use_shader(&self, shader: &Self::Shader);
    fn set_uniform_mat4(&self, shader: &Self::Shader, uniform_name: &str, value: &Mat4);
    fn new_image_texture(&self, name: &str) -> Self::Texture;
    fn new_data_texture(&self, width: i32, height: i32, data: Vec<u8>) -> Self::Texture;
    fn use_texture(&self, texture: &mut Self::Texture);
    fn draw_mesh(&self, mesh: &Mesh);

    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn log(text: &str);
}

pub trait Application<Window: WindowType> {
    fn new() -> Self;
    fn update(&mut self);
    fn get_window(&self) -> &Window;
}