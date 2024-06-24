pub struct Mesh {
    pub vertices: Vec<f32>
}

impl Mesh {
    fn new(vertices: Vec<f32>) -> Mesh {
        Mesh { vertices }
    }
}

pub trait WindowType {
    type Shader;

    fn new(title: &str, width: usize, height: usize) -> Self;
    fn clear(&self, r: f32, g: f32, b: f32, a: f32);
    fn draw_mesh(&self, mesh: &Mesh);
    fn new_shader(&self, vert_src: &str, frag_str: &str) -> Self::Shader;
    fn use_shader(&self, shader: &Self::Shader);
}

pub trait Application<Window: WindowType> {
    fn new() -> Self;
    fn update(&self);
    fn get_window(&self) -> &Window;
}