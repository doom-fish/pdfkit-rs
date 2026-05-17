#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_action_type_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_action_as_url(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_action_as_goto(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_action_as_named(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_action_as_remote_goto(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_action_as_reset_form(handle: *mut c_void) -> *mut c_void;
}
