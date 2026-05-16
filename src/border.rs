use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::{PdfBorderInfo, PdfBorderStyle};
use crate::util::parse_json;

#[derive(Debug, Clone)]
pub struct PdfBorder {
    handle: ObjectHandle,
}

impl PdfBorder {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_border = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_border_new(&mut out_border, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_border,
            "PDFBorder",
        )?))
    }

    pub fn info(&self) -> Result<PdfBorderInfo> {
        parse_json(
            unsafe { ffi::pdf_border_info_json(self.handle.as_ptr()) },
            "PDFBorder",
        )
    }

    pub fn set_style(&self, style: PdfBorderStyle) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_border_set_style(self.handle.as_ptr(), style as i32, &mut out_error) };
        crate::util::status_result(status, out_error)
    }

    pub fn set_line_width(&self, width: f64) {
        unsafe { ffi::pdf_border_set_line_width(self.handle.as_ptr(), width) };
    }

    pub fn set_dash_pattern(&self, dash_pattern: Option<&[f64]>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let (values, len) = dash_pattern.map_or((ptr::null(), 0_u64), |values| (values.as_ptr(), values.len() as u64));
        let status = unsafe {
            ffi::pdf_border_set_dash_pattern(self.handle.as_ptr(), values, len, &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
