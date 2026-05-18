use std::mem::ManuallyDrop;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::PdfSize;

/// Wraps `PDFPageOverlayView`.
#[derive(Debug, Clone)]
pub struct PdfPageOverlayView {
    handle: ObjectHandle,
}

impl PdfPageOverlayView {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding `PDFPageOverlayView` API.
    pub fn new(size: PdfSize) -> Result<Self> {
        let mut out_view = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_page_overlay_view_new(size.width, size.height, &mut out_view, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_view,
            "PDFPageOverlayView",
        )?))
    }

    pub(crate) fn into_handle_ptr(self) -> *mut core::ffi::c_void {
        let this = ManuallyDrop::new(self);
        this.handle.as_ptr()
    }
}
