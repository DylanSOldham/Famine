pub struct Mesh {
    pub vertices: Vec<f32>
}

impl Mesh {
    pub fn new(vertices: Vec<f32>) -> Mesh {
        Mesh { vertices }
    }
}

pub trait WindowType {
    type Shader;
    type Texture;

    fn new(title: &str, width: usize, height: usize) -> Self;
    fn clear(&self, r: f32, g: f32, b: f32, a: f32);
    fn new_shader(&self, vert_src: &str, frag_str: &str) -> Self::Shader;
    fn use_shader(&self, shader: &Self::Shader);
    fn new_texture(&self, name: &str) -> Self::Texture;
    fn use_texture(&self, texture: &mut Self::Texture);
    fn draw_mesh(&self, mesh: &Mesh);
}

pub trait Application<Window: WindowType> {
    fn new() -> Self;
    fn update(&mut self);
    fn get_window(&self) -> &Window;
}