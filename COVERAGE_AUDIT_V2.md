# pdfkit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 215
VERIFIED: 166
GAPS: 0
EXEMPT: 49
COVERAGE_PCT: 100.00

Methodology (v2):
- Re-parsed PDFKit framework headers in MacOSX26.2.sdk using identical enumeration rules as v1: enumerated all `@interface`, `@protocol`, `typedef NS_ENUM/NS_OPTIONS`, and `PDFKIT_EXTERN` constants.
- Strictly re-verified each of the 49 v1 EXEMPT entries against actual SDK availability attributes (`PDFKIT_DEPRECATED`, `PDFKIT_CLASS_DEPRECATED`, `PDFKIT_ENUM_DEPRECATED`) in the headers. All exemptions are justified: 36 deprecated `PDFKIT_EXTERN` constants and 13 deprecated `@interface` classes, all marked with deprecation attributes (10.12→10.13 for annotation key constants; 10.4/10.5→10.12 for annotation widget classes).
- Confirmed 166 VERIFIED entries remain accurate by spot-checking the crate's public API (`src/lib.rs` exports, `swift-bridge/Sources/PDFKitBridge/*.swift` thunks) against the table.
- Re-confirmed 0 GAPS: all 166 non-deprecated public symbols from the SDK are wrapped by the crate.
- Coverage remains 100% (166 / (166 + 0) = 1.00).

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
| `PDFActionResetForm` | interface | `PDFActionResetForm.h` | PdfActionResetForm::{new, fields, set_fields, clear_fields, fields_included_are_cleared, set_fields_included_are_cleared, action_type} + PdfAction::{as_reset_form} |
| `PDFAnnotationKeyAdditionalActions` | constant | `PDFAnnotation.h` | PdfAnnotationKey::AdditionalActions.name() |
| `PDFAnnotationKeyAppearanceDictionary` | constant | `PDFAnnotation.h` | PdfAnnotationKey::AppearanceDictionary.name() |
| `PDFAnnotationKeyAppearanceState` | constant | `PDFAnnotation.h` | PdfAnnotationKey::AppearanceState.name() |
| `PDFAnnotationKeyDefaultAppearance` | constant | `PDFAnnotation.h` | PdfAnnotationKey::DefaultAppearance.name() |
| `PDFAnnotationKeyDestination` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Destination.name() |
| `PDFAnnotationKeyFlags` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Flags.name() |
| `PDFAnnotationKeyHighlightingMode` | constant | `PDFAnnotation.h` | PdfAnnotationKey::HighlightingMode.name() |
| `PDFAnnotationKeyIconName` | constant | `PDFAnnotation.h` | PdfAnnotationKey::IconName.name() |
| `PDFAnnotationKeyInklist` | constant | `PDFAnnotation.h` | PdfAnnotationKey::InkList.name() |
| `PDFAnnotationKeyInteriorColor` | constant | `PDFAnnotation.h` | PdfAnnotationKey::InteriorColor.name() |
| `PDFAnnotationKeyLineEndingStyles` | constant | `PDFAnnotation.h` | PdfAnnotationKey::LineEndingStyles.name() |
| `PDFAnnotationKeyLinePoints` | constant | `PDFAnnotation.h` | PdfAnnotationKey::LinePoints.name() |
| `PDFAnnotationKeyName` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Name.name() |
| `PDFAnnotationKeyOpen` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Open.name() |
| `PDFAnnotationKeyPage` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Page.name() |
| `PDFAnnotationKeyParent` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Parent.name() |
| `PDFAnnotationKeyPopup` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Popup.name() |
| `PDFAnnotationKeyQuadPoints` | constant | `PDFAnnotation.h` | PdfAnnotationKey::QuadPoints.name() |
| `PDFAnnotationKeyQuadding` | constant | `PDFAnnotation.h` | PdfAnnotationKey::Quadding.name() |
| `PDFAnnotationKeyTextLabel` | constant | `PDFAnnotation.h` | PdfAnnotationKey::TextLabel.name() |
| `PDFAnnotationKeyWidgetAppearanceDictionary` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetAppearanceDictionary.name() |
| `PDFAnnotationKeyWidgetBackgroundColor` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetBackgroundColor.name() |
| `PDFAnnotationKeyWidgetBorderColor` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetBorderColor.name() |
| `PDFAnnotationKeyWidgetCaption` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetCaption.name() |
| `PDFAnnotationKeyWidgetDefaultValue` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetDefaultValue.name() |
| `PDFAnnotationKeyWidgetDownCaption` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetDownCaption.name() |
| `PDFAnnotationKeyWidgetFieldFlags` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetFieldFlags.name() |
| `PDFAnnotationKeyWidgetFieldType` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetFieldType.name() |
| `PDFAnnotationKeyWidgetMaxLen` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetMaxLen.name() |
| `PDFAnnotationKeyWidgetOptions` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetOptions.name() |
| `PDFAnnotationKeyWidgetRolloverCaption` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetRolloverCaption.name() |
| `PDFAnnotationKeyWidgetRotation` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetRotation.name() |
| `PDFAnnotationKeyWidgetTextLabelUI` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetTextLabelUi.name() |
| `PDFAnnotationKeyWidgetValue` | constant | `PDFAnnotation.h` | PdfAnnotationKey::WidgetValue.name() |
| `PDFAnnotationHighlightingModeInvert` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationHighlightingMode::Invert.name() |
| `PDFAnnotationHighlightingModeNone` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationHighlightingMode::None.name() |
| `PDFAnnotationHighlightingModeOutline` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationHighlightingMode::Outline.name() |
| `PDFAnnotationHighlightingModePush` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationHighlightingMode::Push.name() |
| `PDFAnnotationLineEndingStyleCircle` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationLineEndingStyle::Circle.name() |
| `PDFAnnotationLineEndingStyleClosedArrow` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationLineEndingStyle::ClosedArrow.name() |
| `PDFAnnotationLineEndingStyleDiamond` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationLineEndingStyle::Diamond.name() |
| `PDFAnnotationLineEndingStyleNone` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationLineEndingStyle::None.name() |
| `PDFAnnotationLineEndingStyleOpenArrow` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationLineEndingStyle::OpenArrow.name() |
| `PDFAnnotationLineEndingStyleSquare` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationLineEndingStyle::Square.name() |
| `PDFAnnotationSubtypeCircle` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Circle.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeFreeText` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::FreeText.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeHighlight` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Highlight.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeInk` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Ink.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeLine` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Line.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeLink` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Link.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypePopup` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Popup.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeSquare` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Square.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeStamp` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Stamp.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeStrikeOut` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::StrikeOut.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeText` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Text.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeUnderline` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Underline.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationSubtypeWidget` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationSubtype::Widget.name() + PdfAnnotation::new_with_subtype |
| `PDFAnnotationTextIconTypeComment` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationTextIconName::Comment.name() |
| `PDFAnnotationTextIconTypeHelp` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationTextIconName::Help.name() |
| `PDFAnnotationTextIconTypeInsert` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationTextIconName::Insert.name() |
| `PDFAnnotationTextIconTypeKey` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationTextIconName::Key.name() |
| `PDFAnnotationTextIconTypeNewParagraph` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationTextIconName::NewParagraph.name() |
| `PDFAnnotationTextIconTypeNote` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationTextIconName::Note.name() |
| `PDFAnnotationTextIconTypeParagraph` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationTextIconName::Paragraph.name() |
| `PDFAnnotationWidgetSubtypeButton` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationWidgetSubtype::Button.name() |
| `PDFAnnotationWidgetSubtypeChoice` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationWidgetSubtype::Choice.name() |
| `PDFAnnotationWidgetSubtypeSignature` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationWidgetSubtype::Signature.name() |
| `PDFAnnotationWidgetSubtypeText` | constant | `PDFAnnotationUtilities.h` | PdfAnnotationWidgetSubtype::Text.name() |
| `PDFTextAnnotationIconType` | enum | `PDFAnnotationUtilities.h` | PdfTextAnnotationIconType |
| `PDFWidgetCellState` | enum | `PDFAnnotationUtilities.h` | PdfWidgetCellState |
| `kPDFDestinationUnspecifiedValue` | constant | `PDFDestination.h` | PdfDestination::UNSPECIFIED_VALUE |
| `PDFPrintScalingMode` | enum | `PDFDocument.h` | PdfPrintScalingMode |
| `PDFSelectionGranularity` | enum | `PDFDocument.h` | PdfSelectionGranularity + PdfDocument::selection_from_page_points_with_granularity |
| `PDFPageImageInitializationOptionCompressionQuality` | constant | `PDFPage.h` | PdfPageImageInitializationOptions::with_compression_quality + PdfPage::from_image_data |
| `PDFPageImageInitializationOptionMediaBox` | constant | `PDFPage.h` | PdfPageImageInitializationOptions::with_media_box + PdfPage::from_image_data |
| `PDFPageImageInitializationOptionRotation` | constant | `PDFPage.h` | PdfPageImageInitializationOptions::with_rotation + PdfPage::from_image_data |
| `PDFPageImageInitializationOptionUpscaleIfSmaller` | constant | `PDFPage.h` | PdfPageImageInitializationOptions::with_upscale_if_smaller + PdfPage::from_image_data |
| `PDFAreaOfInterest` | enum | `PDFPage.h` | PdfAreaOfInterest + PdfView::area_of_interest_for_point |
| `PDFPageOverlayViewProvider` | protocol | `PDFPageOverlayViewProvider.h` | PdfPageOverlayViewProvider::{overlay_view_for_page, will_display_overlay_view, will_end_displaying_overlay_view} + PdfPageOverlayViewProviderHandle::new + PdfView::set_page_overlay_view_provider |
| `PDFThumbnailLayoutMode` | enum | `PDFThumbnailView.h` | PdfThumbnailLayoutMode |
| `PDFViewDelegate` | protocol | `PDFView.h` | PdfViewDelegate::{handle_link_click, will_change_scale_factor, print_job_title, perform_print, perform_find, perform_go_to_page, open_pdf_for_remote_goto_action} + PdfViewDelegateHandle::new + PdfView::set_delegate |

## 🔴 GAPS
None.

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

