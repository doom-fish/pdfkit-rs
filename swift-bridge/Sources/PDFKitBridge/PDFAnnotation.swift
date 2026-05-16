import Foundation
import PDFKit

private func pdf_annotation_info(_ annotation: PDFAnnotation) -> [String: Any] {
    let formatter = ISO8601DateFormatter()
    return [
        "annotation_type": annotation.type as Any,
        "bounds": pdf_rect_dict(annotation.bounds),
        "contents": annotation.contents as Any,
        "should_display": annotation.shouldDisplay,
        "should_print": annotation.shouldPrint,
        "has_appearance_stream": annotation.hasAppearanceStream,
        "user_name": annotation.userName as Any,
        "modification_date": annotation.modificationDate.map(formatter.string(from:)) as Any,
        "color": pdf_color_dict(annotation.color) as Any,
        "highlighted": annotation.isHighlighted,
        "action_type": annotation.action?.type as Any,
        "border": pdf_border_info_dict(annotation.border) as Any,
    ]
}

@_cdecl("pdf_annotation_new")
public func pdf_annotation_new(
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ annotationType: UnsafePointer<CChar>?,
    _ outAnnotation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let annotationType, let outAnnotation else {
            throw PDFBridgeError.invalidArgument("missing annotation type or output pointer")
        }
        let annotation = PDFAnnotation(
            bounds: CGRect(x: x, y: y, width: width, height: height),
            forType: PDFAnnotationSubtype(rawValue: String(cString: annotationType)),
            withProperties: nil
        )
        outAnnotation.pointee = pdf_retain_annotation(annotation)
    }
}

@_cdecl("pdf_annotation_info_json")
public func pdf_annotation_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let annotation = pdf_annotation_value(handle) else { return nil }
    return pdf_string(pdf_json_string(from: pdf_annotation_info(annotation)) ?? "{}")
}

@_cdecl("pdf_annotation_set_contents")
public func pdf_annotation_set_contents(
    _ handle: UnsafeMutableRawPointer?,
    _ value: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let annotation = pdf_annotation_value(handle) else {
            throw PDFBridgeError.invalidArgument("missing annotation handle")
        }
        annotation.contents = pdf_optional_string(value)
    }
}

@_cdecl("pdf_annotation_set_highlighted")
public func pdf_annotation_set_highlighted(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let annotation = pdf_annotation_value(handle) else { return }
    annotation.isHighlighted = value != 0
}

@_cdecl("pdf_annotation_border")
public func pdf_annotation_border(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let annotation = pdf_annotation_value(handle), let border = annotation.border else {
        return nil
    }
    return pdf_retain_border(border)
}

@_cdecl("pdf_annotation_set_border")
public func pdf_annotation_set_border(
    _ annotationHandle: UnsafeMutableRawPointer?,
    _ borderHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let annotation = pdf_annotation_value(annotationHandle) else {
            throw PDFBridgeError.invalidArgument("missing annotation handle")
        }
        annotation.border = pdf_border_value(borderHandle)
    }
}

@_cdecl("pdf_annotation_action_url")
public func pdf_annotation_action_url(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let annotation = pdf_annotation_value(handle), let action = annotation.action as? PDFActionURL else {
        return nil
    }
    return pdf_retain_action_url(action)
}

@_cdecl("pdf_annotation_set_action_url")
public func pdf_annotation_set_action_url(
    _ annotationHandle: UnsafeMutableRawPointer?,
    _ actionHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let annotation = pdf_annotation_value(annotationHandle) else {
            throw PDFBridgeError.invalidArgument("missing annotation handle")
        }
        annotation.action = pdf_action_url_value(actionHandle)
    }
}

@_cdecl("pdf_annotation_action_goto")
public func pdf_annotation_action_goto(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let annotation = pdf_annotation_value(handle), let action = annotation.action as? PDFActionGoTo else {
        return nil
    }
    return pdf_retain_action_goto(action)
}

@_cdecl("pdf_annotation_set_action_goto")
public func pdf_annotation_set_action_goto(
    _ annotationHandle: UnsafeMutableRawPointer?,
    _ actionHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    pdf_run(outError) {
        guard let annotation = pdf_annotation_value(annotationHandle) else {
            throw PDFBridgeError.invalidArgument("missing annotation handle")
        }
        annotation.action = pdf_action_goto_value(actionHandle)
    }
}
