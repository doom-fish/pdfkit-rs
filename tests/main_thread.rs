mod common;

use std::cell::{Cell, RefCell};
use std::error::Error;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;
use std::rc::Rc;
use std::thread;

use pdfkit::prelude::*;

type TestResult = std::result::Result<(), Box<dyn Error>>;
type Test = (&'static str, fn() -> TestResult);

fn check(condition: bool, what: &str) -> TestResult {
    if condition {
        Ok(())
    } else {
        Err(format!("check failed: {what}").into())
    }
}

fn view() -> Result<PdfView> {
    PdfView::new(PdfSize {
        width: 320.0,
        height: 480.0,
    })
}

fn scale_factor(view: &PdfView) -> std::result::Result<f64, Box<dyn Error>> {
    Ok(view.info()?.scale_factor)
}

struct NoopViewDelegate;

impl PdfViewDelegate for NoopViewDelegate {}

struct NoopPageOverlayProvider;

impl PdfPageOverlayViewProvider for NoopPageOverlayProvider {}

fn pdf_view_tracks_document_and_selection() -> TestResult {
    let document = common::fixture_document()?;
    let page = document.page(0).ok_or("missing page")?;
    let selection = page.selection_for_range(0, 5).ok_or("missing selection")?;
    let view = view()?;
    view.set_document(Some(&document))?;
    view.set_display_mode(PdfDisplayMode::SinglePage)?;
    view.set_display_direction(PdfDisplayDirection::Vertical)?;
    view.set_display_box(DisplayBox::CropBox)?;
    view.set_auto_scales(true);
    view.layout_document_view();
    view.go_to_page(&page)?;
    view.set_current_selection(Some(&selection), false)?;

    let info = view.info()?;
    check(view.document().is_some(), "view has a document")?;
    check(view.current_page().is_some(), "view has a current page")?;
    check(view.current_selection().is_some(), "view has a selection")?;
    check(
        view.visible_page_count() == info.visible_page_count,
        "visible page count matches the snapshot",
    )
}

fn thumbnail_view_roundtrip() -> TestResult {
    let document = common::fixture_document()?;
    let view = view()?;
    view.set_document(Some(&document))?;
    view.layout_document_view();

    let thumbnails = PdfThumbnailView::new(PdfSize {
        width: 180.0,
        height: 360.0,
    })?;
    thumbnails.set_pdf_view(Some(&view))?;
    thumbnails.set_thumbnail_size(PdfSize {
        width: 96.0,
        height: 128.0,
    });
    thumbnails.set_maximum_number_of_columns(2);
    thumbnails.set_allows_dragging(true);
    thumbnails.set_allows_multiple_selection(true);

    let info = thumbnails.info()?;
    check(info.has_pdf_view, "thumbnail view is attached")?;
    check(
        (info.thumbnail_size.width - 96.0).abs() < f64::EPSILON,
        "thumbnail width",
    )?;
    check(info.maximum_number_of_columns == 2, "column count")
}

fn view_delegate_and_overlay_provider_attach() -> TestResult {
    let view = view()?;
    let delegate = PdfViewDelegateHandle::new(NoopViewDelegate)?;
    view.set_delegate(Some(&delegate))?;
    let provider = PdfPageOverlayViewProviderHandle::new(NoopPageOverlayProvider)?;
    view.set_page_overlay_view_provider(Some(&provider))?;
    let _overlay = PdfPageOverlayView::new(PdfSize {
        width: 32.0,
        height: 24.0,
    })?;
    let _ = view.area_of_interest_for_point(common::word_point());
    view.set_delegate(None)?;
    view.set_page_overlay_view_provider(None)?;
    Ok(())
}

struct FixedScale {
    calls: Rc<Cell<usize>>,
}

impl PdfViewDelegate for FixedScale {
    fn will_change_scale_factor(&mut self, _view: PdfView, _scale_factor: f64) -> f64 {
        self.calls.set(self.calls.get() + 1);
        2.0
    }
}

fn view_delegate_decides_scale_changes_until_dropped() -> TestResult {
    let document = common::fixture_document()?;
    let view = view()?;
    view.set_document(Some(&document))?;
    let calls = Rc::new(Cell::new(0));
    let delegate = PdfViewDelegateHandle::new(FixedScale {
        calls: Rc::clone(&calls),
    })?;
    view.set_delegate(Some(&delegate))?;

    view.set_scale_factor(5.0);
    check(calls.get() > 0, "delegate consulted")?;
    check(
        (scale_factor(&view)? - 2.0).abs() < 1e-9,
        "delegate scale applied",
    )?;

    let delivered = calls.get();
    drop(delegate);
    view.set_scale_factor(4.0);
    check(calls.get() == delivered, "no delivery after drop")?;
    check(
        (scale_factor(&view)? - 4.0).abs() < 1e-9,
        "default scale after drop",
    )
}

struct Reentrant {
    calls: Rc<Cell<usize>>,
}

impl PdfViewDelegate for Reentrant {
    fn will_change_scale_factor(&mut self, view: PdfView, _scale_factor: f64) -> f64 {
        self.calls.set(self.calls.get() + 1);
        view.set_scale_factor(3.0);
        2.0
    }
}

fn reentrant_view_delegate_callback_is_skipped() -> TestResult {
    let document = common::fixture_document()?;
    let view = view()?;
    view.set_document(Some(&document))?;
    let calls = Rc::new(Cell::new(0));
    let delegate = PdfViewDelegateHandle::new(Reentrant {
        calls: Rc::clone(&calls),
    })?;
    view.set_delegate(Some(&delegate))?;

    view.set_scale_factor(5.0);

    check(calls.get() == 1, "nested callback not delivered")?;
    check(
        (scale_factor(&view)? - 2.0).abs() < 1e-9,
        "outer result wins",
    )
}

struct SelfDropping {
    handle: Rc<RefCell<Option<PdfViewDelegateHandle>>>,
    calls: Rc<Cell<usize>>,
}

impl PdfViewDelegate for SelfDropping {
    fn will_change_scale_factor(&mut self, _view: PdfView, scale_factor: f64) -> f64 {
        self.calls.set(self.calls.get() + 1);
        drop(self.handle.borrow_mut().take());
        scale_factor
    }
}

fn delegate_can_drop_its_own_handle_during_a_callback() -> TestResult {
    let document = common::fixture_document()?;
    let view = view()?;
    view.set_document(Some(&document))?;
    let slot = Rc::new(RefCell::new(None));
    let calls = Rc::new(Cell::new(0));
    let handle = PdfViewDelegateHandle::new(SelfDropping {
        handle: Rc::clone(&slot),
        calls: Rc::clone(&calls),
    })?;
    view.set_delegate(Some(&handle))?;
    *slot.borrow_mut() = Some(handle);

    view.set_scale_factor(1.5);
    check(slot.borrow().is_none(), "handle dropped inside the callback")?;
    view.set_scale_factor(2.5);
    check(calls.get() == 1, "no delivery after the handle dropped")?;
    check(
        (scale_factor(&view)? - 2.5).abs() < 1e-9,
        "default scale after drop",
    )
}

fn view_types_refuse_other_threads() -> TestResult {
    let codes = thread::spawn(|| {
        let size = PdfSize {
            width: 10.0,
            height: 10.0,
        };
        [
            PdfView::new(size).map(drop).map_err(|error| error.code()),
            PdfThumbnailView::new(size)
                .map(drop)
                .map_err(|error| error.code()),
            PdfPageOverlayView::new(size)
                .map(drop)
                .map_err(|error| error.code()),
            PdfViewDelegateHandle::new(NoopViewDelegate)
                .map(drop)
                .map_err(|error| error.code()),
            PdfPageOverlayViewProviderHandle::new(NoopPageOverlayProvider)
                .map(drop)
                .map_err(|error| error.code()),
        ]
    })
    .join()
    .map_err(|_| "worker thread panicked")?;
    check(
        codes.iter().all(|code| *code == Err(-4)),
        "every view type refuses a worker thread",
    )
}

const TESTS: &[Test] = &[
    (
        "pdf_view_tracks_document_and_selection",
        pdf_view_tracks_document_and_selection,
    ),
    ("thumbnail_view_roundtrip", thumbnail_view_roundtrip),
    (
        "view_delegate_and_overlay_provider_attach",
        view_delegate_and_overlay_provider_attach,
    ),
    (
        "view_delegate_decides_scale_changes_until_dropped",
        view_delegate_decides_scale_changes_until_dropped,
    ),
    (
        "reentrant_view_delegate_callback_is_skipped",
        reentrant_view_delegate_callback_is_skipped,
    ),
    (
        "delegate_can_drop_its_own_handle_during_a_callback",
        delegate_can_drop_its_own_handle_during_a_callback,
    ),
    (
        "view_types_refuse_other_threads",
        view_types_refuse_other_threads,
    ),
];

fn main() -> ExitCode {
    let filters: Vec<String> = std::env::args()
        .skip(1)
        .filter(|argument| !argument.starts_with('-'))
        .collect();
    let selected: Vec<_> = TESTS
        .iter()
        .filter(|(name, _)| {
            filters.is_empty() || filters.iter().any(|filter| name.contains(filter.as_str()))
        })
        .collect();

    println!("running {} main-thread PDFKit tests", selected.len());
    let mut failed = Vec::new();
    for (name, test) in &selected {
        match panic::catch_unwind(AssertUnwindSafe(test)) {
            Ok(Ok(())) => println!("test {name} ... ok"),
            Ok(Err(error)) => {
                println!("test {name} ... FAILED: {error}");
                failed.push(*name);
            }
            Err(_) => {
                println!("test {name} ... FAILED: panicked");
                failed.push(*name);
            }
        }
    }
    println!(
        "test result: {}. {} passed; {} failed",
        if failed.is_empty() { "ok" } else { "FAILED" },
        selected.len() - failed.len(),
        failed.len()
    );
    if failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
