import AppKit
import Foundation
import PDFKit

public typealias PDFRustViewDelegateLinkClickCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Int32
public typealias PDFRustViewDelegateScaleFactorCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    Double
) -> Double
public typealias PDFRustViewDelegatePrintJobTitleCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>?
public typealias PDFRustViewDelegateBoolCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Int32
public typealias PDFRustViewDelegateRemoteGoToCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Int32

final class PDFRustViewDelegate: NSObject, PDFViewDelegate {

    let context: UnsafeMutableRawPointer?
    let linkClickCallback: PDFRustViewDelegateLinkClickCallback?
    let scaleFactorCallback: PDFRustViewDelegateScaleFactorCallback?
    let printJobTitleCallback: PDFRustViewDelegatePrintJobTitleCallback?
    let performPrintCallback: PDFRustViewDelegateBoolCallback?
    let performFindCallback: PDFRustViewDelegateBoolCallback?
    let performGoToPageCallback: PDFRustViewDelegateBoolCallback?
    let remoteGoToCallback: PDFRustViewDelegateRemoteGoToCallback?

    init(
        context: UnsafeMutableRawPointer?,
        linkClickCallback: PDFRustViewDelegateLinkClickCallback?,
        scaleFactorCallback: PDFRustViewDelegateScaleFactorCallback?,
        printJobTitleCallback: PDFRustViewDelegatePrintJobTitleCallback?,
        performPrintCallback: PDFRustViewDelegateBoolCallback?,
        performFindCallback: PDFRustViewDelegateBoolCallback?,
        performGoToPageCallback: PDFRustViewDelegateBoolCallback?,
        remoteGoToCallback: PDFRustViewDelegateRemoteGoToCallback?
    ) {
        self.context = context
        self.linkClickCallback = linkClickCallback
        self.scaleFactorCallback = scaleFactorCallback
        self.printJobTitleCallback = printJobTitleCallback
        self.performPrintCallback = performPrintCallback
        self.performFindCallback = performFindCallback
        self.performGoToPageCallback = performGoToPageCallback
        self.remoteGoToCallback = remoteGoToCallback
    }

    private func defaultPrintJobTitle(for view: PDFView) -> String {
        if let title = view.document?.documentAttributes?["Title"] as? String, !title.isEmpty {
            return title
        }
        if let lastPathComponent = view.document?.documentURL?.lastPathComponent, !lastPathComponent.isEmpty {
            return lastPathComponent
        }
        return "PDF Document"
    }

    func pdfViewWillClick(onLink sender: PDFView, with url: URL) {
        let handled = url.absoluteString.withCString { rawURL in
            (linkClickCallback?(context, pdf_retain_view(sender), rawURL) ?? 0) != 0
        }
        if !handled {
            NSWorkspace.shared.open(url)
        }
    }

    func pdfViewWillChangeScaleFactor(_ sender: PDFView, toScale scaler: CGFloat) -> CGFloat {
        let requestedScale = Double(scaler)
        let resolvedScale = scaleFactorCallback?(context, pdf_retain_view(sender), requestedScale)
            ?? requestedScale.clamped(to: 0.1...10.0)
        return CGFloat(resolvedScale)
    }

    func pdfViewPrintJobTitle(_ sender: PDFView) -> String {
        if let callback = printJobTitleCallback,
           let title = pdf_take_string(callback(context, pdf_retain_view(sender))),
           !title.isEmpty {
            return title
        }
        return defaultPrintJobTitle(for: sender)
    }

    func pdfViewPerformPrint(_ sender: PDFView) {
        _ = performPrintCallback?(context, pdf_retain_view(sender))
    }

    func pdfViewPerformFind(_ sender: PDFView) {
        _ = performFindCallback?(context, pdf_retain_view(sender))
    }

    func pdfViewPerformGo(toPage sender: PDFView) {
        _ = performGoToPageCallback?(context, pdf_retain_view(sender))
    }

    func pdfViewOpenPDF(_ sender: PDFView, forRemoteGoToAction action: PDFActionRemoteGoTo) {
        _ = remoteGoToCallback?(context, pdf_retain_view(sender), pdf_retain_action_remote_goto(action))
    }
}

private extension Double {
    func clamped(to range: ClosedRange<Double>) -> Double {
        min(max(self, range.lowerBound), range.upperBound)
    }
}

@_cdecl("pdf_view_delegate_new")
public func pdf_view_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ linkClickCallback: PDFRustViewDelegateLinkClickCallback?,
    _ scaleFactorCallback: PDFRustViewDelegateScaleFactorCallback?,
    _ printJobTitleCallback: PDFRustViewDelegatePrintJobTitleCallback?,
    _ performPrintCallback: PDFRustViewDelegateBoolCallback?,
    _ performFindCallback: PDFRustViewDelegateBoolCallback?,
    _ performGoToPageCallback: PDFRustViewDelegateBoolCallback?,
    _ remoteGoToCallback: PDFRustViewDelegateRemoteGoToCallback?,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outDelegate else {
            throw PDFBridgeError.invalidArgument("missing view delegate output pointer")
        }
        let delegate = PDFRustViewDelegate(
            context: context,
            linkClickCallback: linkClickCallback,
            scaleFactorCallback: scaleFactorCallback,
            printJobTitleCallback: printJobTitleCallback,
            performPrintCallback: performPrintCallback,
            performFindCallback: performFindCallback,
            performGoToPageCallback: performGoToPageCallback,
            remoteGoToCallback: remoteGoToCallback
        )
        outDelegate.pointee = pdf_retain_view_delegate(delegate)
    }
}
