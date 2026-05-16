#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_annotation_new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        annotation_type: *const c_char,
        out_annotation: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_annotation_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_annotation_set_contents(
        handle: *mut c_void,
        value: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_annotation_set_highlighted(handle: *mut c_void, value: i32);
    pub fn pdf_annotation_border(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_annotation_set_border(
        annotation_handle: *mut c_void,
        border_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_annotation_action(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_annotation_set_action(
        annotation_handle: *mut c_void,
        action_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_annotation_action_url(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_annotation_action_goto(handle: *mut c_void) -> *mut c_void;
}
