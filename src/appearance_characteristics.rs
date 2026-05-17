use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::{PdfAppearanceCharacteristicsInfo, PdfColor, PdfWidgetControlType};
use crate::util::{option_c_string, parse_json};

#[derive(Debug, Clone)]
pub struct PdfAppearanceCharacteristics {
    handle: ObjectHandle,
}

impl PdfAppearanceCharacteristics {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            unsafe { ffi::pdf_appearance_characteristics_new(&mut out_value, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_value,
            "PDFAppearanceCharacteristics",
        )?))
    }

    pub fn info(&self) -> Result<PdfAppearanceCharacteristicsInfo> {
        parse_json(
            unsafe { ffi::pdf_appearance_characteristics_info_json(self.handle.as_ptr()) },
            "PDFAppearanceCharacteristics",
        )
    }

    pub fn set_control_type(&self, control_type: PdfWidgetControlType) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_appearance_characteristics_set_control_type(
                self.handle.as_ptr(),
                control_type as i32,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn set_rotation(&self, rotation: i32) {
        unsafe { ffi::pdf_appearance_characteristics_set_rotation(self.handle.as_ptr(), rotation) };
    }

    pub fn set_caption(&self, caption: Option<&str>) -> Result<()> {
        let caption = option_c_string(caption)?;
        unsafe {
            ffi::pdf_appearance_characteristics_set_caption(
                self.handle.as_ptr(),
                caption.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    pub fn set_rollover_caption(&self, caption: Option<&str>) -> Result<()> {
        let caption = option_c_string(caption)?;
        unsafe {
            ffi::pdf_appearance_characteristics_set_rollover_caption(
                self.handle.as_ptr(),
                caption.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    pub fn set_down_caption(&self, caption: Option<&str>) -> Result<()> {
        let caption = option_c_string(caption)?;
        unsafe {
            ffi::pdf_appearance_characteristics_set_down_caption(
                self.handle.as_ptr(),
                caption.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    pub fn set_background_color(&self, color: PdfColor) {
        unsafe {
            ffi::pdf_appearance_characteristics_set_background_color(
                self.handle.as_ptr(),
                color.red,
                color.green,
                color.blue,
                color.alpha,
            );
        };
    }

    pub fn set_border_color(&self, color: PdfColor) {
        unsafe {
            ffi::pdf_appearance_characteristics_set_border_color(
                self.handle.as_ptr(),
                color.red,
                color.green,
                color.blue,
                color.alpha,
            );
        };
    }
}
