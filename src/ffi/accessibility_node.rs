#![allow(missing_docs)]

use core::ffi::c_char;

unsafe extern "C" {
    pub fn pdf_accessibility_node_public_api_available() -> i32;
    pub fn pdf_accessibility_node_reason() -> *mut c_char;
}
