use std::ffi::CString;

pub fn ignore() {
    unsafe {
        ::raw::erract_c(c_str!("set"), 10, c_str!("return"));
    }
}

pub fn has_failed() -> bool {
    let failed: bool;
    unsafe {
        failed = ::raw::failed_c() == 1;
    }
    failed
}
