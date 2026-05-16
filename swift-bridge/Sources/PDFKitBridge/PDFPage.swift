import Foundation
import PDFKit

@_cdecl("pdf_page_new")
public func pdf_page_new(
    _ outPage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outPage else {
            throw PDFBridgeError.invalidArgument("missing page output pointer")
        }
        outPage.pointee = pdf_retain_page(PDFPage())
    }
}

@_cdecl("pdf_page_label_string")
public func pdf_page_label_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let page = pdf_page_value(handle), let label = page.label else {
        return nil
    }
    return pdf_string(label)
}

@_cdecl("pdf_page_string")
public func pdf_page_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let page = pdf_page_value(handle), let string = page.string else {
        return nil
    }
    return pdf_string(string)
}

@_cdecl("pdf_page_number_of_characters")
public func pdf_page_number_of_characters(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let page = pdf_page_value(handle) else { return 0 }
    return UInt64(page.numberOfCharacters)
}

@_cdecl("pdf_page_rotation")
public func pdf_page_rotation(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let page = pdf_page_value(handle) else { return 0 }
    return Int32(page.rotation)
}

@_cdecl("pdf_page_set_rotation")
public func pdf_page_set_rotation(
    _ handle: UnsafeMutableRawPointer?,
    _ rotation: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let page = pdf_page_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing page handle")
        }
        page.rotation = Int(rotation)
    }
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
    guard let page = pdf_page_value(handle), let displayBox = try? pdf_display_box(displayBoxRaw) else {
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

@_cdecl("pdf_page_set_bounds")
public func pdf_page_set_bounds(
    _ handle: UnsafeMutableRawPointer?,
    _ displayBoxRaw: Int32,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let page = pdf_page_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing page handle")
        }
        let displayBox = try pdf_display_box(displayBoxRaw)
        page.setBounds(CGRect(x: x, y: y, width: width, height: height), for: displayBox)
    }
}

@_cdecl("pdf_page_document")
public func pdf_page_document(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(handle), let document = page.document else {
        return nil
    }
    return pdf_retain_document(document)
}

@_cdecl("pdf_page_annotation_count")
public func pdf_page_annotation_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let page = pdf_page_value(handle) else { return 0 }
    return UInt64(page.annotations.count)
}

@_cdecl("pdf_page_annotation_at")
public func pdf_page_annotation_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(handle), index < UInt64(page.annotations.count) else {
        return nil
    }
    return pdf_retain_annotation(page.annotations[Int(index)])
}

@_cdecl("pdf_page_add_annotation")
public func pdf_page_add_annotation(
    _ pageHandle: UnsafeMutableRawPointer?,
    _ annotationHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let page = pdf_page_value(pageHandle), let annotation = pdf_annotation_value(annotationHandle) else {
            throw PDFBridgeError.invalidArgument("missing page or annotation handle")
        }
        page.addAnnotation(annotation)
    }
}

@_cdecl("pdf_page_remove_annotation")
public func pdf_page_remove_annotation(
    _ pageHandle: UnsafeMutableRawPointer?,
    _ annotationHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let page = pdf_page_value(pageHandle), let annotation = pdf_annotation_value(annotationHandle) else {
            throw PDFBridgeError.invalidArgument("missing page or annotation handle")
        }
        page.removeAnnotation(annotation)
    }
}

@_cdecl("pdf_page_annotation_at_point")
public func pdf_page_annotation_at_point(
    _ pageHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(pageHandle), let annotation = page.annotation(at: CGPoint(x: x, y: y)) else {
        return nil
    }
    return pdf_retain_annotation(annotation)
}

@_cdecl("pdf_page_selection_for_range")
public func pdf_page_selection_for_range(
    _ handle: UnsafeMutableRawPointer?,
    _ location: UInt64,
    _ length: UInt64
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(handle),
          let selection = page.selection(for: NSRange(location: Int(location), length: Int(length)))
    else {
        return nil
    }
    return pdf_retain_selection(selection)
}

@_cdecl("pdf_page_selection_for_rect")
public func pdf_page_selection_for_rect(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(handle),
          let selection = page.selection(for: CGRect(x: x, y: y, width: width, height: height))
    else {
        return nil
    }
    return pdf_retain_selection(selection)
}

@_cdecl("pdf_page_selection_for_word_at_point")
public func pdf_page_selection_for_word_at_point(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(handle),
          let selection = page.selectionForWord(at: CGPoint(x: x, y: y))
    else {
        return nil
    }
    return pdf_retain_selection(selection)
}

@_cdecl("pdf_page_selection_for_line_at_point")
public func pdf_page_selection_for_line_at_point(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(handle),
          let selection = page.selectionForLine(at: CGPoint(x: x, y: y))
    else {
        return nil
    }
    return pdf_retain_selection(selection)
}

@_cdecl("pdf_page_selection_from_points")
public func pdf_page_selection_from_points(
    _ handle: UnsafeMutableRawPointer?,
    _ startX: Double,
    _ startY: Double,
    _ endX: Double,
    _ endY: Double
) -> UnsafeMutableRawPointer? {
    guard let page = pdf_page_value(handle),
          let selection = page.selection(from: CGPoint(x: startX, y: startY), to: CGPoint(x: endX, y: endY))
    else {
        return nil
    }
    return pdf_retain_selection(selection)
}

@_cdecl("pdf_page_character_bounds_at")
public func pdf_page_character_bounds_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let page = pdf_page_value(handle), index < UInt64(page.numberOfCharacters) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let bounds = page.characterBounds(at: Int(index))
    outX?.pointee = bounds.origin.x
    outY?.pointee = bounds.origin.y
    outWidth?.pointee = bounds.size.width
    outHeight?.pointee = bounds.size.height
}

@_cdecl("pdf_page_character_index_at_point")
public func pdf_page_character_index_at_point(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double
) -> Int64 {
    guard let page = pdf_page_value(handle) else { return Int64.max }
    return Int64(page.characterIndex(at: CGPoint(x: x, y: y)))
}

@_cdecl("pdf_page_displays_annotations")
public func pdf_page_displays_annotations(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let page = pdf_page_value(handle) else { return 0 }
    return page.displaysAnnotations ? 1 : 0
}

@_cdecl("pdf_page_set_displays_annotations")
public func pdf_page_set_displays_annotations(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let page = pdf_page_value(handle) else { return }
    page.displaysAnnotations = value != 0
}
