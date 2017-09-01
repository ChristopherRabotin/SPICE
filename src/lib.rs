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
    use std::ffi::CString; // needed for c_str macro
    use errors::{ignore, has_failed, latest};

    #[test]
    fn constants() {
        assert_eq!(::raw::SPICETRUE, 1);
        assert_eq!(::raw::SPICEFALSE, 0);
    }

    #[test]
    fn errors() {
        ignore();
        assert_eq!(has_failed(), false);
        unsafe {
            ::raw::setmsg_c(c_str!("some full traceback kinda thing"));
            ::raw::sigerr_c(c_str!("some error type"));
        }
        assert_eq!(has_failed(), true);
        let err = latest().unwrap();
        assert_eq!(err.short, "some error type");
        assert_eq!(err.long, "some full traceback kinda thing");
    }

    #[test]
    fn load_kernel() {
        ignore();
        assert_eq!(has_failed(), false);
        unsafe {
            ::raw::furnsh_c(c_str!("krnl"));
        }
        assert_eq!(has_failed(), true);
    }
}
