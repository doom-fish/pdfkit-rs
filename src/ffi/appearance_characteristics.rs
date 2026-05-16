#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_appearance_characteristics_new(
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_appearance_characteristics_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_appearance_characteristics_set_control_type(
        handle: *mut c_void,
        raw_value: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_appearance_characteristics_set_rotation(handle: *mut c_void, value: i32);
    pub fn pdf_appearance_characteristics_set_caption(handle: *mut c_void, value: *const c_char);
    pub fn pdf_appearance_characteristics_set_rollover_caption(handle: *mut c_void, value: *const c_char);
    pub fn pdf_appearance_characteristics_set_down_caption(handle: *mut c_void, value: *const c_char);
    pub fn pdf_appearance_characteristics_set_background_color(
        handle: *mut c_void,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    );
    pub fn pdf_appearance_characteristics_set_border_color(
        handle: *mut c_void,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    );
}
