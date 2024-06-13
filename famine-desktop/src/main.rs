use std::ffi::CString;
use famine::Window;

extern "C" {
    fn window_create(title: *const libc::c_char, width: libc::size_t, height: libc::size_t) -> *const libc::c_void;
    fn window_destroy(window: *const libc::c_void);
    fn window_should_close(window: *const libc::c_void) -> bool;
    fn window_clear(window: *const libc::c_void, r: libc::c_float, g: libc::c_float, b: libc::c_float, a: libc::c_float);
    fn window_process(window: *const libc::c_void);
}

#[repr(C)]
struct DesktopWindow {
    __impl: *const libc::c_void,
}

impl Window for DesktopWindow {
    fn new(title: &str, width: usize, height: usize) -> DesktopWindow {

        let cstr: CString = CString::new(title).unwrap();
        let raw_title = cstr.as_ptr();

        unsafe { DesktopWindow { __impl: window_create(raw_title, width, height) } }
    }

    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { window_clear(self.__impl, r, g, b, a) }
    }
}

impl Drop for DesktopWindow {
    fn drop(&mut self) {
        unsafe { window_destroy( self.__impl ) }
    }
}

fn main() {
    let window: DesktopWindow = famine_application::startup::<DesktopWindow>();
    while !unsafe { window_should_close(window.__impl) } {
        famine_application::update(&window);
        unsafe { window_process(window.__impl) };
    }
}
