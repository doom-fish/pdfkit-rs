use crate::annotation::PdfAnnotation;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::selection::PdfSelection;
use crate::types::{DisplayBox, PdfRect};
use crate::util::take_string;

#[derive(Debug, Clone)]
pub struct PdfPage {
    handle: ObjectHandle,
}

impl PdfPage {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
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

    #[must_use]
    pub fn selection_for_range(&self, location: usize, length: usize) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_page_selection_for_range(self.handle.as_ptr(), location as u64, length as u64)
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
