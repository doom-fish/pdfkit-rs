import CoreGraphics
import Foundation
import PDFKit

private func pdf_rect_dict(_ rect: CGRect) -> [String: Any] {
    [
        "x": rect.origin.x,
        "y": rect.origin.y,
        "width": rect.size.width,
        "height": rect.size.height,
    ]
}

private func pdf_document_info(_ document: PDFDocument) -> [String: Any] {
    [
        "document_url": document.documentURL?.path as Any,
        "major_version": document.majorVersion,
        "minor_version": document.minorVersion,
        "is_encrypted": document.isEncrypted,
        "is_locked": document.isLocked,
        "permissions_status": document.permissionsStatus.rawValue,
        "allows_printing": document.allowsPrinting,
        "allows_copying": document.allowsCopying,
        "allows_document_changes": document.allowsDocumentChanges,
        "allows_document_assembly": document.allowsDocumentAssembly,
        "allows_content_accessibility": document.allowsContentAccessibility,
        "allows_commenting": document.allowsCommenting,
        "allows_form_field_entry": document.allowsFormFieldEntry,
    ]
}

private func pdf_document_attributes(_ document: PDFDocument) -> [String: Any] {
    let attributes = document.documentAttributes ?? [:]
    let formatter = ISO8601DateFormatter()
    return [
        "title": attributes[PDFDocumentAttribute.titleAttribute] as? String as Any,
        "author": attributes[PDFDocumentAttribute.authorAttribute] as? String as Any,
        "subject": attributes[PDFDocumentAttribute.subjectAttribute] as? String as Any,
        "creator": attributes[PDFDocumentAttribute.creatorAttribute] as? String as Any,
        "producer": attributes[PDFDocumentAttribute.producerAttribute] as? String as Any,
        "creation_date": (attributes[PDFDocumentAttribute.creationDateAttribute] as? Date).map(formatter.string(from:)) as Any,
        "modification_date": (attributes[PDFDocumentAttribute.modificationDateAttribute] as? Date).map(formatter.string(from:)) as Any,
        "keywords": attributes[PDFDocumentAttribute.keywordsAttribute] as? [String] as Any,
    ]
}

private func pdf_annotation_info(_ annotation: PDFAnnotation) -> [String: Any] {
    [
        "annotation_type": annotation.type as Any,
        "bounds": pdf_rect_dict(annotation.bounds),
        "contents": annotation.contents as Any,
        "should_display": annotation.shouldDisplay,
        "should_print": annotation.shouldPrint,
        "has_appearance_stream": annotation.hasAppearanceStream,
        "user_name": annotation.userName as Any,
    ]
}

private func pdf_display_box(_ rawValue: Int32) throws -> PDFDisplayBox {
    guard let box = PDFDisplayBox(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFDisplayBox: \(rawValue)")
    }
    return box
}

@_cdecl("pdf_document_new_with_url")
public func pdf_document_new_with_url(
    _ path: UnsafePointer<CChar>?,
    _ outDocument: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let path, let outDocument else {
            throw PDFBridgeError.invalidArgument("missing PDF path or output pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard let document = PDFDocument(url: url) else {
            throw PDFBridgeError.nullResult("PDFDocument(url:) returned nil")
        }
        outDocument.pointee = pdf_retain(document)
    }
}

@_cdecl("pdf_document_new_with_data")
public func pdf_document_new_with_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outDocument: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let bytes, len > 0, let outDocument else {
            throw PDFBridgeError.invalidArgument("missing PDF bytes or output pointer")
        }
        let data = Data(bytes: bytes, count: len)
        guard let document = PDFDocument(data: data) else {
            throw PDFBridgeError.nullResult("PDFDocument(data:) returned nil")
        }
        outDocument.pointee = pdf_retain(document)
    }
}

@_cdecl("pdf_document_info_json")
public func pdf_document_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let document = pdf_borrow_object(handle) as? PDFDocument else { return nil }
    return pdf_string(pdf_json_string(from: pdf_document_info(document)) ?? "{}")
}

@_cdecl("pdf_document_attributes_json")
public func pdf_document_attributes_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let document = pdf_borrow_object(handle) as? PDFDocument else { return nil }
    return pdf_string(pdf_json_string(from: pdf_document_attributes(document)) ?? "{}")
}

@_cdecl("pdf_document_string")
public func pdf_document_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let document = pdf_borrow_object(handle) as? PDFDocument,
          let string = document.string
    else {
        return nil
    }
    return pdf_string(string)
}

@_cdecl("pdf_document_page_count")
public func pdf_document_page_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let document = pdf_borrow_object(handle) as? PDFDocument else { return 0 }
    return UInt64(document.pageCount)
}

@_cdecl("pdf_document_page_at")
public func pdf_document_page_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let document = pdf_borrow_object(handle) as? PDFDocument,
          index < UInt64(document.pageCount),
          let page = document.page(at: Int(index))
    else {
        return nil
    }
    return pdf_retain(page)
}

@_cdecl("pdf_document_outline_root")
public func pdf_document_outline_root(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let document = pdf_borrow_object(handle) as? PDFDocument,
          let outline = document.outlineRoot
    else {
        return nil
    }
    return pdf_retain(outline)
}

