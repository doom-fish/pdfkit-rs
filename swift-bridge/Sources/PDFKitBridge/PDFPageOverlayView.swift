import AppKit
import Foundation
import PDFKit

@_cdecl("pdf_page_overlay_view_new")
public func pdf_page_overlay_view_new(
    _ width: Double,
    _ height: Double,
    _ outView: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outView else {
            throw PDFBridgeError.invalidArgument("missing overlay view output pointer")
        }
        let view = NSView(frame: CGRect(x: 0, y: 0, width: width, height: height))
        outView.pointee = pdf_retain_page_overlay_view(view)
    }
}
