import AppKit
import CoreGraphics
import Foundation
import PDFKit

public let PDFX_OK: Int32 = 0
public let PDFX_INVALID_ARGUMENT: Int32 = -1
public let PDFX_NULL_RESULT: Int32 = -2
public let PDFX_FRAMEWORK: Int32 = -3

final class PDFDocumentBox { let value: PDFDocument; init(_ value: PDFDocument) { self.value = value } }
final class PDFPageBox { let value: PDFPage; init(_ value: PDFPage) { self.value = value } }
final class PDFAnnotationBox { let value: PDFAnnotation; init(_ value: PDFAnnotation) { self.value = value } }
final class PDFOutlineBox { let value: PDFOutline; init(_ value: PDFOutline) { self.value = value } }
final class PDFSelectionBox { let value: PDFSelection; init(_ value: PDFSelection) { self.value = value } }
final class PDFViewBox { let value: PDFView; init(_ value: PDFView) { self.value = value } }
final class PDFThumbnailViewBox { let value: PDFThumbnailView; init(_ value: PDFThumbnailView) { self.value = value } }
final class PDFActionBox { let value: PDFAction; init(_ value: PDFAction) { self.value = value } }
final class PDFActionURLBox { let value: PDFActionURL; init(_ value: PDFActionURL) { self.value = value } }
final class PDFActionGoToBox { let value: PDFActionGoTo; init(_ value: PDFActionGoTo) { self.value = value } }
final class PDFActionNamedBox { let value: PDFActionNamed; init(_ value: PDFActionNamed) { self.value = value } }
final class PDFActionRemoteGoToBox {
    let value: PDFActionRemoteGoTo
    init(_ value: PDFActionRemoteGoTo) { self.value = value }
}
final class PDFActionResetFormBox {
    let value: PDFActionResetForm
    init(_ value: PDFActionResetForm) { self.value = value }
}
final class PDFBorderBox { let value: PDFBorder; init(_ value: PDFBorder) { self.value = value } }
final class PDFDestinationBox { let value: PDFDestination; init(_ value: PDFDestination) { self.value = value } }
final class PDFPageOverlayViewBox {
    let value: NSView
    init(_ value: NSView) { self.value = value }
}
final class PDFAppearanceCharacteristicsBox {
    let value: PDFAppearanceCharacteristics
    init(_ value: PDFAppearanceCharacteristics) { self.value = value }
}
final class PDFRustDocumentDelegateBox {
    let value: PDFRustDocumentDelegate
    init(_ value: PDFRustDocumentDelegate) { self.value = value }
}
final class PDFRustViewDelegateBox {
    let value: PDFRustViewDelegate
    init(_ value: PDFRustViewDelegate) { self.value = value }
}
final class PDFRustPageOverlayViewProviderBox {
    let value: PDFRustPageOverlayViewProvider
    init(_ value: PDFRustPageOverlayViewProvider) { self.value = value }
}

@inline(__always)
func pdf_retain_any(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func pdf_retain_document(_ value: PDFDocument) -> UnsafeMutableRawPointer { pdf_retain_any(PDFDocumentBox(value)) }
@inline(__always)
func pdf_retain_page(_ value: PDFPage) -> UnsafeMutableRawPointer { pdf_retain_any(PDFPageBox(value)) }
@inline(__always)
func pdf_retain_annotation(_ value: PDFAnnotation) -> UnsafeMutableRawPointer { pdf_retain_any(PDFAnnotationBox(value)) }
@inline(__always)
func pdf_retain_outline(_ value: PDFOutline) -> UnsafeMutableRawPointer { pdf_retain_any(PDFOutlineBox(value)) }
@inline(__always)
func pdf_retain_selection(_ value: PDFSelection) -> UnsafeMutableRawPointer { pdf_retain_any(PDFSelectionBox(value)) }
@inline(__always)
func pdf_retain_view(_ value: PDFView) -> UnsafeMutableRawPointer { pdf_retain_any(PDFViewBox(value)) }
@inline(__always)
func pdf_retain_thumbnail_view(_ value: PDFThumbnailView) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFThumbnailViewBox(value))
}
@inline(__always)
func pdf_retain_action(_ value: PDFAction) -> UnsafeMutableRawPointer { pdf_retain_any(PDFActionBox(value)) }
@inline(__always)
func pdf_retain_action_url(_ value: PDFActionURL) -> UnsafeMutableRawPointer { pdf_retain_any(PDFActionURLBox(value)) }
@inline(__always)
func pdf_retain_action_goto(_ value: PDFActionGoTo) -> UnsafeMutableRawPointer { pdf_retain_any(PDFActionGoToBox(value)) }
@inline(__always)
func pdf_retain_action_named(_ value: PDFActionNamed) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFActionNamedBox(value))
}
@inline(__always)
func pdf_retain_action_remote_goto(_ value: PDFActionRemoteGoTo) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFActionRemoteGoToBox(value))
}
@inline(__always)
func pdf_retain_action_reset_form(_ value: PDFActionResetForm) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFActionResetFormBox(value))
}
@inline(__always)
func pdf_retain_border(_ value: PDFBorder) -> UnsafeMutableRawPointer { pdf_retain_any(PDFBorderBox(value)) }
@inline(__always)
func pdf_retain_destination(_ value: PDFDestination) -> UnsafeMutableRawPointer { pdf_retain_any(PDFDestinationBox(value)) }
@inline(__always)
func pdf_retain_page_overlay_view(_ value: NSView) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFPageOverlayViewBox(value))
}
@inline(__always)
func pdf_retain_appearance_characteristics(_ value: PDFAppearanceCharacteristics) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFAppearanceCharacteristicsBox(value))
}
@inline(__always)
func pdf_retain_document_delegate(_ value: PDFRustDocumentDelegate) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFRustDocumentDelegateBox(value))
}
@inline(__always)
func pdf_retain_view_delegate(_ value: PDFRustViewDelegate) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFRustViewDelegateBox(value))
}
@inline(__always)
func pdf_retain_page_overlay_view_provider(_ value: PDFRustPageOverlayViewProvider) -> UnsafeMutableRawPointer {
    pdf_retain_any(PDFRustPageOverlayViewProviderBox(value))
}

