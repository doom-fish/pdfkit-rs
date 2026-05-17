use std::ptr;

use crate::action::{sealed, PdfActionLike};
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::PdfPoint;
use crate::util::{c_string, take_string};

#[derive(Debug, Clone)]
pub struct PdfActionRemoteGoTo {
    handle: ObjectHandle,
}

impl PdfActionRemoteGoTo {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(page_index: usize, point: PdfPoint, url: &str) -> Result<Self> {
        let url = c_string(url)?;
        let mut out_action = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_action_remote_goto_new(
                page_index as u64,
                point.x,
                point.y,
                url.as_ptr(),
                &mut out_action,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_action,
            "PDFActionRemoteGoTo",
        )?))
    }

    #[must_use]
    pub fn page_index(&self) -> usize {
        unsafe { ffi::pdf_action_remote_goto_page_index(self.handle.as_ptr()) as usize }
    }

    pub fn set_page_index(&self, page_index: usize) {
        unsafe {
            ffi::pdf_action_remote_goto_set_page_index(self.handle.as_ptr(), page_index as u64);
        };
    }

    #[must_use]
    pub fn point(&self) -> PdfPoint {
        PdfPoint {
            x: unsafe { ffi::pdf_action_remote_goto_point_x(self.handle.as_ptr()) },
            y: unsafe { ffi::pdf_action_remote_goto_point_y(self.handle.as_ptr()) },
        }
    }

    pub fn set_point(&self, point: PdfPoint) {
        unsafe { ffi::pdf_action_remote_goto_set_point(self.handle.as_ptr(), point.x, point.y) };
    }

    #[must_use]
    pub fn url(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_remote_goto_url_string(self.handle.as_ptr()) })
    }

    pub fn set_url(&self, url: &str) -> Result<()> {
        let url = c_string(url)?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_action_remote_goto_set_url(self.handle.as_ptr(), url.as_ptr(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn action_type(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_remote_goto_type_string(self.handle.as_ptr()) })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}

impl sealed::Sealed for PdfActionRemoteGoTo {}

impl PdfActionLike for PdfActionRemoteGoTo {
    fn as_action_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.as_handle_ptr()
    }
}
