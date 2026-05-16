import Foundation
import PDFKit

private func pdf_document_info(_ document: PDFDocument) -> [String: Any] {
    [
        "document_url": document.documentURL?.path as Any,
        "major_version": document.majorVersion,
        "minor_version": document.minorVersion,
        "is_encrypted": document.isEncrypted,
        "is_locked": document.isLocked,
        "permissions_status": document.permissionsStatus.rawValue,
        "access_permissions": document.accessPermissions.rawValue,
        "allows_printing": document.allowsPrinting,
        "allows_copying": document.allowsCopying,
        "allows_document_changes": document.allowsDocumentChanges,
        "allows_document_assembly": document.allowsDocumentAssembly,
        "allows_content_accessibility": document.allowsContentAccessibility,
        "allows_commenting": document.allowsCommenting,
        "allows_form_field_entry": document.allowsFormFieldEntry,
        "page_class": NSStringFromClass(document.pageClass),
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

@_cdecl("pdf_document_new")
public func pdf_document_new(
    _ outDocument: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outDocument else {
            throw PDFBridgeError.invalidArgument("missing document output pointer")
        }
        outDocument.pointee = pdf_retain_document(PDFDocument())
    }
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
        outDocument.pointee = pdf_retain_document(document)
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
        outDocument.pointee = pdf_retain_document(document)
    }
}

@_cdecl("pdf_document_info_json")
public func pdf_document_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let document = pdf_document_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: pdf_document_info(document)) ?? "{}")
}

@_cdecl("pdf_document_attributes_json")
public func pdf_document_attributes_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let document = pdf_document_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: pdf_document_attributes(document)) ?? "{}")
}

@_cdecl("pdf_document_string")
public func pdf_document_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let document = pdf_document_value(handle), let string = document.string else {
        return nil
    }
    return pdf_string(string)
}

@_cdecl("pdf_document_page_count")
public func pdf_document_page_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let document = pdf_document_value(handle) else { return 0 }
    return UInt64(document.pageCount)
}

@_cdecl("pdf_document_page_at")
public func pdf_document_page_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let document = pdf_document_value(handle),
          index < UInt64(document.pageCount),
          let page = document.page(at: Int(index))
    else {
        return nil
    }
    return pdf_retain_page(page)
}

@_cdecl("pdf_document_index_for_page")
public func pdf_document_index_for_page(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ pageHandle: UnsafeMutableRawPointer?
) -> UInt64 {
    guard let document = pdf_document_value(documentHandle), let page = pdf_page_value(pageHandle) else {
        return UInt64.max
    }
    let index = document.index(for: page)
    return index == NSNotFound ? UInt64.max : UInt64(index)
}

@_cdecl("pdf_document_outline_root")
public func pdf_document_outline_root(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let document = pdf_document_value(handle), let outline = document.outlineRoot else {
        return nil
    }
    return pdf_retain_outline(outline)
}

@_cdecl("pdf_document_set_outline_root")
public func pdf_document_set_outline_root(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ outlineHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let document = pdf_document_value(documentHandle) else {
            throw PDFBridgeError.invalidArgument("missing document handle")
        }
        document.outlineRoot = pdf_outline_value(outlineHandle)
    }
}

@_cdecl("pdf_document_outline_item_for_selection")
public func pdf_document_outline_item_for_selection(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ selectionHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let document = pdf_document_value(documentHandle),
          let selection = pdf_selection_value(selectionHandle),
          let outline = document.outlineItem(for: selection)
    else {
        return nil
    }
    return pdf_retain_outline(outline)
}

@_cdecl("pdf_document_selection_for_entire_document")
public func pdf_document_selection_for_entire_document(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let document = pdf_document_value(handle), let selection = document.selectionForEntireDocument else {
        return nil
    }
    return pdf_retain_selection(selection)
}

