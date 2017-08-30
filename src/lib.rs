#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(unused_macros)]
#[macro_use]
macro_rules! c_str {
    ($string:expr) => {{CString::new($string).unwrap().into_raw()}};
}
#[allow(dead_code)]
mod raw {
    include!(concat!(env!("OUT_DIR"), "/cspice_bindings.rs"));
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    #[test]
    fn constants() {
        assert_eq!(::raw::SPICETRUE, 1);
        assert_eq!(::raw::SPICEFALSE, 0);
    }

    #[test]
    fn errors() {
        unsafe {
            ::raw::erract_c(c_str!("set"), 10, c_str!("return"));
            ::raw::sigerr_c(c_str!("some error this is really long"));
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
