use std::ffi::CString;

pub fn ignore() {
    unsafe {
        ::raw::erract_c(c_str!("set"), 10, c_str!("return"));
    }
}
