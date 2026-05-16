#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_view_new(
        width: f64,
        height: f64,
        out_view: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_view_document(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_view_set_document(
        view_handle: *mut c_void,
        document_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_current_page(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_view_current_destination(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_view_current_selection(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_view_set_current_selection(
        view_handle: *mut c_void,
        selection_handle: *mut c_void,
        animate: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_clear_selection(handle: *mut c_void);
    pub fn pdf_view_go_to_page(
        view_handle: *mut c_void,
        page_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_go_to_destination(
        view_handle: *mut c_void,
        destination_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_go_to_selection(
        view_handle: *mut c_void,
        selection_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_set_display_mode(
        handle: *mut c_void,
        raw_value: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_set_display_direction(
        handle: *mut c_void,
        raw_value: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_set_display_box(
        handle: *mut c_void,
        raw_value: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_view_set_auto_scales(handle: *mut c_void, value: i32);
    pub fn pdf_view_set_scale_factor(handle: *mut c_void, value: f64);
    pub fn pdf_view_set_min_scale_factor(handle: *mut c_void, value: f64);
    pub fn pdf_view_set_max_scale_factor(handle: *mut c_void, value: f64);
    pub fn pdf_view_layout_document_view(handle: *mut c_void);
    pub fn pdf_view_visible_page_count(handle: *mut c_void) -> u64;
    pub fn pdf_view_visible_page_at(handle: *mut c_void, index: u64) -> *mut c_void;
}
