# PDFKit v0.2.2 coverage audit

Audit source: `PDFKit.framework/Headers/` from the active macOS SDK (`xcrun --sdk macosx --show-sdk-path`).

This crate's v0.2.2 target closes every non-deprecated top-level Objective-C declaration currently counted by the symbol audit. Remaining non-implemented rows below are either forward declarations with no public interface, deprecated headers treated as exempt, or SDK metadata/umbrella headers.

| Header / surface | Status | Notes |
| --- | --- | --- |
| `PDFDocument.h` | ✅ implemented | `PdfDocument` covers creation/loading, metadata, page access, page mutation, outline roots, delegate attachment, print-scaling and selection-granularity enums, write options, document selections, unlock, notification-name enums, and document notification user-info keys. `exchange_pages` defensively rejects synthetic in-memory pages to avoid a framework exception. |
| `PDFPage.h` | ✅ implemented | `PdfPage` covers creation, image-backed page creation, image initialization options, bounds/rotation, annotation access/mutation, `PdfAreaOfInterest`, text geometry, and range/point-based selections. |
| `PDFAnnotation.h` | ✅ implemented | `PdfAnnotation` covers creation, typed subtype creation, metadata, contents, highlight state, border association, generic action attachment/inspection, and typed annotation key wrappers. |
| `PDFAnnotationUtilities.h` | ✅ implemented | The crate now exposes `PDFWidgetControlType`, `PDFLineStyle`, `PDFMarkupType`, `PDFTextAnnotationIconType`, `PDFWidgetCellState`, all public annotation string constants, and widget/text-icon subtype wrappers. |
| `PDFOutline.h` | ✅ implemented | `PdfOutline` covers creation, labels, children, open state, destinations, generic action attachment/inspection, and URL/GoTo convenience helpers. |
| `PDFSelection.h` | ✅ implemented | `PdfSelection` covers creation, pages, bounds, text ranges, line splitting, add-selection, and extension helpers. |
| `PDFView.h` | ✅ implemented | `PdfView` covers document association, display/scaling state, navigation, current selection, visible page access, structured `PDFView` notification names, delegate attachment, page-overlay providers, and area-of-interest queries. |
| `PDFThumbnailView.h` | ✅ implemented | `PdfThumbnailView` covers `pdfView` association, thumbnail sizing, column count, dragging/multi-select flags, selected page access, the structured document-edited notification name, and the `PdfThumbnailLayoutMode` audit mirror. |
| `PDFAction.h` | ✅ implemented | `PdfAction` covers abstract action handles, action-type lookup, and downcasts to URL, GoTo, named, remote-go-to, and reset-form actions. |
| `PDFActionURL.h` | ✅ implemented | `PdfActionUrl` covers construction, URL get/set, and action type lookup. |
| `PDFActionGoTo.h` | ✅ implemented | `PdfActionGoTo` covers construction, destination get/set, and action type lookup. |
| `PDFActionNamed.h` | ✅ implemented | `PdfActionNamedName` and `PdfActionNamed` cover named viewer-action enums plus construction, name get/set, and action type lookup. |
| `PDFActionRemoteGoTo.h` | ✅ implemented | `PdfActionRemoteGoTo` covers construction plus page index, point, URL, and action type accessors. |
| `PDFActionResetForm.h` | ✅ implemented | `PdfActionResetForm` covers construction, field inspection/mutation, clear-all helpers, and action type lookup. |
| `PDFBorder.h` | ✅ implemented | `PdfBorder` covers style, line width, dash pattern, and info inspection. |
| `PDFDestination.h` | ✅ implemented | `PdfDestination` covers construction, page access, zoom updates, info inspection, comparison, and `UNSPECIFIED_VALUE`. |
| `PDFAppearanceCharacteristics.h` | ✅ implemented | `PdfAppearanceCharacteristics` covers control type, colors, captions, rotation, and info inspection. |
| `PDFPageOverlayViewProvider.h` | ✅ implemented | `PdfPageOverlayView`, `PdfPageOverlayViewProvider`, and `PdfView::set_page_overlay_view_provider` cover the overlay-provider protocol surface. |
| `PDFAccessibilityNode` | ⏭️ skipped | PDFKit only forward-declares `PDFAccessibilityNode` in `PDFPage.h`; there is no public header interface to bind. v0.2.2 exposes a status shim (`PdfAccessibilityNode`) instead. |
| `PDFAnnotationButtonWidget.h` | ⚪ exempt | Deprecated on macOS; the audit treats the deprecated subclass headers as exempt and the safe API relies on base `PdfAnnotation` plus `PdfAppearanceCharacteristics`. |
| `PDFAnnotationChoiceWidget.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationCircle.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationFreeText.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationInk.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationLine.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationLink.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationMarkup.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationPopup.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationSquare.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationStamp.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationText.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFAnnotationTextWidget.h` | ⚪ exempt | Deprecated on macOS; exempt in the audit. |
| `PDFKit.h` | ⏭️ skipped | Umbrella header only. |
| `PDFKitPlatform.h` | ⏭️ skipped | Platform typedefs/macros only. |
| `PDFKit.apinotes` | ⏭️ skipped | API-notes metadata only. |
