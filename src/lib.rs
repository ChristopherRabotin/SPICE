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
pub mod datetime;


#[cfg(test)]
mod tests {
    use std::ffi::CString; // needed for c_str macro
    use errors::{ignore, has_failed, latest};
    use datetime::EphemerisTime;

    #[test]
    fn constants() {
        assert_eq!(::raw::SPICETRUE, 1);
        assert_eq!(::raw::SPICEFALSE, 0);
    }
    /*
    #[test]
    fn errors() {
        ignore();
        assert_eq!(has_failed(), false);
        unsafe {
            ::raw::setmsg_c(c_str!("some full traceback kinda thing"));
            ::raw::sigerr_c(c_str!("some error type"));
        }
        let err = latest().unwrap();
        assert_eq!(err.short, "some error type");
        assert_eq!(err.long, "some full traceback kinda thing");
    }*/

    #[test]
    fn ephemeris_time() {
        let mut et = EphemerisTime::from_et(-527644192.5403653);
        match et.as_iso() {
            Err(e) => panic!("ERRORED = {:?}", e),
            Ok(val) => assert_eq!("1983 APR 13 12:09:14.2740000", val),
        };
        match et.as_julian() {
            Err(e) => panic!("ERRORED = {:?}", e),
            Ok(val) => assert_eq!(2445438.0064152, val),
        }
    }
}
