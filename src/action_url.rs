use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::{c_string, take_string};

#[derive(Debug, Clone)]
pub struct PdfActionUrl {
    handle: ObjectHandle,
}

impl PdfActionUrl {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(url: &str) -> Result<Self> {
        let url = c_string(url)?;
        let mut out_action = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_action_url_new(url.as_ptr(), &mut out_action, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_action,
            "PDFActionURL",
        )?))
    }

    #[must_use]
    pub fn url(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_url_string(self.handle.as_ptr()) })
    }

    pub fn set_url(&self, url: &str) -> Result<()> {
        let url = c_string(url)?;
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_action_url_set_url(self.handle.as_ptr(), url.as_ptr(), &mut out_error) };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn action_type(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_url_type_string(self.handle.as_ptr()) })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
