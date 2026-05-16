# PDFKit v0.2.0 coverage audit

Audit source: `PDFKit.framework/Headers/` from the active macOS SDK (`xcrun --sdk macosx --show-sdk-path`).

This crate's v0.2.0 target is the requested headless-safe/document-editing surface. Rows marked `🟡 partial` or `⏭️ skipped` are intentionally deferred and called out explicitly.

| Header / surface | Status | Notes |
| --- | --- | --- |
| `PDFDocument.h` | ✅ implemented | `PdfDocument` covers creation/loading, metadata, page access, page mutation, outline roots, document selections, unlock, and write-to-URL. `exchange_pages` defensively rejects synthetic in-memory pages to avoid a framework exception. |
| `PDFPage.h` | ✅ implemented | `PdfPage` covers creation, bounds/rotation, annotation access/mutation, text geometry, and range/point-based selections. |
| `PDFAnnotation.h` | ✅ implemented | `PdfAnnotation` covers creation, metadata, contents, highlight state, border association, and URL/GoTo action attachment. |
| `PDFOutline.h` | ✅ implemented | `PdfOutline` covers creation, labels, children, open state, destinations, URL actions, and GoTo actions. |
| `PDFSelection.h` | ✅ implemented | `PdfSelection` covers creation, pages, bounds, text ranges, line splitting, add-selection, and extension helpers. |
| `PDFView.h` | ✅ implemented | `PdfView` covers document association, display/scaling state, navigation, current selection, and visible page access. |
| `PDFThumbnailView.h` | ✅ implemented | `PdfThumbnailView` covers `pdfView` association, thumbnail sizing, column count, dragging/multi-select flags, and selected page access. |
| `PDFActionURL.h` | ✅ implemented | `PdfActionUrl` covers construction, URL get/set, and action type lookup. |
| `PDFActionGoTo.h` | ✅ implemented | `PdfActionGoTo` covers construction, destination get/set, and action type lookup. |
| `PDFBorder.h` | ✅ implemented | `PdfBorder` covers style, line width, dash pattern, and info inspection. |
| `PDFDestination.h` | ✅ implemented | `PdfDestination` covers construction, page access, zoom updates, info inspection, and comparison. |
| `PDFAppearanceCharacteristics.h` | ✅ implemented | `PdfAppearanceCharacteristics` covers control type, colors, captions, rotation, and info inspection. |
| `PDFAccessibilityNode` | ⏭️ skipped | PDFKit only forward-declares `PDFAccessibilityNode` in `PDFPage.h`; there is no public header interface to bind. v0.2.0 exposes a status shim (`PdfAccessibilityNode`) instead. |
| `PDFAction.h` | 🟡 partial | Concrete URL and GoTo actions expose `action_type()`, but the abstract `PDFAction` base class is not wrapped as a standalone Rust handle. |
| `PDFActionNamed.h` | ⏭️ skipped | Named viewer actions are UI-driven and were outside the requested v0.2.0 surface. |
| `PDFActionRemoteGoTo.h` | ⏭️ skipped | Remote-document navigation is deferred; `PdfView` already surfaces local `PdfDestination` navigation. |
| `PDFActionResetForm.h` | ⏭️ skipped | Form-reset actions are deferred until widget/form helpers are expanded. |
| `PDFAnnotationButtonWidget.h` | ⏭️ skipped | Specialized widget subclass helpers are deferred; v0.2.0 uses base `PdfAnnotation` plus `PdfAppearanceCharacteristics`. |
| `PDFAnnotationChoiceWidget.h` | ⏭️ skipped | Choice-widget subclass helpers are deferred. |
| `PDFAnnotationCircle.h` | ⏭️ skipped | Geometry subclass sugar is deferred; base `PdfAnnotation` is available. |
| `PDFAnnotationFreeText.h` | ⏭️ skipped | Free-text subclass sugar is deferred; base `PdfAnnotation` is available. |
| `PDFAnnotationInk.h` | ⏭️ skipped | Ink-path helpers from `PDFAnnotationUtilities.h` are deferred. |
| `PDFAnnotationLine.h` | ⏭️ skipped | Line subclass helpers are deferred. |
| `PDFAnnotationLink.h` | ⏭️ skipped | Link subclass sugar is deferred; base `PdfAnnotation` + `PdfActionUrl` / `PdfActionGoTo` covers headless usage. |
| `PDFAnnotationMarkup.h` | ⏭️ skipped | Markup subclass helpers are deferred. |
| `PDFAnnotationPopup.h` | ⏭️ skipped | Popup annotation subclass helpers are deferred. |
| `PDFAnnotationSquare.h` | ⏭️ skipped | Square subclass helpers are deferred. |
| `PDFAnnotationStamp.h` | ⏭️ skipped | Stamp subclass helpers are deferred. |
| `PDFAnnotationText.h` | ⏭️ skipped | Text-note subclass helpers are deferred. |
| `PDFAnnotationTextWidget.h` | ⏭️ skipped | Text-widget subclass helpers are deferred. |
| `PDFAnnotationUtilities.h` | 🟡 partial | v0.2.0 covers `PDFWidgetControlType`, borders, destinations, actions, and appearance characteristics, but not the full category-property surface. |
| `PDFPageOverlayViewProvider.h` | ⏭️ skipped | Overlay-provider delegates are window/UI specific and not headless-safe. |
| `PDFKit.h` | ⏭️ skipped | Umbrella header only. |
| `PDFKitPlatform.h` | ⏭️ skipped | Platform typedefs/macros only. |
| `PDFKit.apinotes` | ⏭️ skipped | API-notes metadata only. |
