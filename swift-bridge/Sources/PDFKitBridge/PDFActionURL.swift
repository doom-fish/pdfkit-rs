import Foundation
import PDFKit

@_cdecl("pdf_action_url_new")
public func pdf_action_url_new(
    _ urlString: UnsafePointer<CChar>?,
    _ outAction: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let urlString, let outAction else {
            throw PDFBridgeError.invalidArgument("missing URL string or output pointer")
        }
        guard let url = URL(string: String(cString: urlString)) else {
            throw PDFBridgeError.invalidArgument("invalid URL string")
        }
        outAction.pointee = pdf_retain_action_url(PDFActionURL(url: url))
    }
}

@_cdecl("pdf_action_url_string")
public func pdf_action_url_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_url_value(handle), let url = action.url else {
        return nil
    }
    return pdf_string(url.absoluteString)
}

@_cdecl("pdf_action_url_set_url")
public func pdf_action_url_set_url(
    _ handle: UnsafeMutableRawPointer?,
    _ urlString: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let action = pdf_action_url_value(handle), let urlString else {
            throw PDFBridgeError.invalidArgument("missing action handle or URL string")
        }
        guard let url = URL(string: String(cString: urlString)) else {
            throw PDFBridgeError.invalidArgument("invalid URL string")
        }
        action.url = url
    }
}

@_cdecl("pdf_action_url_type_string")
public func pdf_action_url_type_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_url_value(handle) else { return nil }
    return pdf_string(action.type)
}
