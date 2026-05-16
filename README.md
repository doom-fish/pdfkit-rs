# pdfkit-rs

Safe Rust bindings for Apple's [PDFKit](https://developer.apple.com/documentation/pdfkit) framework on macOS. The published Cargo package is `pdfkit-rs`; the Rust library target is `pdfkit`.

> **Status:** v0.2.0 extends the Swift bridge and safe Rust API with document/page editing, selections, outlines, actions, destinations, borders, appearance characteristics, `PDFView`, `PDFThumbnailView`, and a documented `PDFAccessibilityNode` status shim for the public-header gap.

## Highlights

- `PdfDocument` for loading, creating, mutating, and saving PDF documents
- `PdfPage` for bounds, text, character geometry, annotations, and selections
- `PdfAnnotation`, `PdfBorder`, `PdfActionUrl`, and `PdfActionGoTo` for lightweight annotation editing
- `PdfOutline` and `PdfDestination` for outline trees and in-document navigation targets
- `PdfSelection` for range/line inspection and selection composition
- `PdfView` and `PdfThumbnailView` for headless-safe view state and thumbnail configuration
- `PdfAppearanceCharacteristics` for widget appearance metadata
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

- `PdfDocument::new`, `from_url`, `from_bytes`, `write_to_url`
- `PdfDocument::insert_page`, `remove_page`, `exchange_pages`, `set_outline_root`
- `PdfDocument::selection_for_entire_document`, `selection_from_page_points`, `selection_from_page_characters`
- `PdfPage::new`, `bounds`, `set_bounds`, `rotation`, `set_rotation`
- `PdfPage::selection_for_range`, `selection_for_rect`, `selection_for_word_at_point`, `selection_for_line_at_point`
- `PdfSelection::new`, `text_range`, `selections_by_line`, `add_selection`

### Outlines, annotations, and navigation

- `PdfOutline::new`, `insert_child`, `set_label`, `set_destination`, `set_action_url`, `set_action_goto`
- `PdfAnnotation::new`, `info`, `set_contents`, `set_border`, `set_action_url`, `set_action_goto`
- `PdfBorder::new`, `info`, `set_style`, `set_line_width`, `set_dash_pattern`
- `PdfDestination::new`, `info`, `page`, `set_zoom`, `compare`
- `PdfActionUrl::new`, `url`, `set_url`, `action_type`
- `PdfActionGoTo::new`, `destination`, `set_destination`, `action_type`

### View state

- `PdfView::new`, `set_document`, `set_display_mode`, `set_display_direction`, `set_display_box`
- `PdfView::set_current_selection`, `go_to_page`, `go_to_destination`, `visible_pages`
- `PdfThumbnailView::new`, `set_pdf_view`, `set_thumbnail_size`, `set_maximum_number_of_columns`

### Widget appearance + accessibility status

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

Run one example:

```bash
cargo run --example 06_view_state
```

Run all examples:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the v0.2.0 header audit and deferred/skipped items.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
