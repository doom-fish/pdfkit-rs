use std::ptr;

use crate::action::{sealed, PdfActionLike};
use crate::error::{PdfKitError, Result};
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::{c_string, parse_json, take_string};

#[derive(Debug, Clone)]
pub struct PdfActionResetForm {
    handle: ObjectHandle,
}

impl PdfActionResetForm {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_action = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_action_reset_form_new(&mut out_action, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_action,
            "PDFActionResetForm",
        )?))
    }

    pub fn fields(&self) -> Result<Vec<String>> {
        parse_json(
            unsafe { ffi::pdf_action_reset_form_fields_json(self.handle.as_ptr()) },
            "PDFActionResetForm fields",
        )
    }

    pub fn set_fields<I, S>(&self, fields: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let fields = fields
            .into_iter()
            .map(|field| field.as_ref().to_string())
            .collect::<Vec<_>>();
        let fields_json = serde_json::to_string(&fields).map_err(|error| {
            PdfKitError::new(
                ffi::status::FRAMEWORK,
                format!("failed to encode PDFActionResetForm fields: {error}"),
            )
        })?;
        let fields_json = c_string(&fields_json)?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_action_reset_form_set_fields_json(
                self.handle.as_ptr(),
                fields_json.as_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn clear_fields(&self) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_action_reset_form_set_fields_json(self.handle.as_ptr(), ptr::null(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn fields_included_are_cleared(&self) -> bool {
        unsafe { ffi::pdf_action_reset_form_fields_included_are_cleared(self.handle.as_ptr()) != 0 }
    }

    pub fn set_fields_included_are_cleared(&self, value: bool) {
        unsafe {
            ffi::pdf_action_reset_form_set_fields_included_are_cleared(
                self.handle.as_ptr(),
                i32::from(value),
            );
        }
    }

    #[must_use]
    pub fn action_type(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_reset_form_type_string(self.handle.as_ptr()) })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}

impl sealed::Sealed for PdfActionResetForm {}

impl PdfActionLike for PdfActionResetForm {
    fn as_action_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.as_handle_ptr()
    }
}
