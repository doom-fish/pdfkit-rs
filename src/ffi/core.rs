#![allow(missing_docs)]

use core::ffi::c_void;

unsafe extern "C" {
    pub fn pdf_object_retain(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_object_release(handle: *mut c_void);
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const NULL_RESULT: i32 = -2;
    pub const FRAMEWORK: i32 = -3;
}
