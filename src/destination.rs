use std::cmp::Ordering;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::page::PdfPage;
use crate::types::{PdfDestinationInfo, PdfPoint};
use crate::util::parse_json;

#[derive(Debug, Clone)]
pub struct PdfDestination {
    handle: ObjectHandle,
}

impl PdfDestination {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(page: &PdfPage, point: PdfPoint) -> Result<Self> {
        let mut out_destination = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_destination_new(
                page.as_handle_ptr(),
                point.x,
                point.y,
                &mut out_destination,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_destination,
            "PDFDestination",
        )?))
    }

    pub fn info(&self) -> Result<PdfDestinationInfo> {
        parse_json(
            unsafe { ffi::pdf_destination_info_json(self.handle.as_ptr()) },
            "PDFDestination",
        )
    }

    #[must_use]
    pub fn page(&self) -> Option<PdfPage> {
        let ptr = unsafe { ffi::pdf_destination_page(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfPage::from_handle)
    }

    pub fn set_zoom(&self, zoom: f64) {
        unsafe { ffi::pdf_destination_set_zoom(self.handle.as_ptr(), zoom) };
    }

    #[must_use]
    pub fn compare(&self, other: &Self) -> Ordering {
        match unsafe { ffi::pdf_destination_compare(self.handle.as_ptr(), other.handle.as_ptr()) } {
            value if value < 0 => Ordering::Less,
            value if value > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
