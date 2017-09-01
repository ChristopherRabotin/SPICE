#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(dead_code)]
mod raw {
    include!(concat!(env!("OUT_DIR"), "/spice_bindings.rs"));
}

#[macro_use]
pub mod macros;
pub mod errors;
#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::ffi::CStr;

    #[test]
    fn constants() {
        assert_eq!(::raw::SPICETRUE, 1);
        assert_eq!(::raw::SPICEFALSE, 0);
    }

    #[test]
    fn errors() {
        use errors::ignore;
        ignore();
        unsafe {
            assert_eq!(::raw::failed_c(), 0);
            ::raw::sigerr_c(c_str!("some error this is really long"));
            let short_err_msg = CString::new("").unwrap().into_raw();
            ::raw::getmsg_c(c_str!("SHORT"), 40, short_err_msg);
            assert_eq!(
                CStr::from_ptr(short_err_msg).to_string_lossy(),
                "some error this is really"
            );
            assert_eq!(::raw::failed_c(), 1);
        }
    }

    #[test]
    fn load_kernel() {
        unsafe {
            ::raw::erract_c(c_str!("set"), 10, c_str!("return"));
            ::raw::furnsh_c(c_str!("krnl"));
            assert_eq!(::raw::failed_c(), 1);
        }
    }
}
