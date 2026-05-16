# pdfkit-rs

Safe Rust bindings for Apple's [PDFKit](https://developer.apple.com/documentation/pdfkit) framework on macOS. The published Cargo package is `pdfkit-rs`; the Rust library target is `pdfkit`.

> **Status:** v0.2.1 extends the Swift bridge and safe Rust API with generic `PdfAction` handles, named and remote-go-to actions, `PDFDocumentDelegate` bridging, structured document write options, notification-name enums, and raw annotation utility enums on top of the v0.2.0 document/view surface.

## Highlights

- `PdfDocument` for loading, creating, mutating, saving, attaching delegates, and option-driven writes
- `PdfPage` for bounds, text, character geometry, annotations, and selections
- `PdfAction`, `PdfActionUrl`, `PdfActionGoTo`, `PdfActionNamed`, and `PdfActionRemoteGoTo` for abstract and concrete action inspection/editing
- `PdfAnnotation`, `PdfOutline`, `PdfBorder`, and `PdfDestination` for generic action attachment, outline trees, and navigation targets
- `PdfDocumentDelegate`, `PdfDocumentWriteOptions`, `PdfDocumentNotification`, `PdfViewNotification`, and `PdfThumbnailViewNotification` for delegate-driven integration points and structured notification names
- `PdfAppearanceCharacteristics`, `PdfActionNamedName`, `PdfLineStyle`, and `PdfMarkupType` for widget and annotation utility metadata
- `PdfAccessibilityNode::public_api_available()` for reporting that PDFKit only forward-declares the type on macOS

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
- `PdfDocument::selection_for_entire_document`, `selection_from_page_points`, `selection_from_page_characters`
- `PdfPage::new`, `bounds`, `set_bounds`, `rotation`, `set_rotation`
- `PdfPage::selection_for_range`, `selection_for_rect`, `selection_for_word_at_point`, `selection_for_line_at_point`
- `PdfSelection::new`, `text_range`, `selections_by_line`, `add_selection`

### Actions, outlines, and annotations

- `PdfAction::action_type`, `as_url`, `as_goto`, `as_named`, `as_remote_goto`
- `PdfActionUrl::new`, `url`, `set_url`, `action_type`
- `PdfActionGoTo::new`, `destination`, `set_destination`, `action_type`
- `PdfActionNamed::new`, `name`, `set_name`, `action_type`
- `PdfActionRemoteGoTo::new`, `page_index`, `set_page_index`, `point`, `set_point`, `url`, `set_url`, `action_type`
- `PdfOutline::new`, `insert_child`, `set_label`, `set_destination`, `action`, `set_action`, `clear_action`
- `PdfAnnotation::new`, `info`, `set_contents`, `set_border`, `action`, `set_action`, `clear_action`
- `PdfBorder::new`, `info`, `set_style`, `set_line_width`, `set_dash_pattern`
- `PdfDestination::new`, `info`, `page`, `set_zoom`, `compare`

### Delegates, notifications, and utilities

- `PdfDocumentDelegate`, `PdfDocumentDelegateHandle`
- `PdfDocumentWriteOptions::{with_owner_password, with_user_password, with_access_permissions, with_burn_in_annotations, with_save_text_from_ocr, with_save_images_as_jpeg, with_optimize_images_for_screen}`
- `PdfDocumentNotification`, `PdfDocumentNotificationUserInfoKey`, `PdfViewNotification`, `PdfThumbnailViewNotification`
- `PdfActionNamedName`, `PdfLineStyle`, `PdfMarkupType`

### View state, widget appearance, and accessibility status

- `PdfView::new`, `set_document`, `set_display_mode`, `set_display_direction`, `set_display_box`
- `PdfView::set_current_selection`, `go_to_page`, `go_to_destination`, `visible_pages`
- `PdfThumbnailView::new`, `set_pdf_view`, `set_thumbnail_size`, `set_maximum_number_of_columns`
- `PdfAppearanceCharacteristics::new`, `set_control_type`, `set_caption`, `set_background_color`, `info`
- `PdfAccessibilityNode::public_api_available`, `PdfAccessibilityNode::availability_note`

## Examples

The crate now ships one smoke example per logical area:

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

Run one example:

```bash
cargo run --example 06_view_state
```

Run all examples:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the v0.2.1 header audit and deferred/skipped items.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
