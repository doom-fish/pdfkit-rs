import Foundation

public let PDFX_OK: Int32 = 0
public let PDFX_INVALID_ARGUMENT: Int32 = -1
public let PDFX_NULL_RESULT: Int32 = -2
public let PDFX_FRAMEWORK: Int32 = -3

@inline(__always)
public func pdf_string(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func pdf_retain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func pdf_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@inline(__always)
public func pdf_borrow_object(_ handle: UnsafeMutableRawPointer?) -> AnyObject? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
}

public enum PDFBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case nullResult(String)
    case framework(Error)

    public var description: String {
        switch self {
        case .invalidArgument(let message), .nullResult(let message):
            return message
        case .framework(let error):
            return error.localizedDescription
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
public func pdf_status(from error: Error) -> Int32 {
    if let error = error as? PDFBridgeError {
        return error.statusCode
    }
    return PDFX_FRAMEWORK
}

@inline(__always)
public func pdf_message(from error: Error) -> String {
    if let error = error as? PDFBridgeError {
        return error.description
    }
    return (error as NSError).localizedDescription
}

@inline(__always)
public func pdf_fail(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) -> Int32 {
    outError?.pointee = pdf_string(pdf_message(from: error))
    return pdf_status(from: error)
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

public func pdf_json_string(from value: Any) -> String? {
    guard JSONSerialization.isValidJSONObject(value) else { return nil }
    do {
        let data = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
        return String(data: data, encoding: .utf8)
    } catch {
        return nil
    }
}

@_cdecl("pdf_object_retain")
public func pdf_object_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = pdf_borrow_object(handle) else { return nil }
    return pdf_retain(object)
}

@_cdecl("pdf_object_release")
public func pdf_object_release(_ handle: UnsafeMutableRawPointer?) {
    pdf_release(handle)
}