@_cdecl("pdf_document_selection_from_pages_points")
public func pdf_document_selection_from_pages_points(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ startPageHandle: UnsafeMutableRawPointer?,
    _ startX: Double,
    _ startY: Double,
    _ endPageHandle: UnsafeMutableRawPointer?,
    _ endX: Double,
    _ endY: Double
) -> UnsafeMutableRawPointer? {
    guard let document = pdf_document_value(documentHandle),
          let startPage = pdf_page_value(startPageHandle),
          let endPage = pdf_page_value(endPageHandle)
    else {
        return nil
    }
    guard startPage.document === document, endPage.document === document else { return nil }
    let selection = document.selection(
        from: startPage,
        at: CGPoint(x: startX, y: startY),
        to: endPage,
        at: CGPoint(x: endX, y: endY)
    )
    return selection.map(pdf_retain_selection)
}

@_cdecl("pdf_document_selection_from_pages_characters")
public func pdf_document_selection_from_pages_characters(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ startPageHandle: UnsafeMutableRawPointer?,
    _ startCharacter: UInt64,
    _ endPageHandle: UnsafeMutableRawPointer?,
    _ endCharacter: UInt64
) -> UnsafeMutableRawPointer? {
    guard let document = pdf_document_value(documentHandle),
          let startPage = pdf_page_value(startPageHandle),
          let endPage = pdf_page_value(endPageHandle)
    else {
        return nil
    }
    guard startPage.document === document, endPage.document === document else { return nil }
    let selection = document.selection(
        from: startPage,
        atCharacterIndex: Int(startCharacter),
        to: endPage,
        atCharacterIndex: Int(endCharacter)
    )
    return selection.map(pdf_retain_selection)
}

@_cdecl("pdf_document_unlock")
public func pdf_document_unlock(
    _ handle: UnsafeMutableRawPointer?,
    _ password: UnsafePointer<CChar>?
) -> Int32 {
    guard let document = pdf_document_value(handle), let password else {
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
        guard let document = pdf_document_value(handle), let path else {
            throw PDFBridgeError.invalidArgument("missing document handle or output path")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard document.write(to: url) else {
            throw PDFBridgeError.nullResult("PDFDocument.write(to:) returned false")
        }
    }
}

@_cdecl("pdf_document_insert_page")
public func pdf_document_insert_page(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ pageHandle: UnsafeMutableRawPointer?,
    _ index: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let document = pdf_document_value(documentHandle), let page = pdf_page_value(pageHandle) else {
            throw PDFBridgeError.invalidArgument("missing document or page handle")
        }
        guard index <= UInt64(document.pageCount) else {
            throw PDFBridgeError.invalidArgument("page index out of range")
        }
        document.insert(page, at: Int(index))
    }
}

@_cdecl("pdf_document_remove_page_at")
public func pdf_document_remove_page_at(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ index: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let document = pdf_document_value(documentHandle) else {
            throw PDFBridgeError.invalidArgument("missing document handle")
        }
        guard index < UInt64(document.pageCount) else {
            throw PDFBridgeError.invalidArgument("page index out of range")
        }
        document.removePage(at: Int(index))
    }
}

@_cdecl("pdf_document_exchange_pages")
public func pdf_document_exchange_pages(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ indexA: UInt64,
    _ indexB: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let document = pdf_document_value(documentHandle) else {
            throw PDFBridgeError.invalidArgument("missing document handle")
        }
        guard indexA < UInt64(document.pageCount), indexB < UInt64(document.pageCount) else {
            throw PDFBridgeError.invalidArgument("page index out of range")
        }
        guard let pageA = document.page(at: Int(indexA)),
              let pageB = document.page(at: Int(indexB)),
              pageA.pageRef != nil,
              pageB.pageRef != nil
        else {
            throw PDFBridgeError.invalidArgument("PDFDocument.exchangePage is unsupported for synthetic in-memory pages")
        }
        document.exchangePage(at: Int(indexA), withPageAt: Int(indexB))
    }
}
