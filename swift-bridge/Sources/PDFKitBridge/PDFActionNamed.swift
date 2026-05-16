import Foundation
import PDFKit

@_cdecl("pdf_action_named_new")
public func pdf_action_named_new(
    _ rawName: Int32,
    _ outAction: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outAction else {
            throw PDFBridgeError.invalidArgument("missing action output pointer")
        }
        outAction.pointee = pdf_retain_action_named(PDFActionNamed(name: try pdf_action_named_name(rawName)))
    }
}

@_cdecl("pdf_action_named_name_raw")
public func pdf_action_named_name_raw(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let action = pdf_action_named_value(handle) else { return .min }
    return Int32(action.name.rawValue)
}

@_cdecl("pdf_action_named_set_name")
public func pdf_action_named_set_name(
    _ handle: UnsafeMutableRawPointer?,
    _ rawName: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let action = pdf_action_named_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing named action handle")
        }
        action.name = try pdf_action_named_name(rawName)
    }
}

@_cdecl("pdf_action_named_type_string")
public func pdf_action_named_type_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_named_value(handle) else { return nil }
    return pdf_string(action.type)
}
