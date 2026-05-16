use crate::ffi;
use crate::handle::ObjectHandle;
use crate::page::PdfPage;
use crate::types::PdfRect;
use crate::util::take_string;

#[derive(Debug, Clone)]
pub struct PdfSelection {
    handle: ObjectHandle,
}

impl PdfSelection {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
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
}
