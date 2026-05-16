use std::ptr;

use crate::annotation::PdfAnnotation;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::selection::PdfSelection;
use crate::types::{DisplayBox, PdfPoint, PdfRect};
use crate::util::take_string;

#[derive(Debug, Clone)]
pub struct PdfPage {
    handle: ObjectHandle,
}

impl PdfPage {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_page = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_page_new(&mut out_page, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_page,
            "PDFPage",
        )?))
    }

    #[must_use]
    pub fn label(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_page_label_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn string(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_page_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn number_of_characters(&self) -> usize {
        unsafe { ffi::pdf_page_number_of_characters(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn rotation(&self) -> i32 {
        unsafe { ffi::pdf_page_rotation(self.handle.as_ptr()) }
    }

    pub fn set_rotation(&self, rotation: i32) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_page_set_rotation(self.handle.as_ptr(), rotation, &mut out_error) };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn bounds(&self, display_box: DisplayBox) -> PdfRect {
        let mut x = 0.0_f64;
        let mut y = 0.0_f64;
        let mut width = 0.0_f64;
        let mut height = 0.0_f64;
        unsafe {
            ffi::pdf_page_bounds(
                self.handle.as_ptr(),
                display_box.as_raw(),
                &mut x,
                &mut y,
                &mut width,
                &mut height,
            );
        }
        PdfRect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn set_bounds(&self, display_box: DisplayBox, bounds: PdfRect) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_page_set_bounds(
                self.handle.as_ptr(),
                display_box.as_raw(),
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn document(&self) -> Option<crate::document::PdfDocument> {
        let ptr = unsafe { ffi::pdf_page_document(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(crate::document::PdfDocument::from_handle)
    }

    #[must_use]
    pub fn annotation_count(&self) -> usize {
        unsafe { ffi::pdf_page_annotation_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn annotation(&self, index: usize) -> Option<PdfAnnotation> {
        let ptr = unsafe { ffi::pdf_page_annotation_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfAnnotation::from_handle)
    }

    #[must_use]
    pub fn annotations(&self) -> Vec<PdfAnnotation> {
        (0..self.annotation_count())
            .filter_map(|index| self.annotation(index))
            .collect()
    }

    pub fn add_annotation(&self, annotation: &PdfAnnotation) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_page_add_annotation(
                self.handle.as_ptr(),
                annotation.as_handle_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn remove_annotation(&self, annotation: &PdfAnnotation) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_page_remove_annotation(
                self.handle.as_ptr(),
                annotation.as_handle_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn annotation_at_point(&self, point: PdfPoint) -> Option<PdfAnnotation> {
        let ptr = unsafe { ffi::pdf_page_annotation_at_point(self.handle.as_ptr(), point.x, point.y) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfAnnotation::from_handle)
    }

    #[must_use]
    pub fn selection_for_range(&self, location: usize, length: usize) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_page_selection_for_range(self.handle.as_ptr(), location as u64, length as u64)
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn selection_for_rect(&self, rect: PdfRect) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_page_selection_for_rect(
                self.handle.as_ptr(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            )
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn selection_for_word_at_point(&self, point: PdfPoint) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_page_selection_for_word_at_point(self.handle.as_ptr(), point.x, point.y)
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn selection_for_line_at_point(&self, point: PdfPoint) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_page_selection_for_line_at_point(self.handle.as_ptr(), point.x, point.y)
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn selection_from_points(&self, start: PdfPoint, end: PdfPoint) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_page_selection_from_points(
                self.handle.as_ptr(),
                start.x,
                start.y,
                end.x,
                end.y,
            )
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn character_bounds_at(&self, index: usize) -> PdfRect {
        let mut x = 0.0_f64;
        let mut y = 0.0_f64;
        let mut width = 0.0_f64;
        let mut height = 0.0_f64;
        unsafe {
            ffi::pdf_page_character_bounds_at(
                self.handle.as_ptr(),
                index as u64,
                &mut x,
                &mut y,
                &mut width,
                &mut height,
            );
        }
        PdfRect {
            x,
            y,
            width,
            height,
        }
    }

    #[must_use]
    pub fn character_index_at_point(&self, point: PdfPoint) -> Option<usize> {
        let index = unsafe { ffi::pdf_page_character_index_at_point(self.handle.as_ptr(), point.x, point.y) };
        (index != i64::MAX)
            .then_some(index)
            .and_then(|index| usize::try_from(index).ok())
    }

    #[must_use]
    pub fn displays_annotations(&self) -> bool {
        unsafe { ffi::pdf_page_displays_annotations(self.handle.as_ptr()) != 0 }
    }

    pub fn set_displays_annotations(&self, value: bool) {
        unsafe { ffi::pdf_page_set_displays_annotations(self.handle.as_ptr(), i32::from(value)) };
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
