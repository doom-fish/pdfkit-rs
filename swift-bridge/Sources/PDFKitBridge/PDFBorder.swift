import Foundation
import PDFKit

@_cdecl("pdf_border_new")
public func pdf_border_new(
    _ outBorder: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outBorder else {
            throw PDFBridgeError.invalidArgument("missing border output pointer")
        }
        outBorder.pointee = pdf_retain_border(PDFBorder())
    }
}

@_cdecl("pdf_border_info_json")
public func pdf_border_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let border = pdf_border_value(handle), let json = pdf_json_string(from: pdf_border_info_dict(border) ?? [:]) else {
        return nil
    }
    return pdf_string(json)
}

@_cdecl("pdf_border_set_style")
public func pdf_border_set_style(
    _ handle: UnsafeMutableRawPointer?,
    _ rawValue: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let border = pdf_border_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing border handle")
        }
        border.style = try pdf_border_style(rawValue)
    }
}

@_cdecl("pdf_border_set_line_width")
public func pdf_border_set_line_width(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let border = pdf_border_value(handle) else { return }
    border.lineWidth = CGFloat(value)
}

@_cdecl("pdf_border_set_dash_pattern")
public func pdf_border_set_dash_pattern(
    _ handle: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<Double>?,
    _ len: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let border = pdf_border_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing border handle")
        }
        guard let values else {
            border.dashPattern = nil
            return
        }
        let buffer = UnsafeBufferPointer(start: values, count: Int(len))
        border.dashPattern = buffer.map { NSNumber(value: $0) }
    }
}
