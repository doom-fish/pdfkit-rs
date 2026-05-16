import Foundation

@_cdecl("pdf_accessibility_node_public_api_available")
public func pdf_accessibility_node_public_api_available() -> Int32 {
    0
}

@_cdecl("pdf_accessibility_node_reason")
public func pdf_accessibility_node_reason() -> UnsafeMutablePointer<CChar>? {
    pdf_string("PDFAccessibilityNode is only forward-declared in PDFPage.h and has no public PDFKit header surface on macOS.")
}
