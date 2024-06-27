use std::ffi::CString;
use famine::{Application, Mesh, WindowType};
use famine_application::App;

extern "C" {
    fn window_create(title: *const libc::c_char, width: libc::size_t, height: libc::size_t) -> *const libc::c_void;
    fn window_destroy(window: *const libc::c_void);
    fn window_should_close(window: *const libc::c_void) -> bool;
    fn window_clear(window: *const libc::c_void, r: libc::c_float, g: libc::c_float, b: libc::c_float, a: libc::c_float);
    fn window_process(window: *const libc::c_void);
    fn window_draw_mesh(window: *const libc::c_void, vertices: *const libc::c_float);
    fn window_use_shader(window: *const libc::c_void);
}

#[repr(C)]
struct DesktopWindow {
    __impl: *const libc::c_void,
}

struct DesktopShader {

}

impl WindowType for DesktopWindow {
    type Shader = DesktopShader;

    fn new(title: &str, width: usize, height: usize) -> DesktopWindow {

        let cstr: CString = CString::new(title).unwrap();
        let raw_title = cstr.as_ptr();

        unsafe { DesktopWindow { __impl: window_create(raw_title, width, height) } }
    }

    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { window_clear(self.__impl, r, g, b, a) }
    }

    fn draw_mesh(&self, mesh: &Mesh) {
        unsafe { window_draw_mesh(self.__impl, mesh.vertices.as_ptr()) }
    }
    
    fn new_shader(&self, _vert_src: &str, _frag_str: &str) -> Self::Shader {
        DesktopShader {}
    }

    fn use_shader(&self, _shader: &Self::Shader) {
        todo!()
    }
}

impl Drop for DesktopWindow {
    fn drop(&mut self) {
        unsafe { window_destroy( self.__impl ) }
    }
}

fn main() {
    let application = App::<DesktopWindow>::new();
    while !unsafe { window_should_close(application.get_window().__impl) } {
        application.update();
        unsafe { window_process(application.get_window().__impl) };
    }
}
