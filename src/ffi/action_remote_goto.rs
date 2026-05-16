#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_action_remote_goto_new(
        page_index: u64,
        x: f64,
        y: f64,
        url_string: *const c_char,
        out_action: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_remote_goto_page_index(handle: *mut c_void) -> u64;
    pub fn pdf_action_remote_goto_set_page_index(handle: *mut c_void, page_index: u64);
    pub fn pdf_action_remote_goto_point_x(handle: *mut c_void) -> f64;
    pub fn pdf_action_remote_goto_point_y(handle: *mut c_void) -> f64;
    pub fn pdf_action_remote_goto_set_point(handle: *mut c_void, x: f64, y: f64);
    pub fn pdf_action_remote_goto_url_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_action_remote_goto_set_url(
        handle: *mut c_void,
        url_string: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_remote_goto_type_string(handle: *mut c_void) -> *mut c_char;
}
