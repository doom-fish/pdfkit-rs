# Changelog

All notable changes to `pdfkit-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- `PdfDocumentDelegateHandle` no longer frees the delegate while a callback is
  in flight: the Swift delegate object holds its own reference to the callback
  context until it is deallocated. Up to 0.3.2, dropping the handle during an
  asynchronous find could use freed memory.
- `PdfDocumentDelegate` requires `Send`, and callbacks no longer create
  aliasing `&mut` references to the delegate. Concurrent callbacks from
  PDFKit's background find queue, or a re-entrant callback such as
  `documentDidUnlock` triggered by `unlock()` inside a delegate method, were
  undefined behaviour.
- `PdfDocumentFindStream` no longer uses the caller's `PDFDocument` from a
  spawned thread while the caller keeps using it; it searches a private copy.
- `PdfDocumentWriteOptions` redacts passwords in `Debug`, compares them without
  an early exit, and wipes the Rust-side copies it passes to PDFKit.
  `PdfDocument::unlock` wipes its copy of the password too.
- A `PdfPage` keeps the document it came from alive. PDFKit only references a
  page's document weakly, so a page that outlived every `PdfDocument` handle
  crashed inside PDFKit (for example in `selection_for_word_at_point`).

### Fixed

- `selection_for_range` and `selection_from_page_characters` return `None`
  instead of aborting the process for indexes above `i64::MAX` (such as
  `usize::MAX`). Remote go-to page indexes saturate instead of trapping,
  including an index that a PDF stores as a negative number.
- Dropping a `PdfDocumentFindStream` no longer blocks until the search
  finishes.
- A match delivered after the delegate handle was dropped releases its
  `PDFSelection` instead of leaking it.

### Changed

- **Breaking:** `PdfDocumentDelegate` requires `Send`. Delegates that hold
  `Rc` or `RefCell` need `Arc` and `Mutex` (or similar) instead.
- **Breaking:** `PdfDocumentWriteOptions::owner_password` and `user_password`
  are `Option<Zeroizing<String>>`; the builder methods are unchanged, and
  `Debug` no longer shows the passwords.
- A delegate callback that PDFKit makes while the same thread is already
  inside the delegate is not delivered; class-name queries then fall back to
  the default class.
- `PdfDocumentFindStream` searches a copy of the document taken when the
  search starts, on a background dispatch queue, so later changes to the
  document are not searched.
- The docs of `number_of_characters`, `selection_for_range` and
  `selection_from_page_characters` state that indexes count UTF-16 code units.
- Requires `doom-fish-utils` 0.4.1 and adds a `zeroize` dependency.
  `rust-version` is now 1.82.

## [0.3.2] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.1] - 2026-05-20

- Added in-`src/` unit tests across annotation_constants, error, notifications, and types (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.3.0] - 2026-05-20

### Added

- `async_api` module behind the `async` feature, providing executor-agnostic async wrappers for `PDFDocument` string finding via `PdfDocumentFindStream`. Uses `doom-fish-utils::stream`.

## [0.2.4] - 2026-05-18

### Changed

- Added one-line `///` rustdoc coverage across the public Rust API outside `src/ffi`, including wrappers, enums, fields, delegates, notifications, and option/info types, bringing rustdoc item coverage to 100%.

## [0.2.3] - 2026-05-17

### Changed

- Added comprehensive SAFETY comments to all unsafe FFI boundary crossing code and callback trampolines in `handle.rs`, `util.rs`, `document_delegate.rs`, `view_delegate.rs`, and `page_overlay_view_provider.rs`. These comments document the safety requirements for pointer validity, memory ownership, and panic safety across C/Swift FFI boundaries.

## [0.2.2] - 2026-05-17

### Added

- Added `PdfActionResetForm` plus `PdfAction::as_reset_form` for form-reset action inspection and editing.
- Added typed wrappers for the remaining public annotation constants and enums, including `PdfAnnotationKey`, `PdfAnnotationHighlightingMode`, `PdfAnnotationLineEndingStyle`, `PdfAnnotationSubtype`, `PdfAnnotationTextIconName`, `PdfAnnotationWidgetSubtype`, `PdfTextAnnotationIconType`, `PdfWidgetCellState`, `PdfPrintScalingMode`, `PdfSelectionGranularity`, `PdfThumbnailLayoutMode`, and `PdfDestination::UNSPECIFIED_VALUE`.
- Added `PdfPage::from_image_data` with `PdfPageImageInitializationOptions`, plus `PdfViewDelegate`, `PdfPageOverlayView`, `PdfPageOverlayViewProvider`, and `PdfView::area_of_interest_for_point`.

### Changed

- Refreshed the README and coverage audits to report 100% coverage of the non-deprecated top-level PDFKit SDK declarations counted by the audit.

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
