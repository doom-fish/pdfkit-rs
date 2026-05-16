use std::ptr;

use crate::action::{sealed, PdfActionLike};
use crate::destination::PdfDestination;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::take_string;

#[derive(Debug, Clone)]
pub struct PdfActionGoTo {
    handle: ObjectHandle,
}

impl PdfActionGoTo {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(destination: &PdfDestination) -> Result<Self> {
        let mut out_action = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_action_goto_new(destination.as_handle_ptr(), &mut out_action, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_action,
            "PDFActionGoTo",
        )?))
    }

    #[must_use]
    pub fn destination(&self) -> Option<PdfDestination> {
        let ptr = unsafe { ffi::pdf_action_goto_destination(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfDestination::from_handle)
    }

    pub fn set_destination(&self, destination: &PdfDestination) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_action_goto_set_destination(
                self.handle.as_ptr(),
                destination.as_handle_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn action_type(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_goto_type_string(self.handle.as_ptr()) })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}

impl sealed::Sealed for PdfActionGoTo {}

impl PdfActionLike for PdfActionGoTo {
    fn as_action_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.as_handle_ptr()
    }
}
