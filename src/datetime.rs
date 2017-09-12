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
    et: f64
}

impl EphemerisTime {
    pub fn from_et(et: f64) -> EphemerisTime {
        EphemerisTime {
            et: et
        }
    }

    fn convert(&self, format: &'static str) -> Result<String, SPICEError> {
        ignore();
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

    pub fn as_iso(&self) -> Result<String, SPICEError> {
        self.convert("C")
    }

    pub fn as_julian(&self) -> Result<f64, SPICEError> {
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
    pub fn as_datetime(&self) -> Result<NaiveDateTime, ParseError> {
        return NaiveDateTime::parse_from_str(
            self.as_iso().unwrap().as_str(),
            "%Y %b %d %H:%M:%S%.6f",
        );
    }
}
