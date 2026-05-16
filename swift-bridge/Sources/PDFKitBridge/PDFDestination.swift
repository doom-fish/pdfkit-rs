import Foundation
import PDFKit

private func pdf_destination_info(_ destination: PDFDestination) -> [String: Any] {
    let page = destination.page
    let pageIndex = page.flatMap { page -> Int? in
        guard let document = page.document else { return nil }
        let index = document.index(for: page)
        return index == NSNotFound ? nil : index
    }
    return [
        "page_label": page?.label as Any,
        "page_index": pageIndex as Any,
        "point": pdf_point_dict(destination.point),
        "zoom": destination.zoom,
    ]
}

@_cdecl("pdf_destination_new")
public func pdf_destination_new(
    _ pageHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ outDestination: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let page = pdf_page_value(pageHandle), let outDestination else {
            throw PDFBridgeError.invalidArgument("missing page handle or destination output pointer")
        }
        let destination = PDFDestination(page: page, at: CGPoint(x: x, y: y))
        outDestination.pointee = pdf_retain_destination(destination)
    }
}

@_cdecl("pdf_destination_info_json")
public func pdf_destination_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let destination = pdf_destination_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: pdf_destination_info(destination)) ?? "{}")
}

@_cdecl("pdf_destination_page")
public func pdf_destination_page(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let destination = pdf_destination_value(handle), let page = destination.page else {
        return nil
    }
    return pdf_retain_page(page)
}

@_cdecl("pdf_destination_set_zoom")
public func pdf_destination_set_zoom(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let destination = pdf_destination_value(handle) else { return }
    destination.zoom = CGFloat(value)
}

@_cdecl("pdf_destination_compare")
public func pdf_destination_compare(
    _ lhsHandle: UnsafeMutableRawPointer?,
    _ rhsHandle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let lhs = pdf_destination_value(lhsHandle), let rhs = pdf_destination_value(rhsHandle) else {
        return 0
    }
    guard let lhsPage = lhs.page,
          let rhsPage = rhs.page,
          let lhsDocument = lhsPage.document,
          let rhsDocument = rhsPage.document,
          lhsDocument === rhsDocument
    else {
        return 0
    }
    return Int32(lhs.compare(rhs).rawValue)
}
