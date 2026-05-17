#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type PdfViewDelegateLinkClickCallback = Option<
    unsafe extern "C" fn(context: *mut c_void, view_handle: *mut c_void, url: *const c_char) -> i32,
>;
pub type PdfViewDelegateScaleFactorCallback = Option<
    unsafe extern "C" fn(context: *mut c_void, view_handle: *mut c_void, scale_factor: f64) -> f64,
>;
pub type PdfViewDelegatePrintJobTitleCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, view_handle: *mut c_void) -> *mut c_char>;
pub type PdfViewDelegateBoolCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, view_handle: *mut c_void) -> i32>;
pub type PdfViewDelegateRemoteGoToCallback = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        view_handle: *mut c_void,
        action_handle: *mut c_void,
    ) -> i32,
>;

unsafe extern "C" {
    pub fn pdf_view_delegate_new(
        context: *mut c_void,
        link_click_callback: PdfViewDelegateLinkClickCallback,
        scale_factor_callback: PdfViewDelegateScaleFactorCallback,
        print_job_title_callback: PdfViewDelegatePrintJobTitleCallback,
        perform_print_callback: PdfViewDelegateBoolCallback,
        perform_find_callback: PdfViewDelegateBoolCallback,
        perform_go_to_page_callback: PdfViewDelegateBoolCallback,
        remote_goto_callback: PdfViewDelegateRemoteGoToCallback,
        out_delegate: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
