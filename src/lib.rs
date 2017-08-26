#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(dead_code)]
mod raw{
    include!(concat!(env!("OUT_DIR"), "/cspice_bindings.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn constants() {
        assert_eq!(::raw::SPICETRUE, 1);
        assert_eq!(::raw::SPICEFALSE, 0);
    }

    // TODO: Move this somewhere else, likely in its own kernel.rs file for kernel management
    fn load_kernel(){
        use std::ffi::CString;

        let krnl = CString::new("krnl").unwrap();
        unsafe {
            ::raw::furnsh_c(krnl.into_raw());
        }
    }
}
