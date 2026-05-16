# Changelog

## [0.2.1] - 2026-05-16

### Added

- Added the abstract `PdfAction` wrapper plus `PdfActionNamed`, `PdfActionRemoteGoTo`, and `PdfActionNamedName`, including generic action accessors on `PdfAnnotation` and `PdfOutline`.
- Added `PdfDocumentDelegate` / `PdfDocumentDelegateHandle`, `PdfDocumentWriteOptions`, and structured notification-name enums for `PDFDocument`, `PDFView`, and `PDFThumbnailView`.
- Added `PdfLineStyle` and `PdfMarkupType`, plus examples `14_action_named_remote_goto` and `15_document_delegate_write_options` and integration tests covering the new surface.

### Changed

- Expanded `PdfDocument` with `set_delegate` and `write_to_url_with_options`, and refreshed the README and coverage audits for the v0.2.1 surface.

## [0.2.0] - 2026-05-16

### Added

- Split the Swift bridge into per-area files covering `PDFDocument`, `PDFPage`, `PDFAnnotation`, `PDFOutline`, `PDFSelection`, `PDFView`, `PDFThumbnailView`, `PDFActionURL`, `PDFActionGoTo`, `PDFBorder`, `PDFDestination`, `PDFAppearanceCharacteristics`, and `PDFAccessibilityNode` status reporting.
- Expanded the safe Rust API with document/page mutation, selection composition, outline editing, annotation action/border helpers, destination comparison, and headless-safe `PdfView` / `PdfThumbnailView` wrappers.
- Added one example and one integration test per logical area, plus an updated header-audit test and `COVERAGE.md`.

### Changed

- Fixed the README doctest to use `std::result::Result` and refreshed the documentation to reflect the v0.2.0 surface.

## [0.1.0] - 2026-05-16

### Added

- Initial `pdfkit-rs` release for macOS PDF document inspection.
- `PdfDocument`, `PdfPage`, `PdfSelection`, `PdfOutline`, and `PdfAnnotation` wrappers.
- Swift bridge for `PDFKit.framework` document loading, metadata extraction, page traversal, outline walking, and selection handling.
- Bundled smoke fixture and `examples/01_document_smoke.rs` for end-to-end validation.
- Header-audit test `tests/api_coverage.rs` validating the targeted v0.1 PDFKit surface against the active SDK.
