
pub trait Window {
    fn new(title: &str, width: usize, height: usize) -> Self;
    fn clear(&self, r: f32, g: f32, b: f32, a: f32);
}