# pdfkit-rs

Safe Rust bindings for Apple's [PDFKit](https://developer.apple.com/documentation/pdfkit) framework — PDF documents, pages, selections, outlines, and annotations on macOS. The published Cargo package is `pdfkit-rs`; the Rust library target is `pdfkit`.

> **Status:** v0.1.0 ships the practical PDFKit surface for loading PDF documents, reading metadata and text, traversing pages and outlines, inspecting annotations, and creating text selections.

## Highlights

- `PdfDocument` for opening PDFs from a file path or raw bytes
- `PdfPage` for page text, bounds, rotation, and annotation access
- `PdfSelection` for selected text and per-page bounds
- `PdfOutline` for simple outline traversal
- `PdfAnnotation` for common annotation metadata

## Quick start

```rust,no_run
use pdfkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = PdfDocument::from_url("document.pdf")?;
    println!("pages={}", document.page_count());
    println!("title={:?}", document.attributes()?.title);
    Ok(())
}
```

## Surface overview

### Documents

- `PdfDocument::from_url`, `from_bytes`, `info`, `attributes`, `string`
- `PdfDocument::page_count`, `page`, `pages`, `outline_root`
- `PdfDocument::unlock`, `write_to_url`

### Pages + selections

- `PdfPage::label`, `bounds`, `rotation`, `number_of_characters`, `string`
- `PdfPage::annotation_count`, `annotation`, `annotations`
- `PdfPage::selection_for_range`
- `PdfSelection::string`, `page_count`, `page`, `bounds_for_page`

### Outlines + annotations

- `PdfOutline::label`, `child_count`, `child`, `children`
- `PdfAnnotation::info`

## Smoke example

Run the document smoke example with:

```bash
cargo run --example 01_document_smoke
```

It loads a bundled one-page PDF fixture, validates the page count and extracted text, inspects the first page bounds, and then prints `✅ pdfkit document OK`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
