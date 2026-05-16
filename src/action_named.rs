use std::ptr;

use crate::action::{sealed, PdfActionLike};
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::PdfActionNamedName;
use crate::util::take_string;

#[derive(Debug, Clone)]
pub struct PdfActionNamed {
    handle: ObjectHandle,
}

impl PdfActionNamed {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(name: PdfActionNamedName) -> Result<Self> {
        let mut out_action = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_action_named_new(name.as_raw(), &mut out_action, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_action,
            "PDFActionNamed",
        )?))
    }

    #[must_use]
    pub fn name(&self) -> Option<PdfActionNamedName> {
        PdfActionNamedName::from_raw(unsafe { ffi::pdf_action_named_name_raw(self.handle.as_ptr()) })
    }

    pub fn set_name(&self, name: PdfActionNamedName) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_action_named_set_name(self.handle.as_ptr(), name.as_raw(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn action_type(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_named_type_string(self.handle.as_ptr()) })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}

impl sealed::Sealed for PdfActionNamed {}

impl PdfActionLike for PdfActionNamed {
    fn as_action_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.as_handle_ptr()
    }
}
