#[allow(unused_macros)]
macro_rules! c_str {
    ($string:expr) => {{CString::new($string).unwrap().into_raw()}};
}
