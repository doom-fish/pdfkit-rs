import Foundation
import PDFKit

private func pdf_thumbnail_view_info(_ view: PDFThumbnailView) -> [String: Any] {
    [
        "has_pdf_view": view.pdfView != nil,
        "background_color": pdf_color_dict(view.backgroundColor) as Any,
        "selected_pages_count": view.selectedPages?.count ?? 0,
        "thumbnail_size": pdf_size_dict(view.thumbnailSize),
        "maximum_number_of_columns": view.maximumNumberOfColumns,
        "allows_dragging": view.allowsDragging,
        "allows_multiple_selection": view.allowsMultipleSelection,
    ]
}

@_cdecl("pdf_thumbnail_view_new")
public func pdf_thumbnail_view_new(
    _ width: Double,
    _ height: Double,
    _ outView: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outView else {
            throw PDFBridgeError.invalidArgument("missing thumbnail view output pointer")
        }
        let view = PDFThumbnailView(frame: CGRect(x: 0, y: 0, width: width, height: height))
        outView.pointee = pdf_retain_thumbnail_view(view)
    }
}

@_cdecl("pdf_thumbnail_view_info_json")
public func pdf_thumbnail_view_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let view = pdf_thumbnail_view_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: pdf_thumbnail_view_info(view)) ?? "{}")
}

@_cdecl("pdf_thumbnail_view_pdf_view")
public func pdf_thumbnail_view_pdf_view(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view = pdf_thumbnail_view_value(handle), let pdfView = view.pdfView else {
        return nil
    }
    return pdf_retain_view(pdfView)
}

@_cdecl("pdf_thumbnail_view_set_pdf_view")
public func pdf_thumbnail_view_set_pdf_view(
    _ handle: UnsafeMutableRawPointer?,
    _ pdfViewHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_thumbnail_view_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing thumbnail view handle")
        }
        view.pdfView = pdf_view_value(pdfViewHandle)
    }
}

@_cdecl("pdf_thumbnail_view_set_thumbnail_size")
public func pdf_thumbnail_view_set_thumbnail_size(_ handle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let view = pdf_thumbnail_view_value(handle) else { return }
    view.thumbnailSize = CGSize(width: width, height: height)
}

@_cdecl("pdf_thumbnail_view_set_maximum_number_of_columns")
public func pdf_thumbnail_view_set_maximum_number_of_columns(_ handle: UnsafeMutableRawPointer?, _ value: UInt64) {
    guard let view = pdf_thumbnail_view_value(handle) else { return }
    view.maximumNumberOfColumns = Int(value)
}

@_cdecl("pdf_thumbnail_view_set_allows_dragging")
public func pdf_thumbnail_view_set_allows_dragging(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let view = pdf_thumbnail_view_value(handle) else { return }
    view.allowsDragging = value != 0
}

@_cdecl("pdf_thumbnail_view_set_allows_multiple_selection")
public func pdf_thumbnail_view_set_allows_multiple_selection(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let view = pdf_thumbnail_view_value(handle) else { return }
    view.allowsMultipleSelection = value != 0
}

@_cdecl("pdf_thumbnail_view_selected_page_count")
public func pdf_thumbnail_view_selected_page_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let view = pdf_thumbnail_view_value(handle) else { return 0 }
    return UInt64(view.selectedPages?.count ?? 0)
}

@_cdecl("pdf_thumbnail_view_selected_page_at")
public func pdf_thumbnail_view_selected_page_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let view = pdf_thumbnail_view_value(handle), let selectedPages = view.selectedPages, index < UInt64(selectedPages.count) else {
        return nil
    }
    return pdf_retain_page(selectedPages[Int(index)])
}
