import AppKit
import Foundation
import PDFKit

public typealias PDFRustPageOverlayViewProviderOverlayCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer?
public typealias PDFRustPageOverlayViewProviderDisplayCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

final class PDFRustPageOverlayViewProvider: NSObject, PDFPageOverlayViewProvider {

    let context: UnsafeMutableRawPointer?
    let overlayCallback: PDFRustPageOverlayViewProviderOverlayCallback?
    let willDisplayCallback: PDFRustPageOverlayViewProviderDisplayCallback?
    let willEndDisplayingCallback: PDFRustPageOverlayViewProviderDisplayCallback?
    private let contextRelease: PDFDocumentDelegateContextCallback?

    init(
        context: UnsafeMutableRawPointer?,
        overlayCallback: PDFRustPageOverlayViewProviderOverlayCallback?,
        willDisplayCallback: PDFRustPageOverlayViewProviderDisplayCallback?,
        willEndDisplayingCallback: PDFRustPageOverlayViewProviderDisplayCallback?,
        contextRetain: PDFDocumentDelegateContextCallback?,
        contextRelease: PDFDocumentDelegateContextCallback?
    ) {
        self.context = context
        self.overlayCallback = overlayCallback
        self.willDisplayCallback = willDisplayCallback
        self.willEndDisplayingCallback = willEndDisplayingCallback
        self.contextRelease = contextRelease
        super.init()
        if let context {
            contextRetain?(context)
        }
    }

    deinit {
        if let context {
            contextRelease?(context)
        }
    }

    @objc(pdfView:overlayViewForPage:)
    func pdfView(_ view: PDFView, overlayViewFor page: PDFPage) -> NSView? {
        guard let callback = overlayCallback,
              let handle = callback(context, pdf_retain_view(view), pdf_retain_page(page))
        else {
            return nil
        }
        defer { pdf_object_release(handle) }
        return pdf_page_overlay_view_value(handle)
    }

    @objc(pdfView:willDisplayOverlayView:forPage:)
    func pdfView(_ pdfView: PDFView, willDisplayOverlayView overlayView: NSView, for page: PDFPage) {
        willDisplayCallback?(context, pdf_retain_view(pdfView), pdf_retain_page_overlay_view(overlayView), pdf_retain_page(page))
    }

    @objc(pdfView:willEndDisplayingOverlayView:forPage:)
    func pdfView(_ pdfView: PDFView, willEndDisplayingOverlayView overlayView: NSView, for page: PDFPage) {
        willEndDisplayingCallback?(context, pdf_retain_view(pdfView), pdf_retain_page_overlay_view(overlayView), pdf_retain_page(page))
    }
}

@_cdecl("pdf_page_overlay_view_provider_new")
public func pdf_page_overlay_view_provider_new(
    _ context: UnsafeMutableRawPointer?,
    _ overlayCallback: PDFRustPageOverlayViewProviderOverlayCallback?,
    _ willDisplayCallback: PDFRustPageOverlayViewProviderDisplayCallback?,
    _ willEndDisplayingCallback: PDFRustPageOverlayViewProviderDisplayCallback?,
    _ contextRetain: @escaping PDFDocumentDelegateContextCallback,
    _ contextRelease: @escaping PDFDocumentDelegateContextCallback,
    _ outProvider: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outProvider else {
            throw PDFBridgeError.invalidArgument("missing page overlay view provider output pointer")
        }
        let provider = PDFRustPageOverlayViewProvider(
            context: context,
            overlayCallback: overlayCallback,
            willDisplayCallback: willDisplayCallback,
            willEndDisplayingCallback: willEndDisplayingCallback,
            contextRetain: contextRetain,
            contextRelease: contextRelease
        )
        outProvider.pointee = pdf_retain_page_overlay_view_provider(provider)
    }
}
