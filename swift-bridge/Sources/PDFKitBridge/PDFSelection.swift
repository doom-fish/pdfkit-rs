import Foundation
import PDFKit

@_cdecl("pdf_selection_new")
public func pdf_selection_new(
    _ documentHandle: UnsafeMutableRawPointer?,
    _ outSelection: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let document = pdf_document_value(documentHandle), let outSelection else {
            throw PDFBridgeError.invalidArgument("missing document handle or selection output pointer")
        }
        outSelection.pointee = pdf_retain_selection(PDFSelection(document: document))
    }
}

@_cdecl("pdf_selection_string")
public func pdf_selection_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let selection = pdf_selection_value(handle), let string = selection.string else {
        return nil
    }
    return pdf_string(string)
}

@_cdecl("pdf_selection_page_count")
public func pdf_selection_page_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let selection = pdf_selection_value(handle) else { return 0 }
    return UInt64(selection.pages.count)
}

@_cdecl("pdf_selection_page_at")
public func pdf_selection_page_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let selection = pdf_selection_value(handle), index < UInt64(selection.pages.count) else {
        return nil
    }
    return pdf_retain_page(selection.pages[Int(index)])
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
    guard let selection = pdf_selection_value(selectionHandle), let page = pdf_page_value(pageHandle) else {
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

@_cdecl("pdf_selection_number_of_text_ranges_on_page")
public func pdf_selection_number_of_text_ranges_on_page(
    _ selectionHandle: UnsafeMutableRawPointer?,
    _ pageHandle: UnsafeMutableRawPointer?
) -> UInt64 {
    guard let selection = pdf_selection_value(selectionHandle), let page = pdf_page_value(pageHandle) else {
        return 0
    }
    return UInt64(selection.numberOfTextRanges(on: page))
}

@_cdecl("pdf_selection_range_at_index_on_page")
public func pdf_selection_range_at_index_on_page(
    _ selectionHandle: UnsafeMutableRawPointer?,
    _ index: UInt64,
    _ pageHandle: UnsafeMutableRawPointer?,
    _ outLocation: UnsafeMutablePointer<UInt64>?,
    _ outLength: UnsafeMutablePointer<UInt64>?
) -> Int32 {
    guard let selection = pdf_selection_value(selectionHandle), let page = pdf_page_value(pageHandle) else {
        outLocation?.pointee = 0
        outLength?.pointee = 0
        return 0
    }
    let rangeCount = selection.numberOfTextRanges(on: page)
    guard index < UInt64(rangeCount) else {
        outLocation?.pointee = 0
        outLength?.pointee = 0
        return 0
    }
    let range = selection.range(at: Int(index), on: page)
    outLocation?.pointee = UInt64(range.location)
    outLength?.pointee = UInt64(range.length)
    return 1
}

@_cdecl("pdf_selection_selections_by_line_count")
public func pdf_selection_selections_by_line_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let selection = pdf_selection_value(handle) else { return 0 }
    return UInt64(selection.selectionsByLine().count)
}

@_cdecl("pdf_selection_selection_by_line_at")
public func pdf_selection_selection_by_line_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let selection = pdf_selection_value(handle) else { return nil }
    let lines = selection.selectionsByLine()
    guard index < UInt64(lines.count) else { return nil }
    return pdf_retain_selection(lines[Int(index)])
}

@_cdecl("pdf_selection_add_selection")
public func pdf_selection_add_selection(
    _ selectionHandle: UnsafeMutableRawPointer?,
    _ otherHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let selection = pdf_selection_value(selectionHandle), let other = pdf_selection_value(otherHandle) else {
            throw PDFBridgeError.invalidArgument("missing selection handle")
        }
        selection.add(other)
    }
}

@_cdecl("pdf_selection_extend_at_end")
public func pdf_selection_extend_at_end(_ handle: UnsafeMutableRawPointer?, _ amount: Int64) {
    guard let selection = pdf_selection_value(handle) else { return }
    selection.extend(atEnd: Int(amount))
}

@_cdecl("pdf_selection_extend_at_start")
public func pdf_selection_extend_at_start(_ handle: UnsafeMutableRawPointer?, _ amount: Int64) {
    guard let selection = pdf_selection_value(handle) else { return }
    selection.extend(atStart: Int(amount))
}

@_cdecl("pdf_selection_extend_for_line_boundaries")
public func pdf_selection_extend_for_line_boundaries(_ handle: UnsafeMutableRawPointer?) {
    guard let selection = pdf_selection_value(handle) else { return }
    selection.extendForLineBoundaries()
}
