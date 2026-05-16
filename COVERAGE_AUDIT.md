# pdfkit-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 215
VERIFIED: 84
GAPS: 82
EXEMPT: 49
COVERAGE_PCT: 50.60

Methodology:
- Parsed top-level Objective-C declarations from `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.2.sdk/System/Library/Frameworks/PDFKit.framework/Headers`: `@interface`, `@protocol`, `typedef NS_ENUM/NS_OPTIONS`, and exported constants.
- Counted deprecated macOS declarations as **EXEMPT** per the audit instructions.
- Counted a symbol as **VERIFIED** only when `swift-bridge/Sources/PDFKitBridge/*.swift` and the public `src/*.rs` API expose a direct safe wrapper or an obvious structured equivalent, including typed notification-name enums and `PdfDocumentWriteOptions` + `PdfDocument::write_to_url_with_options` for grouped write-option constants.
- `PDFAccessibilityNode` is not counted because PDFKit only forward-declares it; there is no public header interface to audit.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `PDFAction` | interface | `PDFAction.h` | PdfAction::{action_type, as_url, as_goto, as_named, as_remote_goto} |
| `PDFActionGoTo` | interface | `PDFActionGoTo.h` | PdfActionGoTo::{new, destination, set_destination, action_type} |
| `PDFActionURL` | interface | `PDFActionURL.h` | PdfActionUrl::{new, url, set_url, action_type} |
| `PDFActionNamedName` | enum | `PDFActionNamed.h` | PdfActionNamedName |
| `PDFActionNamed` | interface | `PDFActionNamed.h` | PdfActionNamed::{new, name, set_name, action_type} |
| `PDFActionRemoteGoTo` | interface | `PDFActionRemoteGoTo.h` | PdfActionRemoteGoTo::{new, page_index, set_page_index, point, set_point, url, set_url, action_type} |
| `PDFAnnotationKeyAction` | constant | `PDFAnnotation.h` | PdfAnnotation::{action, set_action, clear_action, action_url, action_goto, set_action_url, set_action_goto} |
| `PDFAnnotationKeyBorder` | constant | `PDFAnnotation.h` | PdfAnnotation::{border, set_border} |
| `PDFAnnotationKeyBorderStyle` | constant | `PDFAnnotation.h` | PdfBorder::{info, set_style} |
| `PDFAnnotationKeyColor` | constant | `PDFAnnotation.h` | PdfAnnotationInfo::color |
| `PDFAnnotationKeyContents` | constant | `PDFAnnotation.h` | PdfAnnotation::{info, set_contents} |
| `PDFAnnotationKeyDate` | constant | `PDFAnnotation.h` | PdfAnnotationInfo::modification_date |
| `PDFAnnotationKeyRect` | constant | `PDFAnnotation.h` | PdfAnnotation::new / PdfAnnotationInfo::bounds |
| `PDFAnnotationKeySubtype` | constant | `PDFAnnotation.h` | PdfAnnotation::new / PdfAnnotationInfo::annotation_type |
| `PDFAnnotation` | interface | `PDFAnnotation.h` | PdfAnnotation::{new, info, set_contents, set_highlighted, border, set_border, action, set_action, clear_action, action_url, set_action_url, action_goto, set_action_goto} |
| `PDFWidgetControlType` | enum | `PDFAnnotationUtilities.h` | PdfWidgetControlType |
| `PDFLineStyle` | enum | `PDFAnnotationUtilities.h` | PdfLineStyle |
| `PDFMarkupType` | enum | `PDFAnnotationUtilities.h` | PdfMarkupType |
| `PDFAppearanceCharacteristicsKeyBackgroundColor` | constant | `PDFAppearanceCharacteristics.h` | PdfAppearanceCharacteristics::{info, set_background_color} |
| `PDFAppearanceCharacteristicsKeyBorderColor` | constant | `PDFAppearanceCharacteristics.h` | PdfAppearanceCharacteristics::{info, set_border_color} |
| `PDFAppearanceCharacteristicsKeyCaption` | constant | `PDFAppearanceCharacteristics.h` | PdfAppearanceCharacteristics::{info, set_caption} |
| `PDFAppearanceCharacteristicsKeyDownCaption` | constant | `PDFAppearanceCharacteristics.h` | PdfAppearanceCharacteristics::{info, set_down_caption} |
| `PDFAppearanceCharacteristicsKeyRolloverCaption` | constant | `PDFAppearanceCharacteristics.h` | PdfAppearanceCharacteristics::{info, set_rollover_caption} |
| `PDFAppearanceCharacteristicsKeyRotation` | constant | `PDFAppearanceCharacteristics.h` | PdfAppearanceCharacteristics::{info, set_rotation} |
| `PDFAppearanceCharacteristics` | interface | `PDFAppearanceCharacteristics.h` | PdfAppearanceCharacteristics::{new, info, set_control_type, set_rotation, set_caption, set_rollover_caption, set_down_caption, set_background_color, set_border_color} |
| `PDFBorderKeyDashPattern` | constant | `PDFBorder.h` | PdfBorder::{info, set_dash_pattern} |
| `PDFBorderKeyLineWidth` | constant | `PDFBorder.h` | PdfBorder::{info, set_line_width} |
| `PDFBorderKeyStyle` | constant | `PDFBorder.h` | PdfBorder::{info, set_style} |
| `PDFBorderStyle` | enum | `PDFBorder.h` | PdfBorderStyle |
| `PDFBorder` | interface | `PDFBorder.h` | PdfBorder::{new, info, set_style, set_line_width, set_dash_pattern} |
| `PDFDestination` | interface | `PDFDestination.h` | PdfDestination::{new, info, page, set_zoom, compare} |
| `PDFDocumentAuthorAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::author |
| `PDFDocumentCreationDateAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::creation_date |
| `PDFDocumentCreatorAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::creator |
| `PDFDocumentKeywordsAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::keywords |
| `PDFDocumentModificationDateAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::modification_date |
| `PDFDocumentProducerAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::producer |
| `PDFDocumentSubjectAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::subject |
| `PDFDocumentTitleAttribute` | constant | `PDFDocument.h` | PdfDocument::attributes / PdfDocumentAttributes::title |
| `PDFAccessPermissions` | enum | `PDFDocument.h` | PdfDocumentInfo::{access_permissions, allows_printing, allows_copying, allows_document_changes, allows_document_assembly, allows_content_accessibility, allows_commenting, allows_form_field_entry} |
| `PDFDocumentPermissions` | enum | `PDFDocument.h` | PdfDocumentPermissions / PdfDocumentInfo::permissions_status_enum |
| `PDFDocumentOwnerPasswordOption` | constant | `PDFDocument.h` | PdfDocumentWriteOptions::with_owner_password + PdfDocument::write_to_url_with_options |
| `PDFDocumentUserPasswordOption` | constant | `PDFDocument.h` | PdfDocumentWriteOptions::with_user_password + PdfDocument::write_to_url_with_options |
| `PDFDocumentAccessPermissionsOption` | constant | `PDFDocument.h` | PdfDocumentWriteOptions::with_access_permissions + PdfDocument::write_to_url_with_options |
| `PDFDocumentBurnInAnnotationsOption` | constant | `PDFDocument.h` | PdfDocumentWriteOptions::with_burn_in_annotations + PdfDocument::write_to_url_with_options |
| `PDFDocumentSaveTextFromOCROption` | constant | `PDFDocument.h` | PdfDocumentWriteOptions::with_save_text_from_ocr + PdfDocument::write_to_url_with_options |
| `PDFDocumentSaveImagesAsJPEGOption` | constant | `PDFDocument.h` | PdfDocumentWriteOptions::with_save_images_as_jpeg + PdfDocument::write_to_url_with_options |
| `PDFDocumentOptimizeImagesForScreenOption` | constant | `PDFDocument.h` | PdfDocumentWriteOptions::with_optimize_images_for_screen + PdfDocument::write_to_url_with_options |
| `PDFDocumentDidUnlockNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidUnlock.name() |
| `PDFDocumentDidBeginFindNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidBeginFind.name() |
| `PDFDocumentDidEndFindNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidEndFind.name() |
| `PDFDocumentDidBeginPageFindNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidBeginPageFind.name() |
| `PDFDocumentDidEndPageFindNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidEndPageFind.name() |
| `PDFDocumentDidFindMatchNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidFindMatch.name() |
| `PDFDocumentDidBeginWriteNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidBeginWrite.name() |
| `PDFDocumentDidEndWriteNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidEndWrite.name() |
| `PDFDocumentDidBeginPageWriteNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidBeginPageWrite.name() |
| `PDFDocumentDidEndPageWriteNotification` | constant | `PDFDocument.h` | PdfDocumentNotification::DidEndPageWrite.name() |
| `PDFDocumentFoundSelectionKey` | constant | `PDFDocument.h` | PdfDocumentNotificationUserInfoKey::FoundSelection.name() |
| `PDFDocumentPageIndexKey` | constant | `PDFDocument.h` | PdfDocumentNotificationUserInfoKey::PageIndex.name() |
| `PDFDocument` | interface | `PDFDocument.h` | PdfDocument::{new, from_url, from_bytes, info, attributes, string, page_count, page, pages, page_index, outline_root, set_outline_root, outline_item_for_selection, selection_for_entire_document, selection_from_page_points, selection_from_page_characters, unlock, set_delegate, write_to_url, write_to_url_with_options, insert_page, remove_page, exchange_pages} |
| `PDFDocumentDelegate` | protocol | `PDFDocument.h` | PdfDocumentDelegate::{handle_notification, did_match_string, page_class_name, annotation_class_name} + PdfDocumentDelegateHandle::new + PdfDocument::set_delegate |
| `PDFOutline` | interface | `PDFOutline.h` | PdfOutline::{new, label, set_label, child_count, child, children, insert_child, remove_from_parent, index, is_open, set_open, destination, set_destination, action, set_action, clear_action, action_url, set_action_url, action_goto, set_action_goto, parent} |
| `PDFDisplayBox` | enum | `PDFPage.h` | DisplayBox / PdfPage::{bounds, set_bounds} / PdfView::set_display_box |
| `PDFPage` | interface | `PDFPage.h` | PdfPage::{new, label, string, number_of_characters, rotation, set_rotation, bounds, set_bounds, document, annotation_count, annotation, annotations, add_annotation, remove_annotation, annotation_at_point, selection_for_range, selection_for_rect, selection_for_word_at_point, selection_for_line_at_point, selection_from_points, character_bounds_at, character_index_at_point, displays_annotations, set_displays_annotations} |
| `PDFSelection` | interface | `PDFSelection.h` | PdfSelection::{new, string, page_count, page, pages, bounds_for_page, number_of_text_ranges_on_page, text_range, selection_by_line_count, selection_by_line, selections_by_line, add_selection, extend_selection_at_end, extend_selection_at_start, extend_selection_for_line_boundaries} |
| `PDFThumbnailView` | interface | `PDFThumbnailView.h` | PdfThumbnailView::{new, info, pdf_view, set_pdf_view, set_thumbnail_size, set_maximum_number_of_columns, set_allows_dragging, set_allows_multiple_selection, selected_page_count, selected_page, selected_pages} |
| `PDFThumbnailViewDocumentEditedNotification` | constant | `PDFThumbnailView.h` | PdfThumbnailViewNotification::DocumentEdited.name() |
| `PDFDisplayDirection` | enum | `PDFView.h` | PdfDisplayDirection / PdfView::{set_display_direction, info} |
| `PDFDisplayMode` | enum | `PDFView.h` | PdfDisplayMode / PdfView::{set_display_mode, info} |
| `PDFInterpolationQuality` | enum | `PDFView.h` | PdfInterpolationQuality / PdfView::info |
| `PDFView` | interface | `PDFView.h` | PdfView::{new, info, document, set_document, current_page, current_destination, current_selection, set_current_selection, clear_selection, go_to_page, go_to_destination, go_to_selection, set_display_mode, set_display_direction, set_display_box, set_auto_scales, set_scale_factor, set_min_scale_factor, set_max_scale_factor, layout_document_view, visible_page_count, visible_page, visible_pages} |
| `PDFViewAnnotationHitNotification` | constant | `PDFView.h` | PdfViewNotification::AnnotationHit.name() |
| `PDFViewAnnotationWillHitNotification` | constant | `PDFView.h` | PdfViewNotification::AnnotationWillHit.name() |
| `PDFViewChangedHistoryNotification` | constant | `PDFView.h` | PdfViewNotification::ChangedHistory.name() |
| `PDFViewCopyPermissionNotification` | constant | `PDFView.h` | PdfViewNotification::CopyPermission.name() |
| `PDFViewDisplayBoxChangedNotification` | constant | `PDFView.h` | PdfViewNotification::DisplayBoxChanged.name() |
| `PDFViewDisplayModeChangedNotification` | constant | `PDFView.h` | PdfViewNotification::DisplayModeChanged.name() |
| `PDFViewDocumentChangedNotification` | constant | `PDFView.h` | PdfViewNotification::DocumentChanged.name() |
| `PDFViewPageChangedNotification` | constant | `PDFView.h` | PdfViewNotification::PageChanged.name() |
| `PDFViewPrintPermissionNotification` | constant | `PDFView.h` | PdfViewNotification::PrintPermission.name() |
| `PDFViewScaleChangedNotification` | constant | `PDFView.h` | PdfViewNotification::ScaleChanged.name() |
| `PDFViewSelectionChangedNotification` | constant | `PDFView.h` | PdfViewNotification::SelectionChanged.name() |
| `PDFViewVisiblePagesChangedNotification` | constant | `PDFView.h` | PdfViewNotification::VisiblePagesChanged.name() |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `PDFActionResetForm` | interface | `PDFActionResetForm.h` | No Rust wrapper for this action subtype. |
| `PDFAnnotationKeyAdditionalActions` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyAppearanceDictionary` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyAppearanceState` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyDefaultAppearance` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyDestination` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyFlags` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyHighlightingMode` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyIconName` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyInklist` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyInteriorColor` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyLineEndingStyles` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyLinePoints` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyName` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyOpen` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyPage` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyParent` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyPopup` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyQuadPoints` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyQuadding` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyTextLabel` | constant | `PDFAnnotation.h` | Annotation-dictionary key constant is not surfaced as a Rust constant or dedicated accessor. |
| `PDFAnnotationKeyWidgetAppearanceDictionary` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetBackgroundColor` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetBorderColor` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetCaption` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetDefaultValue` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetDownCaption` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetFieldFlags` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetFieldType` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetMaxLen` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetOptions` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetRolloverCaption` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetRotation` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetTextLabelUI` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationKeyWidgetValue` | constant | `PDFAnnotation.h` | Widget annotation-dictionary key is not exposed through the safe Rust API. |
| `PDFAnnotationHighlightingModeInvert` | constant | `PDFAnnotationUtilities.h` | Highlighting-mode string constant is not exposed. |
| `PDFAnnotationHighlightingModeNone` | constant | `PDFAnnotationUtilities.h` | Highlighting-mode string constant is not exposed. |
| `PDFAnnotationHighlightingModeOutline` | constant | `PDFAnnotationUtilities.h` | Highlighting-mode string constant is not exposed. |
| `PDFAnnotationHighlightingModePush` | constant | `PDFAnnotationUtilities.h` | Highlighting-mode string constant is not exposed. |
| `PDFAnnotationLineEndingStyleCircle` | constant | `PDFAnnotationUtilities.h` | Line-ending string constant is not exposed. |
| `PDFAnnotationLineEndingStyleClosedArrow` | constant | `PDFAnnotationUtilities.h` | Line-ending string constant is not exposed. |
| `PDFAnnotationLineEndingStyleDiamond` | constant | `PDFAnnotationUtilities.h` | Line-ending string constant is not exposed. |
| `PDFAnnotationLineEndingStyleNone` | constant | `PDFAnnotationUtilities.h` | Line-ending string constant is not exposed. |
| `PDFAnnotationLineEndingStyleOpenArrow` | constant | `PDFAnnotationUtilities.h` | Line-ending string constant is not exposed. |
| `PDFAnnotationLineEndingStyleSquare` | constant | `PDFAnnotationUtilities.h` | Line-ending string constant is not exposed. |
| `PDFAnnotationSubtypeCircle` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeFreeText` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeHighlight` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeInk` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeLine` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeLink` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypePopup` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeSquare` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeStamp` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeStrikeOut` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeText` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeUnderline` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationSubtypeWidget` | constant | `PDFAnnotationUtilities.h` | Subtype string constant is not exposed; PdfAnnotation::new accepts raw subtype strings only. |
| `PDFAnnotationTextIconTypeComment` | constant | `PDFAnnotationUtilities.h` | Text-icon string constant is not exposed. |
| `PDFAnnotationTextIconTypeHelp` | constant | `PDFAnnotationUtilities.h` | Text-icon string constant is not exposed. |
| `PDFAnnotationTextIconTypeInsert` | constant | `PDFAnnotationUtilities.h` | Text-icon string constant is not exposed. |
| `PDFAnnotationTextIconTypeKey` | constant | `PDFAnnotationUtilities.h` | Text-icon string constant is not exposed. |
| `PDFAnnotationTextIconTypeNewParagraph` | constant | `PDFAnnotationUtilities.h` | Text-icon string constant is not exposed. |
| `PDFAnnotationTextIconTypeNote` | constant | `PDFAnnotationUtilities.h` | Text-icon string constant is not exposed. |
| `PDFAnnotationTextIconTypeParagraph` | constant | `PDFAnnotationUtilities.h` | Text-icon string constant is not exposed. |
| `PDFAnnotationWidgetSubtypeButton` | constant | `PDFAnnotationUtilities.h` | Widget-subtype string constant is not exposed. |
| `PDFAnnotationWidgetSubtypeChoice` | constant | `PDFAnnotationUtilities.h` | Widget-subtype string constant is not exposed. |
| `PDFAnnotationWidgetSubtypeSignature` | constant | `PDFAnnotationUtilities.h` | Widget-subtype string constant is not exposed. |
| `PDFAnnotationWidgetSubtypeText` | constant | `PDFAnnotationUtilities.h` | Widget-subtype string constant is not exposed. |
| `PDFTextAnnotationIconType` | enum | `PDFAnnotationUtilities.h` | Annotation-utility enum is not exposed as a Rust type. |
| `PDFWidgetCellState` | enum | `PDFAnnotationUtilities.h` | Annotation-utility enum is not exposed as a Rust type. |
| `kPDFDestinationUnspecifiedValue` | constant | `PDFDestination.h` | Destination zoom sentinel constant is not surfaced. |
| `PDFPrintScalingMode` | enum | `PDFDocument.h` | Printing APIs are not wrapped. |
| `PDFSelectionGranularity` | enum | `PDFDocument.h` | Search/find APIs that use selection granularity are not wrapped. |
| `PDFPageImageInitializationOptionCompressionQuality` | constant | `PDFPage.h` | Image-to-page initialization options are not wrapped. |
| `PDFPageImageInitializationOptionMediaBox` | constant | `PDFPage.h` | Image-to-page initialization options are not wrapped. |
| `PDFPageImageInitializationOptionRotation` | constant | `PDFPage.h` | Image-to-page initialization options are not wrapped. |
| `PDFPageImageInitializationOptionUpscaleIfSmaller` | constant | `PDFPage.h` | Image-to-page initialization options are not wrapped. |
| `PDFAreaOfInterest` | enum | `PDFPage.h` | Area-of-interest hit-testing APIs are not wrapped. |
| `PDFPageOverlayViewProvider` | protocol | `PDFPageOverlayViewProvider.h` | No delegate/protocol bridging in the Swift bridge or public Rust API. |
| `PDFThumbnailLayoutMode` | enum | `PDFThumbnailView.h` | Thumbnail layout-mode APIs are not wrapped. |
| `PDFViewDelegate` | protocol | `PDFView.h` | No delegate/protocol bridging in the Swift bridge or public Rust API. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `kPDFAnnotationKey_Action` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_AdditionalActions` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_AppearanceDictionary` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_AppearanceState` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Border` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_BorderStyle` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Color` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Contents` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Date` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_DefaultAppearance` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Destination` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Flags` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_HighlightingMode` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_IconName` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Inklist` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_InteriorColor` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_LineEndingStyles` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_LinePoints` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Name` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Open` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Page` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Parent` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Popup` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_QuadPoints` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Quadding` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Rect` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_Subtype` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_TextLabel` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetAppearanceDictionary` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetDefaultValue` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetFieldFlags` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetFieldType` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetMaxLen` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetOptions` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetTextLabelUI` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `kPDFAnnotationKey_WidgetValue` | constant | `PDFAnnotation.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_DEPRECATED(10_12, 10_13, NA, NA)` |
| `PDFAnnotationButtonWidget` | interface | `PDFAnnotationButtonWidget.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationChoiceWidget` | interface | `PDFAnnotationChoiceWidget.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationCircle` | interface | `PDFAnnotationCircle.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationFreeText` | interface | `PDFAnnotationFreeText.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationInk` | interface | `PDFAnnotationInk.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationLine` | interface | `PDFAnnotationLine.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationLink` | interface | `PDFAnnotationLink.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationMarkup` | interface | `PDFAnnotationMarkup.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationPopup` | interface | `PDFAnnotationPopup.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_ENUM_DEPRECATED(10_5, 10_12, NA, NA)` |
| `PDFAnnotationSquare` | interface | `PDFAnnotationSquare.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationStamp` | interface | `PDFAnnotationStamp.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_5, 10_12, NA, NA)` |
| `PDFAnnotationText` | interface | `PDFAnnotationText.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |
| `PDFAnnotationTextWidget` | interface | `PDFAnnotationTextWidget.h` | Deprecated on macOS; audit instructions mark deprecated SDK surface as exempt. | `PDFKIT_CLASS_DEPRECATED(10_4, 10_12, NA, NA)` |

