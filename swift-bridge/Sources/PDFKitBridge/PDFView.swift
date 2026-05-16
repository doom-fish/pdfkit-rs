import Foundation
import PDFKit

private func pdf_view_info(_ view: PDFView) -> [String: Any] {
    let inMarkupMode: Bool
    if #available(macOS 13.0, *) {
        inMarkupMode = view.isInMarkupMode
    } else {
        inMarkupMode = false
    }

    return [
        "display_mode": view.displayMode.rawValue,
        "display_direction": view.displayDirection.rawValue,
        "displays_page_breaks": view.displaysPageBreaks,
        "page_break_margins": pdf_insets_dict(view.pageBreakMargins),
        "display_box": view.displayBox.rawValue,
        "displays_as_book": view.displaysAsBook,
        "displays_rtl": view.displaysRTL,
        "background_color": pdf_color_dict(view.backgroundColor) as Any,
        "interpolation_quality": view.interpolationQuality.rawValue,
        "page_shadows_enabled": view.pageShadowsEnabled,
        "scale_factor": view.scaleFactor,
        "min_scale_factor": view.minScaleFactor,
        "max_scale_factor": view.maxScaleFactor,
        "auto_scales": view.autoScales,
        "scale_factor_for_size_to_fit": view.scaleFactorForSizeToFit,
        "in_markup_mode": inMarkupMode,
        "has_document_view": view.documentView != nil,
        "visible_page_count": view.visiblePages.count,
        "current_page_label": view.currentPage?.label as Any,
    ]
}

@_cdecl("pdf_view_new")
public func pdf_view_new(
    _ width: Double,
    _ height: Double,
    _ outView: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outView else {
            throw PDFBridgeError.invalidArgument("missing view output pointer")
        }
        let view = PDFView(frame: CGRect(x: 0, y: 0, width: width, height: height))
        outView.pointee = pdf_retain_view(view)
    }
}

@_cdecl("pdf_view_info_json")
public func pdf_view_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let view = pdf_view_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: pdf_view_info(view)) ?? "{}")
}

@_cdecl("pdf_view_document")
public func pdf_view_document(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view = pdf_view_value(handle), let document = view.document else {
        return nil
    }
    return pdf_retain_document(document)
}

@_cdecl("pdf_view_set_document")
public func pdf_view_set_document(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ documentHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(viewHandle) else {
            throw PDFBridgeError.invalidArgument("missing view handle")
        }
        view.document = pdf_document_value(documentHandle)
    }
}

@_cdecl("pdf_view_current_page")
public func pdf_view_current_page(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view = pdf_view_value(handle), let page = view.currentPage else {
        return nil
    }
    return pdf_retain_page(page)
}

@_cdecl("pdf_view_current_destination")
public func pdf_view_current_destination(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view = pdf_view_value(handle), let destination = view.currentDestination else {
        return nil
    }
    return pdf_retain_destination(destination)
}

@_cdecl("pdf_view_current_selection")
public func pdf_view_current_selection(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view = pdf_view_value(handle), let selection = view.currentSelection else {
        return nil
    }
    return pdf_retain_selection(selection)
}

@_cdecl("pdf_view_set_current_selection")
public func pdf_view_set_current_selection(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ selectionHandle: UnsafeMutableRawPointer?,
    _ animate: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(viewHandle) else {
            throw PDFBridgeError.invalidArgument("missing view handle")
        }
        view.setCurrentSelection(pdf_selection_value(selectionHandle), animate: animate != 0)
    }
}

@_cdecl("pdf_view_clear_selection")
public func pdf_view_clear_selection(_ handle: UnsafeMutableRawPointer?) {
    guard let view = pdf_view_value(handle) else { return }
    view.clearSelection()
}

@_cdecl("pdf_view_go_to_page")
public func pdf_view_go_to_page(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ pageHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(viewHandle), let page = pdf_page_value(pageHandle) else {
            throw PDFBridgeError.invalidArgument("missing view or page handle")
        }
        view.go(to: page)
    }
}

@_cdecl("pdf_view_go_to_destination")
public func pdf_view_go_to_destination(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ destinationHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(viewHandle), let destination = pdf_destination_value(destinationHandle) else {
            throw PDFBridgeError.invalidArgument("missing view or destination handle")
        }
        view.go(to: destination)
    }
}

@_cdecl("pdf_view_go_to_selection")
public func pdf_view_go_to_selection(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ selectionHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(viewHandle), let selection = pdf_selection_value(selectionHandle) else {
            throw PDFBridgeError.invalidArgument("missing view or selection handle")
        }
        view.go(to: selection)
    }
}

@_cdecl("pdf_view_set_display_mode")
public func pdf_view_set_display_mode(
    _ handle: UnsafeMutableRawPointer?,
    _ rawValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing view handle")
        }
        view.displayMode = try pdf_display_mode(rawValue)
    }
}

@_cdecl("pdf_view_set_display_direction")
public func pdf_view_set_display_direction(
    _ handle: UnsafeMutableRawPointer?,
    _ rawValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing view handle")
        }
        view.displayDirection = try pdf_display_direction(rawValue)
    }
}

@_cdecl("pdf_view_set_display_box")
public func pdf_view_set_display_box(
    _ handle: UnsafeMutableRawPointer?,
    _ rawValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let view = pdf_view_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing view handle")
        }
        view.displayBox = try pdf_display_box(rawValue)
    }
}

@_cdecl("pdf_view_set_auto_scales")
public func pdf_view_set_auto_scales(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let view = pdf_view_value(handle) else { return }
    view.autoScales = value != 0
}

@_cdecl("pdf_view_set_scale_factor")
public func pdf_view_set_scale_factor(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let view = pdf_view_value(handle) else { return }
    view.scaleFactor = CGFloat(value)
}

@_cdecl("pdf_view_set_min_scale_factor")
public func pdf_view_set_min_scale_factor(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let view = pdf_view_value(handle) else { return }
    view.minScaleFactor = CGFloat(value)
}

@_cdecl("pdf_view_set_max_scale_factor")
public func pdf_view_set_max_scale_factor(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let view = pdf_view_value(handle) else { return }
    view.maxScaleFactor = CGFloat(value)
}

@_cdecl("pdf_view_layout_document_view")
public func pdf_view_layout_document_view(_ handle: UnsafeMutableRawPointer?) {
    guard let view = pdf_view_value(handle) else { return }
    view.layoutDocumentView()
}

@_cdecl("pdf_view_visible_page_count")
public func pdf_view_visible_page_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let view = pdf_view_value(handle) else { return 0 }
    return UInt64(view.visiblePages.count)
}

@_cdecl("pdf_view_visible_page_at")
public func pdf_view_visible_page_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let view = pdf_view_value(handle), index < UInt64(view.visiblePages.count) else {
        return nil
    }
    return pdf_retain_page(view.visiblePages[Int(index)])
}
