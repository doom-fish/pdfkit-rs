import Foundation
import PDFKit

public typealias PDFDocumentDelegateNotificationCallback = @convention(c) (UnsafeMutableRawPointer?, Int32) -> Void
public typealias PDFDocumentDelegateMatchCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
public typealias PDFDocumentDelegatePageClassNameCallback = @convention(c) (UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>?
public typealias PDFDocumentDelegateAnnotationClassNameCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
public typealias PDFDocumentDelegateContextCallback = @convention(c) (UnsafeMutableRawPointer) -> Void

final class PDFRustDocumentDelegate: NSObject, PDFDocumentDelegate {
    private let context: UnsafeMutableRawPointer?
    private let notificationCallback: PDFDocumentDelegateNotificationCallback?
    private let matchCallback: PDFDocumentDelegateMatchCallback?
    private let pageClassNameCallback: PDFDocumentDelegatePageClassNameCallback?
    private let annotationClassNameCallback: PDFDocumentDelegateAnnotationClassNameCallback?
    private let contextRelease: PDFDocumentDelegateContextCallback?

    init(
        context: UnsafeMutableRawPointer?,
        notificationCallback: PDFDocumentDelegateNotificationCallback?,
        matchCallback: PDFDocumentDelegateMatchCallback?,
        pageClassNameCallback: PDFDocumentDelegatePageClassNameCallback?,
        annotationClassNameCallback: PDFDocumentDelegateAnnotationClassNameCallback?,
        contextRetain: PDFDocumentDelegateContextCallback?,
        contextRelease: PDFDocumentDelegateContextCallback?
    ) {
        self.context = context
        self.notificationCallback = notificationCallback
        self.matchCallback = matchCallback
        self.pageClassNameCallback = pageClassNameCallback
        self.annotationClassNameCallback = annotationClassNameCallback
        self.contextRelease = contextRelease
        super.init()
        // Take a +1 on the Rust DelegateState for the lifetime of this object so
        // an in-flight callback (e.g. an async find on a background queue) can
        // never observe freed state.
        if let context {
            contextRetain?(context)
        }
    }

    deinit {
        if let context {
            contextRelease?(context)
        }
    }

    private func resolvedPageClassName() -> AnyClass {
        guard let pageClassNameCallback,
              let className = pdf_take_string(pageClassNameCallback(context)),
              let resolvedClass = NSClassFromString(className) as? PDFPage.Type
        else {
            return PDFPage.self
        }
        return resolvedClass
    }

    private func resolvedAnnotationClassName(for annotationType: String) -> AnyClass {
        guard let annotationClassNameCallback else {
            return PDFAnnotation.self
        }
        let className = annotationType.withCString { annotationClassNameCallback(context, $0) }
        guard let className,
              let resolvedName = pdf_take_string(className),
              let resolvedClass = NSClassFromString(resolvedName) as? PDFAnnotation.Type
        else {
            return PDFAnnotation.self
        }
        return resolvedClass
    }

    @objc(documentDidUnlock:)
    func documentDidUnlock(_ notification: Notification) {
        notificationCallback?(context, 0)
    }

    @objc(documentDidBeginDocumentFind:)
    func documentDidBeginDocumentFind(_ notification: Notification) {
        notificationCallback?(context, 1)
    }

    @objc(documentDidEndDocumentFind:)
    func documentDidEndDocumentFind(_ notification: Notification) {
        notificationCallback?(context, 2)
    }

    @objc(documentDidBeginPageFind:)
    func documentDidBeginPageFind(_ notification: Notification) {
        notificationCallback?(context, 3)
    }

    @objc(documentDidEndPageFind:)
    func documentDidEndPageFind(_ notification: Notification) {
        notificationCallback?(context, 4)
    }

    @objc(documentDidFindMatch:)
    func documentDidFindMatch(_ notification: Notification) {
        notificationCallback?(context, 5)
        if let selection = notification.userInfo?["PDFDocumentFoundSelection"] as? PDFSelection {
            matchCallback?(context, pdf_retain_selection(selection))
        }
    }

    @objc(didMatchString:)
    func didMatchString(_ instance: PDFSelection) {
        matchCallback?(context, pdf_retain_selection(instance))
    }

    @objc(classForPage)
    func classForPage() -> AnyClass {
        resolvedPageClassName()
    }

    @objc(classForAnnotationType:)
    func `class`(forAnnotationType annotationType: String) -> AnyClass {
        resolvedAnnotationClassName(for: annotationType)
    }
}

@_cdecl("pdf_document_delegate_new")
public func pdf_document_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ notificationCallback: PDFDocumentDelegateNotificationCallback?,
    _ matchCallback: PDFDocumentDelegateMatchCallback?,
    _ pageClassNameCallback: PDFDocumentDelegatePageClassNameCallback?,
    _ annotationClassNameCallback: PDFDocumentDelegateAnnotationClassNameCallback?,
    _ contextRetain: PDFDocumentDelegateContextCallback,
    _ contextRelease: PDFDocumentDelegateContextCallback,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outDelegate else {
            throw PDFBridgeError.invalidArgument("missing delegate output pointer")
        }
        let delegate = PDFRustDocumentDelegate(
            context: context,
            notificationCallback: notificationCallback,
            matchCallback: matchCallback,
            pageClassNameCallback: pageClassNameCallback,
            annotationClassNameCallback: annotationClassNameCallback,
            contextRetain: contextRetain,
            contextRelease: contextRelease
        )
        outDelegate.pointee = pdf_retain_document_delegate(delegate)
    }
}
