import Foundation
import PDFKit

@_cdecl("pdf_action_remote_goto_new")
public func pdf_action_remote_goto_new(
    _ pageIndex: UInt64,
    _ x: Double,
    _ y: Double,
    _ urlString: UnsafePointer<CChar>?,
    _ outAction: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let urlString, let outAction else {
            throw PDFBridgeError.invalidArgument("missing remote action URL or output pointer")
        }
        guard let url = URL(string: String(cString: urlString)), url.isFileURL else {
            throw PDFBridgeError.invalidArgument("invalid file URL string")
        }
        let action = PDFActionRemoteGoTo(
            pageIndex: Int(pageIndex),
            at: CGPoint(x: x, y: y),
            fileURL: url
        )
        outAction.pointee = pdf_retain_action_remote_goto(action)
    }
}

@_cdecl("pdf_action_remote_goto_page_index")
public func pdf_action_remote_goto_page_index(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let action = pdf_action_remote_goto_value(handle) else { return 0 }
    return UInt64(action.pageIndex)
}

@_cdecl("pdf_action_remote_goto_set_page_index")
public func pdf_action_remote_goto_set_page_index(_ handle: UnsafeMutableRawPointer?, _ pageIndex: UInt64) {
    guard let action = pdf_action_remote_goto_value(handle) else { return }
    action.pageIndex = Int(pageIndex)
}

@_cdecl("pdf_action_remote_goto_point_x")
public func pdf_action_remote_goto_point_x(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let action = pdf_action_remote_goto_value(handle) else { return 0 }
    return Double(action.point.x)
}

@_cdecl("pdf_action_remote_goto_point_y")
public func pdf_action_remote_goto_point_y(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let action = pdf_action_remote_goto_value(handle) else { return 0 }
    return Double(action.point.y)
}

@_cdecl("pdf_action_remote_goto_set_point")
public func pdf_action_remote_goto_set_point(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let action = pdf_action_remote_goto_value(handle) else { return }
    action.point = CGPoint(x: x, y: y)
}

@_cdecl("pdf_action_remote_goto_url_string")
public func pdf_action_remote_goto_url_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_remote_goto_value(handle) else { return nil }
    return pdf_string(action.url.absoluteString)
}

@_cdecl("pdf_action_remote_goto_set_url")
public func pdf_action_remote_goto_set_url(
    _ handle: UnsafeMutableRawPointer?,
    _ urlString: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let action = pdf_action_remote_goto_value(handle), let urlString else {
            throw PDFBridgeError.invalidArgument("missing remote action handle or URL string")
        }
        guard let url = URL(string: String(cString: urlString)), url.isFileURL else {
            throw PDFBridgeError.invalidArgument("invalid file URL string")
        }
        action.url = url
    }
}

@_cdecl("pdf_action_remote_goto_type_string")
public func pdf_action_remote_goto_type_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let action = pdf_action_remote_goto_value(handle) else { return nil }
    return pdf_string(action.type)
}
