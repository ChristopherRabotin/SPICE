#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(dead_code)]
mod raw{
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
/*
    #[test]
    // The following tests are from https://github.com/AndrewAnnex/SpiceyPy/blob/master/spiceypy/tests/test_spiceerrors.py
    fn errors(){
        let msg = CString::new("some error occured").unwrap();
        println!("msg = {:?}", msg);
        unsafe{
            ::raw::sigerr_c(CString::new("error").unwrap().into_raw());
            println!("yo");
            println!("{}",::raw::failed_c());
            assert_eq!(::raw::failed_c(), 1);
        }
        /*
        spice.sigerr("error")
assert spice.failed()
assert spice.getmsg("SHORT", 40) == "error"
assert spice.getmsg("LONG", 200) == "some error occured"
spice.reset()
*/
    }*/

    #[test]
    // TODO: Move this somewhere else, likely in its own kernel.rs file for kernel management
    fn load_kernel(){
        let krnl = CString::new("krnl").unwrap();
        unsafe {
            ::raw::furnsh_c(krnl.into_raw());
        }
    }
}
