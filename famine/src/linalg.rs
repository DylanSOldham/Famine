pub struct Vec4 {
    pub data: [f32; 4],
}

impl Vec4 {
    pub fn new(data: [f32; 4]) -> Self {
        Vec4 { data }
    }
}

pub struct Mat4 {
    pub data: [f32; 16],
}

impl Mat4 {
    pub fn new(data: [f32; 16]) -> Self {
        Mat4 { data }
    }

    pub fn zero() -> Self {
        Mat4 { data: [
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
        ]}
    }

    pub fn identity() -> Self {
        Mat4 { data: [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ]}
    }

    pub fn scale(xfactor: f32, yfactor: f32, zfactor: f32) -> Self {
        Mat4 { data: [
            xfactor, 0., 0., 0.,
            0., yfactor, 0., 0.,
            0., 0., zfactor, 0.,
            0., 0., 0., 1.0,
        ]}
    }

    pub fn rotate_x(theta: f32) -> Self {
        Mat4 { data: [
            1., 0.,          0.,           0.,
            0., theta.cos(), -theta.sin(), 0.,
            0., theta.sin(), theta.cos(),  0.,
            0., 0.,          0.,           1.,
        ]}
    }

    pub fn rotate_y(theta: f32) -> Self {
        Mat4 { data: [
            theta.cos(),  0., theta.sin(), 0.,
            0.,           1., 0.,          0.,
            -theta.sin(), 0., theta.cos(), 0.,
            0.,           0., 0.,          1.,
        ]}
    }

    pub fn rotate_z(theta: f32) -> Self {
        Mat4 { data: [
            theta.cos(), -theta.sin(), 0., 0.,
            theta.sin(),  theta.cos(), 0., 0.,
            0.,           0.,          1., 0.,
            0.,           0.,          0., 1.,
        ]}
    }

    pub fn set(&mut self, i: usize, j: usize, value: f32) {
        self.data[4 * j + i] = value;
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.data[4 * j + i]
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut res = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                res.set(i, j, self.get(0, j) * other.get(i, 0) 
                    + self.get(1, j) * other.get(i, 1) 
                    + self.get(2, j) * other.get(i, 2) 
                    + self.get(3, j) * other.get(i, 3)
                );
            }
        }
        res
    }
}