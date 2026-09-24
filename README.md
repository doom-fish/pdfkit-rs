# pdfkit-rs

Safe Rust bindings for Apple's [PDFKit](https://developer.apple.com/documentation/pdfkit) framework on macOS. The published Cargo package is `pdfkit-rs`; the Rust library target is `pdfkit`.

> **Status:** 0.4.0 is a soundness release. Document delegates must be `Send` and are never aliased across callbacks, the async find stream searches its own copy of the document, passwords are redacted and wiped on the Rust side, and out-of-range indexes no longer abort the process. See the [CHANGELOG](CHANGELOG.md) for the breaking changes.

## Installation

```toml
[dependencies]
pdfkit-rs = "0.4"
```

## Requirements

- macOS 11 or later. Some APIs need macOS 12, 13, 13.4 or 15; on older systems the bridge skips those write options or returns `None` or an error.
- Xcode or the Swift toolchain: `build.rs` builds the Swift bridge with `swift build`.

## Threading and callbacks

- PDFKit can call a `PdfDocumentDelegate` from a background queue, so delegates must be `Send`. Callbacks from different threads are serialized. A callback that PDFKit makes while the same thread is already inside the delegate (for example `unlock()` called from a delegate method) is not delivered, and class-name queries fall back to the default class.
- After a `PdfDocumentDelegateHandle` is dropped, callbacks that PDFKit has not started yet no longer reach the delegate.
- `PdfView`, `PdfThumbnailView`, `PdfPageOverlayView` and the view delegate and overlay provider handles belong to the main thread: their constructors return an error on any other thread, and the wrappers are `!Send`. View delegates and overlay providers run on the main thread and may hold UI handles; a callback that re-enters the same delegate is not delivered and PDFKit gets the default answer.
- Character indexes and ranges (`number_of_characters`, `selection_for_range`, `selection_from_page_characters`, `character_index_at_point`) count UTF-16 code units, not bytes or `char`s of `PdfPage::string`.
- `PdfDocumentWriteOptions` prints `<redacted>` for passwords and wipes the Rust-side copies it hands to PDFKit. The copies that Swift and PDFKit make cannot be wiped from Rust.

## Highlights

- `PdfDocument` for loading, creating, mutating, saving, attaching delegates, option-driven writes, and granularity-aware selections
- `PdfPage` for bounds, text, character geometry, annotations, selections, and image-backed page creation
- `PdfAction`, `PdfActionResetForm`, `PdfActionUrl`, `PdfActionGoTo`, `PdfActionNamed`, and `PdfActionRemoteGoTo` for abstract and concrete action inspection/editing
- `PdfAnnotation`, `PdfOutline`, `PdfBorder`, `PdfDestination`, and typed annotation key/subtype/icon wrappers for navigation and form metadata
- `PdfDocumentDelegate`, `PdfViewDelegate`, `PdfPageOverlayViewProvider`, `PdfDocumentWriteOptions`, and structured `PDFDocument` / `PDFView` / `PDFThumbnailView` notification names
- `PdfAppearanceCharacteristics`, `PdfPrintScalingMode`, `PdfTextAnnotationIconType`, `PdfThumbnailLayoutMode`, `PdfWidgetCellState`, and `PdfAccessibilityNode::public_api_available()`

## Quick start

```rust,no_run
use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let document = PdfDocument::from_url("document.pdf")?;
    let page = document
        .page(0)
        .ok_or_else(|| std::io::Error::other("missing first page"))?;
    let destination = PdfDestination::new(&page, PdfPoint { x: 36.0, y: 72.0 })?;
    let action = PdfActionGoTo::new(&destination)?;

    println!("pages={}", document.page_count());
    println!("first page text={:?}", page.string());
    println!("goto action type={:?}", action.action_type());
    Ok(())
}
```

## Surface overview

### Documents, pages, and selections

- `PdfDocument::new`, `from_url`, `from_bytes`, `write_to_url`, `write_to_url_with_options`
- `PdfDocument::insert_page`, `remove_page`, `exchange_pages`, `set_outline_root`, `set_delegate`, `unlock`
- `PdfDocument::selection_for_entire_document`, `selection_from_page_points`, `selection_from_page_points_with_granularity`, `selection_from_page_characters`
- `PdfPage::new`, `from_image_data`, `bounds`, `set_bounds`, `rotation`, `set_rotation`
- `PdfPageImageInitializationOptions::{new, with_media_box, with_rotation, with_upscale_if_smaller, with_compression_quality}`
- `PdfPage::selection_for_range`, `selection_for_rect`, `selection_for_word_at_point`, `selection_for_line_at_point`
- `PdfSelection::new`, `text_range`, `selections_by_line`, `add_selection`

