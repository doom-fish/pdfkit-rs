#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type PdfPageOverlayViewProviderOverlayCallback = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        view_handle: *mut c_void,
        page_handle: *mut c_void,
    ) -> *mut c_void,
>;
pub type PdfPageOverlayViewProviderDisplayCallback = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        view_handle: *mut c_void,
        overlay_view_handle: *mut c_void,
        page_handle: *mut c_void,
    ),
>;

unsafe extern "C" {
    pub fn pdf_page_overlay_view_provider_new(
        context: *mut c_void,
        overlay_callback: PdfPageOverlayViewProviderOverlayCallback,
        will_display_callback: PdfPageOverlayViewProviderDisplayCallback,
        will_end_displaying_callback: PdfPageOverlayViewProviderDisplayCallback,
        out_provider: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
