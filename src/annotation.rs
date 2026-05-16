use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::PdfAnnotationInfo;
use crate::util::parse_json;

#[derive(Debug, Clone)]
pub struct PdfAnnotation {
    handle: ObjectHandle,
}

impl PdfAnnotation {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn info(&self) -> Result<PdfAnnotationInfo> {
        parse_json(
            unsafe { ffi::pdf_annotation_info_json(self.handle.as_ptr()) },
            "PDFAnnotation",
        )
    }
}
