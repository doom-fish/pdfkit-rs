use pdfkit::prelude::*;

#[test]
fn accessibility_node_reports_unavailable_public_api() {
    assert!(!PdfAccessibilityNode::public_api_available());
    assert!(PdfAccessibilityNode::availability_note()
        .unwrap_or_default()
        .contains("PDFAccessibilityNode"));
}