@inline(__always)
func pdf_document_box(_ handle: UnsafeMutableRawPointer?) -> PDFDocumentBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFDocumentBox.self)
    return Unmanaged<PDFDocumentBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_page_box(_ handle: UnsafeMutableRawPointer?) -> PDFPageBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFPageBox.self)
    return Unmanaged<PDFPageBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_annotation_box(_ handle: UnsafeMutableRawPointer?) -> PDFAnnotationBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFAnnotationBox.self)
    return Unmanaged<PDFAnnotationBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_outline_box(_ handle: UnsafeMutableRawPointer?) -> PDFOutlineBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFOutlineBox.self)
    return Unmanaged<PDFOutlineBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_selection_box(_ handle: UnsafeMutableRawPointer?) -> PDFSelectionBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFSelectionBox.self)
    return Unmanaged<PDFSelectionBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_view_box(_ handle: UnsafeMutableRawPointer?) -> PDFViewBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFViewBox.self)
    return Unmanaged<PDFViewBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_thumbnail_view_box(_ handle: UnsafeMutableRawPointer?) -> PDFThumbnailViewBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFThumbnailViewBox.self)
    return Unmanaged<PDFThumbnailViewBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_action_box(_ handle: UnsafeMutableRawPointer?) -> PDFActionBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFActionBox.self)
    return Unmanaged<PDFActionBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_action_url_box(_ handle: UnsafeMutableRawPointer?) -> PDFActionURLBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFActionURLBox.self)
    return Unmanaged<PDFActionURLBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_action_goto_box(_ handle: UnsafeMutableRawPointer?) -> PDFActionGoToBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFActionGoToBox.self)
    return Unmanaged<PDFActionGoToBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_action_named_box(_ handle: UnsafeMutableRawPointer?) -> PDFActionNamedBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFActionNamedBox.self)
    return Unmanaged<PDFActionNamedBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_action_remote_goto_box(_ handle: UnsafeMutableRawPointer?) -> PDFActionRemoteGoToBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFActionRemoteGoToBox.self)
    return Unmanaged<PDFActionRemoteGoToBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_action_reset_form_box(_ handle: UnsafeMutableRawPointer?) -> PDFActionResetFormBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFActionResetFormBox.self)
    return Unmanaged<PDFActionResetFormBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_border_box(_ handle: UnsafeMutableRawPointer?) -> PDFBorderBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFBorderBox.self)
    return Unmanaged<PDFBorderBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_destination_box(_ handle: UnsafeMutableRawPointer?) -> PDFDestinationBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFDestinationBox.self)
    return Unmanaged<PDFDestinationBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_page_overlay_view_box(_ handle: UnsafeMutableRawPointer?) -> PDFPageOverlayViewBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFPageOverlayViewBox.self)
    return Unmanaged<PDFPageOverlayViewBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_appearance_characteristics_box(_ handle: UnsafeMutableRawPointer?) -> PDFAppearanceCharacteristicsBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFAppearanceCharacteristicsBox.self)
    return Unmanaged<PDFAppearanceCharacteristicsBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_document_delegate_box(_ handle: UnsafeMutableRawPointer?) -> PDFRustDocumentDelegateBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFRustDocumentDelegateBox.self)
    return Unmanaged<PDFRustDocumentDelegateBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_view_delegate_box(_ handle: UnsafeMutableRawPointer?) -> PDFRustViewDelegateBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFRustViewDelegateBox.self)
    return Unmanaged<PDFRustViewDelegateBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_page_overlay_view_provider_box(_ handle: UnsafeMutableRawPointer?) -> PDFRustPageOverlayViewProviderBox? {
    guard let handle else { return nil }
    let typed = handle.assumingMemoryBound(to: PDFRustPageOverlayViewProviderBox.self)
    return Unmanaged<PDFRustPageOverlayViewProviderBox>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func pdf_document_value(_ handle: UnsafeMutableRawPointer?) -> PDFDocument? { pdf_document_box(handle)?.value }
@inline(__always)
func pdf_page_value(_ handle: UnsafeMutableRawPointer?) -> PDFPage? { pdf_page_box(handle)?.value }
@inline(__always)
func pdf_annotation_value(_ handle: UnsafeMutableRawPointer?) -> PDFAnnotation? { pdf_annotation_box(handle)?.value }
@inline(__always)
func pdf_outline_value(_ handle: UnsafeMutableRawPointer?) -> PDFOutline? { pdf_outline_box(handle)?.value }
@inline(__always)
func pdf_selection_value(_ handle: UnsafeMutableRawPointer?) -> PDFSelection? { pdf_selection_box(handle)?.value }
@inline(__always)
func pdf_view_value(_ handle: UnsafeMutableRawPointer?) -> PDFView? { pdf_view_box(handle)?.value }
@inline(__always)
func pdf_thumbnail_view_value(_ handle: UnsafeMutableRawPointer?) -> PDFThumbnailView? { pdf_thumbnail_view_box(handle)?.value }
@inline(__always)
func pdf_action_value(_ handle: UnsafeMutableRawPointer?) -> PDFAction? { pdf_action_box(handle)?.value }
@inline(__always)
func pdf_action_url_value(_ handle: UnsafeMutableRawPointer?) -> PDFActionURL? { pdf_action_url_box(handle)?.value }
@inline(__always)
func pdf_action_goto_value(_ handle: UnsafeMutableRawPointer?) -> PDFActionGoTo? { pdf_action_goto_box(handle)?.value }
@inline(__always)
func pdf_action_named_value(_ handle: UnsafeMutableRawPointer?) -> PDFActionNamed? { pdf_action_named_box(handle)?.value }
@inline(__always)
func pdf_action_remote_goto_value(_ handle: UnsafeMutableRawPointer?) -> PDFActionRemoteGoTo? {
    pdf_action_remote_goto_box(handle)?.value
}
@inline(__always)
func pdf_action_reset_form_value(_ handle: UnsafeMutableRawPointer?) -> PDFActionResetForm? {
    pdf_action_reset_form_box(handle)?.value
}
@inline(__always)
func pdf_any_action_value(_ handle: UnsafeMutableRawPointer?) -> PDFAction? {
    pdf_action_value(handle)
        ?? pdf_action_url_value(handle)
        ?? pdf_action_goto_value(handle)
        ?? pdf_action_named_value(handle)
        ?? pdf_action_remote_goto_value(handle)
        ?? pdf_action_reset_form_value(handle)
}
@inline(__always)
func pdf_border_value(_ handle: UnsafeMutableRawPointer?) -> PDFBorder? { pdf_border_box(handle)?.value }
@inline(__always)
func pdf_destination_value(_ handle: UnsafeMutableRawPointer?) -> PDFDestination? { pdf_destination_box(handle)?.value }
@inline(__always)
func pdf_page_overlay_view_value(_ handle: UnsafeMutableRawPointer?) -> NSView? {
    pdf_page_overlay_view_box(handle)?.value
}
@inline(__always)
func pdf_appearance_characteristics_value(_ handle: UnsafeMutableRawPointer?) -> PDFAppearanceCharacteristics? {
    pdf_appearance_characteristics_box(handle)?.value
}
@inline(__always)
func pdf_document_delegate_value(_ handle: UnsafeMutableRawPointer?) -> PDFRustDocumentDelegate? {
    pdf_document_delegate_box(handle)?.value
}
@inline(__always)
func pdf_view_delegate_value(_ handle: UnsafeMutableRawPointer?) -> PDFRustViewDelegate? {
    pdf_view_delegate_box(handle)?.value
}
@inline(__always)
func pdf_page_overlay_view_provider_value(_ handle: UnsafeMutableRawPointer?) -> PDFRustPageOverlayViewProvider? {
    pdf_page_overlay_view_provider_box(handle)?.value
}

@inline(__always)
public func pdf_string(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

public enum PDFBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case nullResult(String)
    case framework(String)

    public var description: String {
        switch self {
        case .invalidArgument(let message), .nullResult(let message), .framework(let message):
            return message
        }
    }

    public var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return PDFX_INVALID_ARGUMENT
        case .nullResult:
            return PDFX_NULL_RESULT
        case .framework:
            return PDFX_FRAMEWORK
        }
    }
}

