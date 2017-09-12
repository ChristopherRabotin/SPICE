
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(dead_code)]
mod raw {
    include!(concat!(env!("OUT_DIR"), "/spice_bindings.rs"));
}

extern crate chrono;

#[macro_use]
pub mod macros;
pub mod errors;
pub mod datetime;
pub mod kernels;

#[cfg(test)]
mod tests {
    use datetime::EphemerisTime;

    #[test]
    fn constants() {
        assert_eq!(::raw::SPICETRUE, 1);
        assert_eq!(::raw::SPICEFALSE, 0);
    }

    #[test]
    fn errors() {
        use std::ffi::CString;
        use errors::{ignore, has_failed, latest};
        ignore();
        assert_eq!(has_failed(), false);
        unsafe {
            ::raw::setmsg_c(c_str!("some error occured"));
            ::raw::sigerr_c(c_str!("error"));
        }
        assert_eq!(has_failed(), true);
        let err = latest().unwrap();
        assert_eq!(err.short, "error");
        assert_eq!(err.long, "some error occured");
    }

    #[test]
    fn ephemeris_time() {
        use chrono::prelude::{NaiveDate, NaiveDateTime};
        let mut et = EphemerisTime::from_et(-527644192.5403653);
        match et.as_iso() {
            Err(e) => panic!("ERRORED = {:?}", e),
            Ok(val) => assert_eq!("1983 APR 13 12:09:14.274000", val),
        };
        match et.as_julian() {
            Err(e) => panic!("ERRORED = {:?}", e),
            Ok(val) => assert_eq!(2445438.006415, val),
        }
        match et.as_datetime() {
            Err(e) => panic!("ERRORED: {:?}", e),
            Ok(val) => {
                let dt: NaiveDateTime =
                    NaiveDate::from_ymd(1983, 4, 13).and_hms_milli(12, 9, 14, 274);
                assert_eq!(dt, val)
            }
        }
    }

}
