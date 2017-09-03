extern crate chrono;

use self::chrono::prelude::DateTime;
use errors::{ignore, latest, SPICEError};
use std::ffi::CStr;
use std::env;
use std::ffi::CString; // needed for c_str macro

/// EphemerisTime is the ET as defined in NAIF SPICE's
/// [required](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/req/time.html#Ephemeris Time (ET))
// reading. As such, the Terrestrial Barycentric Time is stored in EphemerisTime.et.
#[derive(Clone, Copy, Debug)]
pub struct EphemerisTime {
    et: f64,
    krnl_loaded: bool, // Will be moved to kernel.rs later
}

/*
pub fn et2utc_c(et: SpiceDouble, format: *mut ConstSpiceChar,
                prec: SpiceInt, lenout: SpiceInt, utcstr: *mut SpiceChar);
                */
impl EphemerisTime {
    pub fn from_et(et: f64) -> EphemerisTime {
        EphemerisTime {
            et: et,
            krnl_loaded: false,
        }
    }
    // TODO: Move this into Kernels
    fn load_time_kernel(&mut self) {
        if !self.krnl_loaded {
            /*println!("pwd = {:?}", env::current_exe());
            match env::var("SPICE_KERNELS") {
                Ok(val) => unsafe {
                    ::raw::furnsh_c(c_str!(val + "/naif0012.tls"));
                },
                Err(e) => panic!("couldn't interpret SPICE_KERNELS: {}", e),
            }*/
            unsafe {
                ::raw::furnsh_c(c_str!("naif0012.tls"));
            }
            self.krnl_loaded = true;
        }
    }

    pub fn as_iso(&mut self) -> Result<String, SPICEError> {
        ignore();
        self.load_time_kernel();
        let mut utc_cstr: [::raw::SpiceChar; 30] = [0; 30];
        let utc_str: String;
        unsafe {
            ::raw::et2utc_c(self.et, c_str!("J"), 9, 29, utc_cstr.as_mut_ptr()); // XXX: Set to ISOC
            utc_str = CStr::from_ptr(utc_cstr.as_ptr())
                .to_string_lossy()
                .into_owned();
        }
        match latest() {
            Some(err) => Err(err),
            None => Ok(utc_str),
        }
    }
    //    pub fn as_datetime(&self) -> DateTime {}
}

#[cfg(test)]
mod tests {
    /*#[test]
    fn et() {
        let et = EphemerisTime { et: -527644192.5403653 };
        println!("{:?}", et.as_iso());
    }*/
}
