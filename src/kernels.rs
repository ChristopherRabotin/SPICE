use errors::{ignore, latest, SPICEError};
use std::env;
use std::ffi::CString; // needed for c_str macro
use std::error::Error;

use std::collections::HashMap;

lazy_static! {
    // Using a HashMap for the O(1) access
    static ref loaded_kernels: HashMap<&'static str, bool> = {
        let m = HashMap::new();
        m
    };
}

pub fn load(krnl: &'static str) -> Option<SPICEError> {
    match loaded_kernels.get(krnl) {
        Some(_) => None,
        _ => {
            // Kernel isn't yet loaded
            match env::var("SPICE_KERNELS") {
                Ok(val) => {
                    unsafe {
                        ::raw::furnsh_c(c_str!(val + "/" + krnl));
                    };
                    loaded_kernels.insert(krnl, true);
                    latest()
                }
                Err(e) => Some(SPICEError {
                    short: String::from("SPICE_KERNELS"),
                    long: String::from(e.description()),
                }),
            }
        }
    }
}
