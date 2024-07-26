
pub fn euler_step(dt: f32, state: &mut [f32], derivatives: &mut [f32], derivative_func: fn(&[f32], &[f32])) {
    derivative_func(state, derivatives);
    state.iter_mut()
        .zip(derivatives)
        .for_each(|(val, derivative)| { *val += *derivative * dt; });
}