@inline(__always)
public func pdf_fail(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) -> Int32 {
    let bridgeError = (error as? PDFBridgeError) ?? .framework((error as NSError).localizedDescription)
    outError?.pointee = pdf_string(bridgeError.description)
    return bridgeError.statusCode
}

@inline(__always)
public func pdf_run(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ work: () throws -> Void
) -> Int32 {
    do {
        try work()
        outError?.pointee = nil
        return PDFX_OK
    } catch {
        return pdf_fail(outError, error)
    }
}

func pdf_json_string(from value: Any) -> String? {
    guard JSONSerialization.isValidJSONObject(value) else { return nil }
    do {
        let data = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
        return String(data: data, encoding: .utf8)
    } catch {
        return nil
    }
}

func pdf_optional_string(_ value: UnsafePointer<CChar>?) -> String? {
    guard let value else { return nil }
    return String(cString: value)
}

func pdf_take_string(_ value: UnsafeMutablePointer<CChar>?) -> String? {
    guard let value else { return nil }
    defer { free(value) }
    return String(cString: value)
}

func pdf_rect_dict(_ rect: CGRect) -> [String: Any] {
    [
        "x": rect.origin.x,
        "y": rect.origin.y,
        "width": rect.size.width,
        "height": rect.size.height,
    ]
}

