use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::take_string;

#[derive(Debug, Clone)]
pub struct PdfOutline {
    handle: ObjectHandle,
}

impl PdfOutline {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    #[must_use]
    pub fn label(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_outline_label_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn child_count(&self) -> usize {
        unsafe { ffi::pdf_outline_child_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn child(&self, index: usize) -> Option<Self> {
        let ptr = unsafe { ffi::pdf_outline_child_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle)
    }

    #[must_use]
    pub fn children(&self) -> Vec<Self> {
        (0..self.child_count())
            .filter_map(|index| self.child(index))
            .collect()
    }
}
