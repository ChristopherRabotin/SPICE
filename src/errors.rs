use std::ffi::CString;
use std::ffi::CStr;

#[derive(Debug)]
pub struct SPICEError {
    pub short: String,
    pub long: String,
}

pub fn ignore() {
    unsafe {
        ::raw::erract_c(c_str!("set"), 10, c_str!("return"));
    }
}

pub fn has_failed() -> bool {
    let failed: bool;
    unsafe {
        failed = ::raw::failed_c() == 1;
    }
    failed
}

pub fn latest() -> Option<SPICEError> {
    if has_failed() {
        let mut short_err: [::raw::SpiceChar; 40] = [0; 40];
        let mut long_err: [::raw::SpiceChar; 200] = [0; 200];
        let short: String;
        let long: String;
        unsafe {
            ::raw::getmsg_c(c_str!("SHORT"), 40, short_err.as_mut_ptr());
            ::raw::getmsg_c(c_str!("LONG"), 200, long_err.as_mut_ptr());
            short = CStr::from_ptr(short_err.as_ptr())
                .to_string_lossy()
                .into_owned();
            long = CStr::from_ptr(long_err.as_ptr())
                .to_string_lossy()
                .into_owned();
            // Reset the SPICE error
            ::raw::reset_c();
        }
        Some(SPICEError {
            short: short,
            long: long,
        })
    } else {
        None
    }
}
