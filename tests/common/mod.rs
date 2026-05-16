#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use pdfkit::prelude::*;

pub fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets/hello.pdf")
}

pub fn fixture_document() -> Result<PdfDocument> {
    PdfDocument::from_url(fixture_path())
}

pub fn fixture_page() -> Result<PdfPage> {
    Ok(fixture_document()?.page(0).expect("missing first page"))
}

pub fn empty_document_with_page() -> Result<(PdfDocument, PdfPage)> {
    let document = PdfDocument::new()?;
    let page = PdfPage::new()?;
    document.insert_page(&page, 0)?;
    Ok((document, page))
}

pub fn output_path(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-output");
    fs::create_dir_all(&dir).expect("create test output dir");
    dir.join(name)
}

pub const fn word_point() -> PdfPoint {
    PdfPoint { x: 80.0, y: 700.0 }
}

pub fn sample_destination(page: &PdfPage) -> Result<PdfDestination> {
    PdfDestination::new(page, PdfPoint { x: 36.0, y: 72.0 })
}

pub fn sample_url_action() -> Result<PdfActionUrl> {
    PdfActionUrl::new("https://example.com")
}

pub fn sample_border() -> Result<PdfBorder> {
    let border = PdfBorder::new()?;
    border.set_style(PdfBorderStyle::Dashed)?;
    border.set_line_width(2.0);
    border.set_dash_pattern(Some(&[3.0, 1.0]))?;
    Ok(border)
}

pub fn sample_link_annotation() -> Result<PdfAnnotation> {
    let annotation = PdfAnnotation::new(
        PdfRect {
            x: 36.0,
            y: 36.0,
            width: 120.0,
            height: 40.0,
        },
        "Link",
    )?;
    annotation.set_contents(Some("Example link"))?;
    let border = sample_border()?;
    annotation.set_border(Some(&border))?;
    let action = sample_url_action()?;
    annotation.set_action_url(Some(&action))?;
    Ok(annotation)
}
