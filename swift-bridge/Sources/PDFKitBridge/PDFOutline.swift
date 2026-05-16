import Foundation
import PDFKit

@_cdecl("pdf_outline_new")
public func pdf_outline_new(
    _ outOutline: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outOutline else {
            throw PDFBridgeError.invalidArgument("missing outline output pointer")
        }
        outOutline.pointee = pdf_retain_outline(PDFOutline())
    }
}

@_cdecl("pdf_outline_label_string")
public func pdf_outline_label_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let outline = pdf_outline_value(handle), let label = outline.label else {
        return nil
    }
    return pdf_string(label)
}

@_cdecl("pdf_outline_set_label")
public func pdf_outline_set_label(
    _ handle: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outline = pdf_outline_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing outline handle")
        }
        outline.label = pdf_optional_string(value)
    }
}

@_cdecl("pdf_outline_child_count")
public func pdf_outline_child_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let outline = pdf_outline_value(handle) else { return 0 }
    return UInt64(outline.numberOfChildren)
}

@_cdecl("pdf_outline_child_at")
public func pdf_outline_child_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let outline = pdf_outline_value(handle),
          index < UInt64(outline.numberOfChildren),
          let child = outline.child(at: Int(index))
    else {
        return nil
    }
    return pdf_retain_outline(child)
}

@_cdecl("pdf_outline_insert_child")
public func pdf_outline_insert_child(
    _ handle: UnsafeMutableRawPointer?,
    _ childHandle: UnsafeMutableRawPointer?,
    _ index: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outline = pdf_outline_value(handle), let child = pdf_outline_value(childHandle) else {
            throw PDFBridgeError.invalidArgument("missing outline or child handle")
        }
        guard index <= UInt64(outline.numberOfChildren) else {
            throw PDFBridgeError.invalidArgument("child index out of range")
        }
        outline.insertChild(child, at: Int(index))
    }
}

@_cdecl("pdf_outline_remove_from_parent")
public func pdf_outline_remove_from_parent(_ handle: UnsafeMutableRawPointer?) {
    guard let outline = pdf_outline_value(handle) else { return }
    outline.removeFromParent()
}

@_cdecl("pdf_outline_index")
public func pdf_outline_index(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let outline = pdf_outline_value(handle) else { return 0 }
    return UInt64(outline.index)
}

@_cdecl("pdf_outline_is_open")
public func pdf_outline_is_open(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let outline = pdf_outline_value(handle) else { return 0 }
    return outline.isOpen ? 1 : 0
}

@_cdecl("pdf_outline_set_open")
public func pdf_outline_set_open(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let outline = pdf_outline_value(handle) else { return }
    outline.isOpen = value != 0
}

@_cdecl("pdf_outline_destination")
public func pdf_outline_destination(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let outline = pdf_outline_value(handle), let destination = outline.destination else {
        return nil
    }
    return pdf_retain_destination(destination)
}

@_cdecl("pdf_outline_set_destination")
public func pdf_outline_set_destination(
    _ handle: UnsafeMutableRawPointer?,
    _ destinationHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outline = pdf_outline_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing outline handle")
        }
        outline.destination = pdf_destination_value(destinationHandle)
    }
}

@_cdecl("pdf_outline_action")
public func pdf_outline_action(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let outline = pdf_outline_value(handle), let action = outline.action else {
        return nil
    }
    return pdf_retain_action(action)
}

@_cdecl("pdf_outline_set_action")
public func pdf_outline_set_action(
    _ handle: UnsafeMutableRawPointer?,
    _ actionHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outline = pdf_outline_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing outline handle")
        }
        outline.action = pdf_any_action_value(actionHandle)
    }
}

@_cdecl("pdf_outline_action_url")
public func pdf_outline_action_url(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let outline = pdf_outline_value(handle), let action = outline.action as? PDFActionURL else {
        return nil
    }
    return pdf_retain_action_url(action)
}

@_cdecl("pdf_outline_set_action_url")
public func pdf_outline_set_action_url(
    _ handle: UnsafeMutableRawPointer?,
    _ actionHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outline = pdf_outline_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing outline handle")
        }
        outline.action = pdf_action_url_value(actionHandle)
    }
}

@_cdecl("pdf_outline_action_goto")
public func pdf_outline_action_goto(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let outline = pdf_outline_value(handle), let action = outline.action as? PDFActionGoTo else {
        return nil
    }
    return pdf_retain_action_goto(action)
}

@_cdecl("pdf_outline_set_action_goto")
public func pdf_outline_set_action_goto(
    _ handle: UnsafeMutableRawPointer?,
    _ actionHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let outline = pdf_outline_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing outline handle")
        }
        outline.action = pdf_action_goto_value(actionHandle)
    }
}

@_cdecl("pdf_outline_parent")
public func pdf_outline_parent(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let outline = pdf_outline_value(handle), let parent = outline.parent else {
        return nil
    }
    return pdf_retain_outline(parent)
}