@_cdecl("pdf_document_unlock")
public func pdf_document_unlock(
    _ handle: UnsafeMutableRawPointer?,
    _ password: UnsafePointer<CChar>?
) -> Int32 {
    guard let document = pdf_borrow_object(handle) as? PDFDocument,
          let password
    else {
        return 0
    }
    return document.unlock(withPassword: String(cString: password)) ? 1 : 0
}

@_cdecl("pdf_document_write_to_url")
public func pdf_document_write_to_url(
    _ handle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let document = pdf_borrow_object(handle) as? PDFDocument,
              let path
        else {
            throw PDFBridgeError.invalidArgument("missing document handle or output path")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard document.write(to: url) else {
            throw PDFBridgeError.nullResult("PDFDocument.write(to:) returned false")
        }
    }
}

@_cdecl("pdf_page_label_string")
public func pdf_page_label_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let page = pdf_borrow_object(handle) as? PDFPage,
          let label = page.label
    else {
        return nil
    }
    return pdf_string(label)
}

@_cdecl("pdf_page_string")
public func pdf_page_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let page = pdf_borrow_object(handle) as? PDFPage,
          let string = page.string
    else {
        return nil
    }
    return pdf_string(string)
}

@_cdecl("pdf_page_number_of_characters")
public func pdf_page_number_of_characters(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let page = pdf_borrow_object(handle) as? PDFPage else { return 0 }
    return UInt64(page.numberOfCharacters)
}

@_cdecl("pdf_page_rotation")
public func pdf_page_rotation(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let page = pdf_borrow_object(handle) as? PDFPage else { return 0 }
    return Int32(page.rotation)
}

@_cdecl("pdf_page_bounds")
public func pdf_page_bounds(
    _ handle: UnsafeMutableRawPointer?,
    _ displayBoxRaw: Int32,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let page = pdf_borrow_object(handle) as? PDFPage,
          let displayBox = try? pdf_display_box(displayBoxRaw)
    else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let bounds = page.bounds(for: displayBox)
    outX?.pointee = bounds.origin.x
    outY?.pointee = bounds.origin.y
    outWidth?.pointee = bounds.size.width
    outHeight?.pointee = bounds.size.height
}

@_cdecl("pdf_page_annotation_count")
public func pdf_page_annotation_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let page = pdf_borrow_object(handle) as? PDFPage else { return 0 }
    return UInt64(page.annotations.count)
}

@_cdecl("pdf_page_annotation_at")
public func pdf_page_annotation_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_borrow_object(handle) as? PDFPage,
          index < UInt64(page.annotations.count)
    else {
        return nil
    }
    return pdf_retain(page.annotations[Int(index)])
}

@_cdecl("pdf_page_selection_for_range")
public func pdf_page_selection_for_range(
    _ handle: UnsafeMutableRawPointer?,
    _ location: UInt64,
    _ length: UInt64
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_borrow_object(handle) as? PDFPage else { return nil }
    guard let selection = page.selection(for: NSRange(location: Int(location), length: Int(length))) else {
        return nil
    }
    return pdf_retain(selection)
}

@_cdecl("pdf_selection_string")
public func pdf_selection_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let selection = pdf_borrow_object(handle) as? PDFSelection,
          let string = selection.string
    else {
        return nil
    }
    return pdf_string(string)
}

@_cdecl("pdf_selection_page_count")
public func pdf_selection_page_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let selection = pdf_borrow_object(handle) as? PDFSelection else { return 0 }
    return UInt64(selection.pages.count)
}

@_cdecl("pdf_selection_page_at")
public func pdf_selection_page_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let selection = pdf_borrow_object(handle) as? PDFSelection,
          index < UInt64(selection.pages.count)
    else {
        return nil
    }
    return pdf_retain(selection.pages[Int(index)])
}

@_cdecl("pdf_selection_bounds_for_page")
public func pdf_selection_bounds_for_page(
    _ selectionHandle: UnsafeMutableRawPointer?,
    _ pageHandle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let selection = pdf_borrow_object(selectionHandle) as? PDFSelection,
          let page = pdf_borrow_object(pageHandle) as? PDFPage
    else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let bounds = selection.bounds(for: page)
    outX?.pointee = bounds.origin.x
    outY?.pointee = bounds.origin.y
    outWidth?.pointee = bounds.size.width
    outHeight?.pointee = bounds.size.height
}

@_cdecl("pdf_outline_label_string")
public func pdf_outline_label_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let outline = pdf_borrow_object(handle) as? PDFOutline,
          let label = outline.label
    else {
        return nil
    }
    return pdf_string(label)
}

@_cdecl("pdf_outline_child_count")
public func pdf_outline_child_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let outline = pdf_borrow_object(handle) as? PDFOutline else { return 0 }
    return UInt64(outline.numberOfChildren)
}

@_cdecl("pdf_outline_child_at")
public func pdf_outline_child_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let outline = pdf_borrow_object(handle) as? PDFOutline,
          index < UInt64(outline.numberOfChildren),
          let child = outline.child(at: Int(index))
    else {
        return nil
    }
    return pdf_retain(child)
}

@_cdecl("pdf_annotation_info_json")
public func pdf_annotation_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let annotation = pdf_borrow_object(handle) as? PDFAnnotation else { return nil }
    return pdf_string(pdf_json_string(from: pdf_annotation_info(annotation)) ?? "{}")
}
