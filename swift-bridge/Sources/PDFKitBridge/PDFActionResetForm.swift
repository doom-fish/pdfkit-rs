import Foundation
import PDFKit

@_cdecl("pdf_action_reset_form_new")
public func pdf_action_reset_form_new(
    _ outAction: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outAction else {
            throw PDFBridgeError.invalidArgument("missing action output pointer")
        }
        outAction.pointee = pdf_retain_action_reset_form(PDFActionResetForm())
    }
}

@_cdecl("pdf_action_reset_form_fields_json")
public func pdf_action_reset_form_fields_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_reset_form_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: action.fields ?? []) ?? "[]")
}

@_cdecl("pdf_action_reset_form_set_fields_json")
public func pdf_action_reset_form_set_fields_json(
    _ handle: UnsafeMutableRawPointer?,
    _ fieldsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let action = pdf_action_reset_form_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing reset-form action handle")
        }
        guard let fieldsJSON else {
            action.fields = nil
            return
        }
        let data = Data(String(cString: fieldsJSON).utf8)
        guard let fields = try JSONSerialization.jsonObject(with: data) as? [String] else {
            throw PDFBridgeError.invalidArgument("reset-form fields JSON must decode to an array of strings")
        }
        action.fields = fields
    }
}

@_cdecl("pdf_action_reset_form_fields_included_are_cleared")
public func pdf_action_reset_form_fields_included_are_cleared(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let action = pdf_action_reset_form_value(handle) else { return 0 }
    return action.fieldsIncludedAreCleared ? 1 : 0
}

@_cdecl("pdf_action_reset_form_set_fields_included_are_cleared")
public func pdf_action_reset_form_set_fields_included_are_cleared(
    _ handle: UnsafeMutableRawPointer?,
    _ value: Int32
) {
    guard let action = pdf_action_reset_form_value(handle) else { return }
    action.fieldsIncludedAreCleared = value != 0
}

@_cdecl("pdf_action_reset_form_type_string")
public func pdf_action_reset_form_type_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_reset_form_value(handle) else { return nil }
    return pdf_string(action.type)
}
