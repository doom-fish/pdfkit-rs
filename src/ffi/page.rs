#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_page_new(out_page: *mut *mut c_void, out_error_message: *mut *mut c_char) -> i32;
    pub fn pdf_page_new_with_image_data(
        image_data_ptr: *const u8,
        image_data_len: usize,
        options_json: *const c_char,
        out_page: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_page_label_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_page_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_page_number_of_characters(handle: *mut c_void) -> u64;
    pub fn pdf_page_rotation(handle: *mut c_void) -> i32;
    pub fn pdf_page_set_rotation(
        handle: *mut c_void,
        rotation: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_page_bounds(
        handle: *mut c_void,
        display_box: i32,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn pdf_page_set_bounds(
        handle: *mut c_void,
        display_box: i32,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_page_document(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_page_annotation_count(handle: *mut c_void) -> u64;
    pub fn pdf_page_annotation_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn pdf_page_add_annotation(
        page_handle: *mut c_void,
        annotation_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_page_remove_annotation(
        page_handle: *mut c_void,
        annotation_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_page_annotation_at_point(page_handle: *mut c_void, x: f64, y: f64) -> *mut c_void;
    pub fn pdf_page_selection_for_range(
        handle: *mut c_void,
        location: u64,
        length: u64,
    ) -> *mut c_void;
    pub fn pdf_page_selection_for_rect(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn pdf_page_selection_for_word_at_point(handle: *mut c_void, x: f64, y: f64)
        -> *mut c_void;
    pub fn pdf_page_selection_for_line_at_point(handle: *mut c_void, x: f64, y: f64)
        -> *mut c_void;
    pub fn pdf_page_selection_from_points(
        handle: *mut c_void,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
    ) -> *mut c_void;
    pub fn pdf_page_character_bounds_at(
        handle: *mut c_void,
        index: u64,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn pdf_page_character_index_at_point(handle: *mut c_void, x: f64, y: f64) -> i64;
    pub fn pdf_page_displays_annotations(handle: *mut c_void) -> i32;
    pub fn pdf_page_set_displays_annotations(handle: *mut c_void, value: i32);
}
