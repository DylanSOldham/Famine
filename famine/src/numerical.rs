pub struct EulerSolver<T> {
    pub state: Vec<T>,
    pub derivatives: Vec<T>,
    derivative_func: fn(&Vec<T>, &mut Vec<T>),
}

impl<T> EulerSolver<T> where
    T: Copy + core::ops::Mul<f32> + std::ops::AddAssign<<T>::Output> {
    pub fn new(initial_state: Vec<T>, derivative_func: fn(&Vec<T>, &mut Vec<T>)) -> Self {
        EulerSolver {
            state: initial_state.clone(),
            derivatives: initial_state,
            derivative_func
        }
    }

    pub fn step(&mut self, dt: f32) {
        (self.derivative_func)(&self.state, &mut self.derivatives);
        self.state.iter_mut()
            .zip(self.derivatives.iter())
            .for_each(|(val, derivative)| { *val += *derivative * dt; });
    }
}