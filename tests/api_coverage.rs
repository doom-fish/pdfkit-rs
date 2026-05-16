use std::path::{Path, PathBuf};
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim())
}

fn read(path: &Path) -> String {
    let bytes =
        std::fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    String::from_utf8_lossy(&bytes).into_owned()
}

fn read_header(name: &str) -> String {
    read(&sdk_root().join(format!(
        "System/Library/Frameworks/PDFKit.framework/Headers/{name}.h"
    )))
}

fn read_bridge() -> String {
    let bridge_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("swift-bridge/Sources/PDFKitBridge");
    let mut paths = std::fs::read_dir(&bridge_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "swift"))
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .iter()
        .map(|path| read(path))
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(haystack.contains(needle), "missing `{needle}`");
    }
}

#[test]
fn pdf_document_surface_is_present() {
    let header = read_header("PDFDocument");
    assert_contains_all(
        &header,
        &[
            "- (nullable instancetype)initWithURL:(NSURL *)url",
            "- (nullable instancetype)initWithData:(NSData *)data",
            "@property (nonatomic, readonly, nullable) NSURL *documentURL;",
            "@property (nonatomic, copy, nullable) NSDictionary *documentAttributes;",
            "@property (nonatomic, readonly) BOOL isEncrypted;",
            "@property (nonatomic, readonly) BOOL isLocked;",
            "- (BOOL)unlockWithPassword:(NSString *)password;",
            "@property (nonatomic, readonly, nullable) NSString *string;",
            "@property (nonatomic, readonly) NSUInteger pageCount;",
            "- (nullable PDFPage *)pageAtIndex:(NSUInteger)index;",
            "@property (nonatomic, strong, nullable) PDFOutline *outlineRoot",
            "- (BOOL)writeToURL:(NSURL *)url;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_document_new_with_url",
            "pdf_document_new_with_data",
            "pdf_document_info_json",
            "pdf_document_attributes_json",
            "pdf_document_page_at",
            "pdf_document_unlock",
            "pdf_document_write_to_url",
        ],
    );
}

#[test]
fn pdf_page_and_selection_surface_is_present() {
    let page_header = read_header("PDFPage");
    assert_contains_all(
        &page_header,
        &[
            "@property (nonatomic, readonly, nullable) NSString *label;",
            "- (PDFRect)boundsForBox:(PDFDisplayBox)box;",
            "@property (nonatomic) NSInteger rotation;",
            "@property (nonatomic, readonly) NSArray<PDFAnnotation*> *annotations;",
            "@property (nonatomic, readonly) NSUInteger numberOfCharacters;",
            "@property (nonatomic, readonly, nullable) NSString *string;",
            "- (nullable PDFSelection *)selectionForRange:(NSRange)range;",
        ],
    );

    let selection_header = read_header("PDFSelection");
    assert_contains_all(
        &selection_header,
        &[
            "@property (nonatomic, readonly) NSArray<PDFPage*> *pages;",
            "@property (nonatomic, readonly, nullable) NSString *string;",
            "- (PDFRect)boundsForPage:(PDFPage *)page;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_page_bounds",
            "pdf_page_annotation_at",
            "pdf_page_selection_for_range",
            "pdf_selection_page_at",
            "pdf_selection_bounds_for_page",
        ],
    );
}

#[test]
fn pdf_outline_and_annotation_surface_is_present() {
    let outline_header = read_header("PDFOutline");
    assert_contains_all(
        &outline_header,
        &[
            "@property (nonatomic, readonly) NSUInteger numberOfChildren;",
            "- (nullable PDFOutline *)childAtIndex:(NSUInteger)index;",
            "@property (nonatomic, copy, nullable) NSString *label;",
        ],
    );

    let annotation_header = read_header("PDFAnnotation");
    assert_contains_all(
        &annotation_header,
        &[
            "@property (nonatomic, copy, nullable) NSString *type;",
            "@property (nonatomic) PDFRect bounds;",
            "@property (nonatomic, copy, nullable) NSString *contents",
            "@property (nonatomic, readonly) BOOL hasAppearanceStream;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_outline_child_at",
            "pdf_outline_label_string",
            "pdf_annotation_info_json",
        ],
    );
}
