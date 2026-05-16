import Foundation
import PDFKit

@_cdecl("pdf_action_type_string")
public func pdf_action_type_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_value(handle) else { return nil }
    return pdf_string(action.type)
}

@_cdecl("pdf_action_as_url")
public func pdf_action_as_url(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action = pdf_action_value(handle), let typed = action as? PDFActionURL else {
        return nil
    }
    return pdf_retain_action_url(typed)
}

@_cdecl("pdf_action_as_goto")
public func pdf_action_as_goto(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action = pdf_action_value(handle), let typed = action as? PDFActionGoTo else {
        return nil
    }
    return pdf_retain_action_goto(typed)
}

@_cdecl("pdf_action_as_named")
public func pdf_action_as_named(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action = pdf_action_value(handle), let typed = action as? PDFActionNamed else {
        return nil
    }
    return pdf_retain_action_named(typed)
}

@_cdecl("pdf_action_as_remote_goto")
public func pdf_action_as_remote_goto(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action = pdf_action_value(handle), let typed = action as? PDFActionRemoteGoTo else {
        return nil
    }
    return pdf_retain_action_remote_goto(typed)
}
