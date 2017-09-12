use errors::{ignore, latest, SPICEError};
use std::env;
use std::ffi::CString; // needed for c_str macro
use std::error::Error;

/// Loads a new kernel
pub fn load(krnl: &'static str) -> Option<SPICEError> {
    ignore();
    match env::var("SPICE_KERNELS") {
        Ok(val) => {
            unsafe {
                ::raw::furnsh_c(c_str!(val + "/" + krnl));
            };
            latest()
        }
        Err(e) => Some(SPICEError {
            short: String::from("SPICE_KERNELS"),
            long: String::from(e.description()),
        }),
    }
}

/// Unloads a kernel
pub fn unload(krnl: &'static str) -> Option<SPICEError> {
    ignore();
    match env::var("SPICE_KERNELS") {
        Ok(val) => {
            unsafe {
                ::raw::unload_c(c_str!(val + "/" + krnl));
            };
            latest()
        }
        Err(e) => Some(SPICEError {
            short: String::from("SPICE_KERNELS"),
            long: String::from(e.description()),
        }),
    }
}
