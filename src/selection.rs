use std::ptr;

use crate::document::PdfDocument;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::page::PdfPage;
use crate::types::{PdfRect, PdfTextRange};
use crate::util::take_string;

#[derive(Debug, Clone)]
pub struct PdfSelection {
    handle: ObjectHandle,
}

impl PdfSelection {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(document: &PdfDocument) -> Result<Self> {
        let mut out_selection = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_selection_new(document.as_handle_ptr(), &mut out_selection, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_selection,
            "PDFSelection",
        )?))
    }

    #[must_use]
    pub fn string(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_selection_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn page_count(&self) -> usize {
        unsafe { ffi::pdf_selection_page_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn page(&self, index: usize) -> Option<PdfPage> {
        let ptr = unsafe { ffi::pdf_selection_page_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfPage::from_handle)
    }

    #[must_use]
    pub fn pages(&self) -> Vec<PdfPage> {
        (0..self.page_count())
            .filter_map(|index| self.page(index))
            .collect()
    }

    #[must_use]
    pub fn bounds_for_page(&self, page: &PdfPage) -> PdfRect {
        let mut x = 0.0_f64;
        let mut y = 0.0_f64;
        let mut width = 0.0_f64;
        let mut height = 0.0_f64;
        unsafe {
            ffi::pdf_selection_bounds_for_page(
                self.handle.as_ptr(),
                page.as_handle_ptr(),
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
    pub fn number_of_text_ranges_on_page(&self, page: &PdfPage) -> usize {
        unsafe {
            ffi::pdf_selection_number_of_text_ranges_on_page(
                self.handle.as_ptr(),
                page.as_handle_ptr(),
            ) as usize
        }
    }

    #[must_use]
    pub fn text_range(&self, index: usize, page: &PdfPage) -> Option<PdfTextRange> {
        let mut location = 0_u64;
        let mut length = 0_u64;
        let ok = unsafe {
            ffi::pdf_selection_range_at_index_on_page(
                self.handle.as_ptr(),
                index as u64,
                page.as_handle_ptr(),
                &mut location,
                &mut length,
            ) != 0
        };
        ok.then_some(PdfTextRange {
            location: location as usize,
            length: length as usize,
        })
    }

    #[must_use]
    pub fn selection_by_line_count(&self) -> usize {
        unsafe { ffi::pdf_selection_selections_by_line_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn selection_by_line(&self, index: usize) -> Option<Self> {
        let ptr =
            unsafe { ffi::pdf_selection_selection_by_line_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle)
    }

    #[must_use]
    pub fn selections_by_line(&self) -> Vec<Self> {
        (0..self.selection_by_line_count())
            .filter_map(|index| self.selection_by_line(index))
            .collect()
    }

    pub fn add_selection(&self, other: &Self) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_selection_add_selection(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn extend_selection_at_end(&self, amount: isize) {
        unsafe { ffi::pdf_selection_extend_at_end(self.handle.as_ptr(), amount as i64) };
    }

    pub fn extend_selection_at_start(&self, amount: isize) {
        unsafe { ffi::pdf_selection_extend_at_start(self.handle.as_ptr(), amount as i64) };
    }

    pub fn extend_selection_for_line_boundaries(&self) {
        unsafe { ffi::pdf_selection_extend_for_line_boundaries(self.handle.as_ptr()) };
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
