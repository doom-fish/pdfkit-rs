import Foundation
import PDFKit

@_cdecl("pdf_action_goto_new")
public func pdf_action_goto_new(
    _ destinationHandle: UnsafeMutableRawPointer?,
    _ outAction: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let destination = pdf_destination_value(destinationHandle), let outAction else {
            throw PDFBridgeError.invalidArgument("missing destination handle or output pointer")
        }
        outAction.pointee = pdf_retain_action_goto(PDFActionGoTo(destination: destination))
    }
}

@_cdecl("pdf_action_goto_destination")
public func pdf_action_goto_destination(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action = pdf_action_goto_value(handle) else { return nil }
    return pdf_retain_destination(action.destination)
}

@_cdecl("pdf_action_goto_set_destination")
public func pdf_action_goto_set_destination(
    _ handle: UnsafeMutableRawPointer?,
    _ destinationHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let action = pdf_action_goto_value(handle), let destination = pdf_destination_value(destinationHandle) else {
            throw PDFBridgeError.invalidArgument("missing action or destination handle")
        }
        action.destination = destination
    }
}

@_cdecl("pdf_action_goto_type_string")
public func pdf_action_goto_type_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_goto_value(handle) else { return nil }
    return pdf_string(action.type)
}
