import Foundation
import PDFKit

private func pdf_appearance_characteristics_info(_ value: PDFAppearanceCharacteristics) -> [String: Any] {
    [
        "control_type": value.controlType.rawValue,
        "background_color": pdf_color_dict(value.backgroundColor) as Any,
        "border_color": pdf_color_dict(value.borderColor) as Any,
        "rotation": value.rotation,
        "caption": value.caption as Any,
        "rollover_caption": value.rolloverCaption as Any,
        "down_caption": value.downCaption as Any,
    ]
}

@_cdecl("pdf_appearance_characteristics_new")
public func pdf_appearance_characteristics_new(
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outValue else {
            throw PDFBridgeError.invalidArgument("missing appearance characteristics output pointer")
        }
        outValue.pointee = pdf_retain_appearance_characteristics(PDFAppearanceCharacteristics())
    }
}

@_cdecl("pdf_appearance_characteristics_info_json")
public func pdf_appearance_characteristics_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let value = pdf_appearance_characteristics_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: pdf_appearance_characteristics_info(value)) ?? "{}")
}

@_cdecl("pdf_appearance_characteristics_set_control_type")
public func pdf_appearance_characteristics_set_control_type(
    _ handle: UnsafeMutableRawPointer?,
    _ rawValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let value = pdf_appearance_characteristics_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing appearance characteristics handle")
        }
        value.controlType = try pdf_widget_control_type(rawValue)
    }
}

@_cdecl("pdf_appearance_characteristics_set_rotation")
public func pdf_appearance_characteristics_set_rotation(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let appearance = pdf_appearance_characteristics_value(handle) else { return }
    appearance.rotation = Int(value)
}

@_cdecl("pdf_appearance_characteristics_set_caption")
public func pdf_appearance_characteristics_set_caption(
    _ handle: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?
) {
    guard let appearance = pdf_appearance_characteristics_value(handle) else { return }
    appearance.caption = pdf_optional_string(value)
}

@_cdecl("pdf_appearance_characteristics_set_rollover_caption")
public func pdf_appearance_characteristics_set_rollover_caption(
    _ handle: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?
) {
    guard let appearance = pdf_appearance_characteristics_value(handle) else { return }
    appearance.rolloverCaption = pdf_optional_string(value)
}

@_cdecl("pdf_appearance_characteristics_set_down_caption")
public func pdf_appearance_characteristics_set_down_caption(
    _ handle: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?
) {
    guard let appearance = pdf_appearance_characteristics_value(handle) else { return }
    appearance.downCaption = pdf_optional_string(value)
}

@_cdecl("pdf_appearance_characteristics_set_background_color")
public func pdf_appearance_characteristics_set_background_color(
    _ handle: UnsafeMutableRawPointer?,
    _ red: Double,
    _ green: Double,
    _ blue: Double,
    _ alpha: Double
) {
    guard let appearance = pdf_appearance_characteristics_value(handle) else { return }
    appearance.backgroundColor = pdf_color(red: red, green: green, blue: blue, alpha: alpha)
}

@_cdecl("pdf_appearance_characteristics_set_border_color")
public func pdf_appearance_characteristics_set_border_color(
    _ handle: UnsafeMutableRawPointer?,
    _ red: Double,
    _ green: Double,
    _ blue: Double,
    _ alpha: Double
) {
    guard let appearance = pdf_appearance_characteristics_value(handle) else { return }
    appearance.borderColor = pdf_color(red: red, green: green, blue: blue, alpha: alpha)
}
