use crate::ffi;
use crate::util::take_string;

/// Wraps `PDFAccessibilityNode`.
#[derive(Debug, Clone, Copy, Default)]
pub struct PdfAccessibilityNode;

impl PdfAccessibilityNode {
    /// Wraps the corresponding `PDFAccessibilityNode` API.
    #[must_use]
    pub fn public_api_available() -> bool {
        unsafe { ffi::pdf_accessibility_node_public_api_available() != 0 }
    }

    /// Wraps the corresponding `PDFAccessibilityNode` API.
    #[must_use]
    pub fn availability_note() -> Option<String> {
        take_string(unsafe { ffi::pdf_accessibility_node_reason() })
    }
}
