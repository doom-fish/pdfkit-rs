use std::ptr;

use crate::action::{PdfAction, PdfActionLike};
use crate::action_goto::PdfActionGoTo;
use crate::action_url::PdfActionUrl;
use crate::annotation_constants::PdfAnnotationSubtype;
use crate::border::PdfBorder;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::{PdfAnnotationInfo, PdfRect};
use crate::util::{c_string, option_c_string, parse_json};

/// Wraps `PDFAnnotation`.
#[derive(Debug, Clone)]
pub struct PdfAnnotation {
    handle: ObjectHandle,
}

impl PdfAnnotation {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps `PDFAnnotation(bounds:forType:withProperties:)`.
    pub fn new(bounds: PdfRect, annotation_type: &str) -> Result<Self> {
        let annotation_type = c_string(annotation_type)?;
        let mut out_annotation = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_annotation_new(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                annotation_type.as_ptr(),
                &mut out_annotation,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_annotation,
            "PDFAnnotation",
        )?))
    }

    /// Wraps `PDFAnnotation(bounds:forType:withProperties:)` using a typed subtype.
    pub fn new_with_subtype(
        bounds: PdfRect,
        annotation_subtype: PdfAnnotationSubtype,
    ) -> Result<Self> {
        let annotation_type = c_string(annotation_subtype.name())?;
        let mut out_annotation = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_annotation_new(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                annotation_type.as_ptr(),
                &mut out_annotation,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_annotation,
            "PDFAnnotation",
        )?))
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn info(&self) -> Result<PdfAnnotationInfo> {
        parse_json(
            unsafe { ffi::pdf_annotation_info_json(self.handle.as_ptr()) },
            "PDFAnnotation",
        )
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn set_contents(&self, value: Option<&str>) -> Result<()> {
        let value = option_c_string(value)?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_annotation_set_contents(
                self.handle.as_ptr(),
                value.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn set_highlighted(&self, value: bool) {
        unsafe { ffi::pdf_annotation_set_highlighted(self.handle.as_ptr(), i32::from(value)) };
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    #[must_use]
    pub fn border(&self) -> Option<PdfBorder> {
        let ptr = unsafe { ffi::pdf_annotation_border(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfBorder::from_handle)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn set_border(&self, border: Option<&PdfBorder>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_annotation_set_border(
                self.handle.as_ptr(),
                border.map_or(ptr::null_mut(), PdfBorder::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    #[must_use]
    pub fn action(&self) -> Option<PdfAction> {
        let ptr = unsafe { ffi::pdf_annotation_action(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfAction::from_handle)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn set_action<A: PdfActionLike>(&self, action: Option<&A>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_annotation_set_action(
                self.handle.as_ptr(),
                action.map_or(ptr::null_mut(), PdfActionLike::as_action_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn clear_action(&self) -> Result<()> {
        self.set_action::<PdfAction>(None)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    #[must_use]
    pub fn action_url(&self) -> Option<PdfActionUrl> {
        let ptr = unsafe { ffi::pdf_annotation_action_url(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionUrl::from_handle)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn set_action_url(&self, action: Option<&PdfActionUrl>) -> Result<()> {
        self.set_action(action)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    #[must_use]
    pub fn action_goto(&self) -> Option<PdfActionGoTo> {
        let ptr = unsafe { ffi::pdf_annotation_action_goto(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionGoTo::from_handle)
    }

    /// Wraps the corresponding `PDFAnnotation` API.
    pub fn set_action_goto(&self, action: Option<&PdfActionGoTo>) -> Result<()> {
        self.set_action(action)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
