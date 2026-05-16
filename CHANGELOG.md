# Changelog

## [0.1.0] - 2026-05-16

### Added

- Initial `pdfkit-rs` release for macOS PDF document inspection.
- `PdfDocument`, `PdfPage`, `PdfSelection`, `PdfOutline`, and `PdfAnnotation` wrappers.
- Swift bridge for `PDFKit.framework` document loading, metadata extraction, page traversal, outline walking, and selection handling.
- Bundled smoke fixture and `examples/01_document_smoke.rs` for end-to-end validation.
- Header-audit test `tests/api_coverage.rs` validating the targeted v0.1 PDFKit surface against the active SDK.