### Actions, outlines, and annotations

- `PdfAction::action_type`, `as_url`, `as_goto`, `as_named`, `as_remote_goto`, `as_reset_form`
- `PdfActionResetForm::new`, `fields`, `set_fields`, `clear_fields`, `fields_included_are_cleared`, `set_fields_included_are_cleared`, `action_type`
- `PdfActionUrl::new`, `url`, `set_url`, `action_type`
- `PdfActionGoTo::new`, `destination`, `set_destination`, `action_type`
- `PdfActionNamed::new`, `name`, `set_name`, `action_type`
- `PdfActionRemoteGoTo::new`, `page_index`, `set_page_index`, `point`, `set_point`, `url`, `set_url`, `action_type`
- `PdfOutline::new`, `insert_child`, `set_label`, `set_destination`, `action`, `set_action`, `clear_action`
- `PdfAnnotation::new`, `new_with_subtype`, `info`, `set_contents`, `set_border`, `action`, `set_action`, `clear_action`
- `PdfAnnotationKey`, `PdfAnnotationHighlightingMode`, `PdfAnnotationLineEndingStyle`, `PdfAnnotationSubtype`, `PdfAnnotationTextIconName`, `PdfAnnotationWidgetSubtype`
- `PdfBorder::new`, `info`, `set_style`, `set_line_width`, `set_dash_pattern`
- `PdfDestination::new`, `info`, `page`, `set_zoom`, `compare`

### Delegates, notifications, and utilities

- `PdfDocumentDelegate`, `PdfDocumentDelegateHandle`, `PdfViewDelegate`, `PdfViewDelegateHandle`, and `async_api::PdfDocumentFindStream`
- `PdfPageOverlayViewProvider`, `PdfPageOverlayViewProviderHandle`, `PdfPageOverlayView`
- `PdfDocumentWriteOptions::{with_owner_password, with_user_password, with_access_permissions, with_burn_in_annotations, with_save_text_from_ocr, with_save_images_as_jpeg, with_optimize_images_for_screen}`
- `PdfDocumentNotification`, `PdfDocumentNotificationUserInfoKey`, `PdfViewNotification`, `PdfThumbnailViewNotification`
- `PdfActionNamedName`, `PdfLineStyle`, `PdfMarkupType`, `PdfPrintScalingMode`, `PdfSelectionGranularity`, `PdfTextAnnotationIconType`, `PdfThumbnailLayoutMode`, `PdfWidgetCellState`

### View state, widget appearance, and accessibility status

- `PdfView::new`, `set_document`, `set_delegate`, `set_page_overlay_view_provider`, `set_display_mode`, `set_display_direction`, `set_display_box`
- `PdfView::set_current_selection`, `go_to_page`, `go_to_destination`, `visible_pages`, `area_of_interest_for_point`
- `PdfThumbnailView::new`, `set_pdf_view`, `set_thumbnail_size`, `set_maximum_number_of_columns`
- `PdfAppearanceCharacteristics::new`, `set_control_type`, `set_caption`, `set_background_color`, `info`
- `PdfAreaOfInterest`, `PdfDestination::UNSPECIFIED_VALUE`, `PdfAccessibilityNode::public_api_available`, `PdfAccessibilityNode::availability_note`

## Async document find

Enable the `async` Cargo feature to use `pdfkit::async_api::PdfDocumentFindStream`. It copies the document when the search starts, runs `PDFDocument.findString(_:withOptions:)` on that copy on a background dispatch queue, and exposes owned match snapshots through an executor-agnostic bounded async stream. Every match is delivered: once `capacity` events are buffered, the search waits for the consumer. Dropping the stream does not wait for the search.

```toml
pdfkit-rs = { version = "0.4", features = ["async"] }
```

## Examples

The crate now ships smoke examples covering the core logical areas:

- `01_document_smoke`
- `02_page_basics`
- `03_annotation_link`
- `04_outline_tree`
- `05_selection_ranges`
- `06_view_state`
- `07_thumbnail_view_state`
- `08_action_url`
- `09_action_goto`
- `10_border_style`
- `11_destination_compare`
- `12_appearance_characteristics`
- `13_accessibility_node_status`
- `14_action_named_remote_goto`
- `15_document_delegate_write_options`
- `16_async_find_stream` *(requires `--features async`)*

Run one example:

```bash
cargo run --example 06_view_state
```

Run all examples:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the header audit and [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) for the symbol-level report. Its 100% figure counts top-level declarations of MacOSX26.2.sdk that have a wrapper; it does not verify every method.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