func pdf_point_dict(_ point: CGPoint) -> [String: Any] {
    ["x": point.x, "y": point.y]
}

func pdf_size_dict(_ size: CGSize) -> [String: Any] {
    ["width": size.width, "height": size.height]
}

func pdf_range_dict(_ range: NSRange) -> [String: Any] {
    ["location": range.location, "length": range.length]
}

func pdf_insets_dict(_ insets: NSEdgeInsets) -> [String: Any] {
    [
        "top": insets.top,
        "left": insets.left,
        "bottom": insets.bottom,
        "right": insets.right,
    ]
}

func pdf_color_dict(_ color: NSColor?) -> [String: Any]? {
    guard let color else { return nil }
    let converted = color.usingColorSpace(.sRGB) ?? color.usingColorSpace(.deviceRGB)
    guard let converted else { return nil }
    return [
        "red": converted.redComponent,
        "green": converted.greenComponent,
        "blue": converted.blueComponent,
        "alpha": converted.alphaComponent,
    ]
}

func pdf_color(red: Double, green: Double, blue: Double, alpha: Double) -> NSColor {
    NSColor(
        calibratedRed: CGFloat(red),
        green: CGFloat(green),
        blue: CGFloat(blue),
        alpha: CGFloat(alpha)
    )
}

func pdf_border_info_dict(_ border: PDFBorder?) -> [String: Any]? {
    guard let border else { return nil }
    return [
        "style": border.style.rawValue,
        "line_width": border.lineWidth,
        "dash_pattern": border.dashPattern?.compactMap { ($0 as? NSNumber)?.doubleValue } as Any,
    ]
}

func pdf_display_box(_ rawValue: Int32) throws -> PDFDisplayBox {
    guard let box = PDFDisplayBox(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFDisplayBox: \(rawValue)")
    }
    return box
}

func pdf_border_style(_ rawValue: Int32) throws -> PDFBorderStyle {
    guard let style = PDFBorderStyle(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFBorderStyle: \(rawValue)")
    }
    return style
}

func pdf_display_mode(_ rawValue: Int32) throws -> PDFDisplayMode {
    guard let mode = PDFDisplayMode(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFDisplayMode: \(rawValue)")
    }
    return mode
}

func pdf_display_direction(_ rawValue: Int32) throws -> PDFDisplayDirection {
    guard let direction = PDFDisplayDirection(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFDisplayDirection: \(rawValue)")
    }
    return direction
}

func pdf_interpolation_quality(_ rawValue: Int32) throws -> PDFInterpolationQuality {
    guard let quality = PDFInterpolationQuality(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFInterpolationQuality: \(rawValue)")
    }
    return quality
}

func pdf_widget_control_type(_ rawValue: Int32) throws -> PDFWidgetControlType {
    guard let controlType = PDFWidgetControlType(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFWidgetControlType: \(rawValue)")
    }
    return controlType
}

func pdf_action_named_name(_ rawValue: Int32) throws -> PDFActionNamedName {
    guard let name = PDFActionNamedName(rawValue: Int(rawValue)) else {
        throw PDFBridgeError.invalidArgument("invalid PDFActionNamedName: \(rawValue)")
    }
    return name
}

@_cdecl("pdf_object_retain")
public func pdf_object_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
    return Unmanaged.passRetained(object).toOpaque()
}

@_cdecl("pdf_object_release")
public func pdf_object_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}
