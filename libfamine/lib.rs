use std::ffi::CString;

extern "C" {
    fn window_create(title: *const libc::c_char, width: libc::size_t, height: libc::size_t) -> *const libc::c_void;
    fn window_destroy(window: *const libc::c_void);
    fn window_should_close(window: *const libc::c_void) -> bool;
}

#[repr(C)]
pub struct Window {
    __impl: *const libc::c_void,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Window {

        let cstr: CString = CString::new(title).unwrap();
        let raw_title = cstr.as_ptr();

        unsafe { Window { __impl: window_create(raw_title, width, height) } }
    }

    pub fn should_close(&self) -> bool {
        return unsafe { window_should_close(self.__impl) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { window_destroy( self.__impl ) }
    }
}