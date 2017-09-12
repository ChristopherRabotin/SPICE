extern crate chrono;

use self::chrono::prelude::NaiveDateTime;
use datetime::chrono::format::ParseError;
use errors::{ignore, latest, SPICEError};
use std::ffi::CStr;
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
    /*
    // TODO: Move this into Kernels
    // TODO: Make this immutable by changing the loaded kernel mutability in the Kernel module
    fn load_time_kernel(&mut self) {
        if !self.krnl_loaded {
            match env::var("SPICE_KERNELS") {
                Ok(val) => unsafe {
                    ::raw::furnsh_c(c_str!(val + "/naif0012.tls"));
                },
                Err(e) => panic!("couldn't interpret SPICE_KERNELS: {}", e),
            }
            self.krnl_loaded = true;
        }
    }
*/
    fn convert(&self, format: &'static str) -> Result<String, SPICEError> {
        ignore();
        //self.load_time_kernel();
        ::kernels::load("naif0012.tls");
        let mut utc_cstr: [::raw::SpiceChar; 30] = [0; 30];
        let utc_str: String;
        unsafe {
            ::raw::et2utc_c(self.et, c_str!(format), 6, 29, utc_cstr.as_mut_ptr());
            utc_str = CStr::from_ptr(utc_cstr.as_ptr())
                .to_string_lossy()
                .into_owned();
        }
        match latest() {
            Some(err) => Err(err),
            None => Ok(utc_str),
        }
    }

    pub fn as_iso(&mut self) -> Result<String, SPICEError> {
        self.convert("C")
    }

    pub fn as_julian(&mut self) -> Result<f64, SPICEError> {
        match self.convert("J") {
            Err(err) => Err(err),
            Ok(julian_str) => {;
                let jd: f64 = julian_str.split(" ").collect::<Vec<_>>()[1]
                    .parse()
                    .unwrap();
                return Ok(jd);
            }
        }
    }
    pub fn as_datetime(&mut self) -> Result<NaiveDateTime, ParseError> {
        return NaiveDateTime::parse_from_str(
            self.as_iso().unwrap().as_str(),
            "%Y %b %d %H:%M:%S%.6f",
        );
    }
}

#[cfg(test)]
mod tests {
    /*#[test]
    fn et() {
        let et = EphemerisTime { et: -527644192.5403653 };
        println!("{:?}", et.as_iso());
    }*/
}
