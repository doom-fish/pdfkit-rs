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

fn read_src(path: &str) -> String {
    read(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src").join(path))
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(haystack.contains(needle), "missing `{needle}`");
    }
}

#[test]
fn document_page_selection_outline_annotation_surface_is_present() {
    let document_header = read_header("PDFDocument");
    assert_contains_all(
        &document_header,
        &[
            "- (instancetype)init NS_DESIGNATED_INITIALIZER;",
            "- (void)insertPage:(PDFPage *)page atIndex:(NSUInteger)index;",
            "- (nullable PDFSelection *)selectionFromPage:(PDFPage *)startPage atPoint:(PDFPoint)startPoint toPage:(PDFPage *)endPage atPoint:(PDFPoint)endPoint;",
            "@property (nonatomic, strong, nullable) PDFOutline *outlineRoot",
            "@property (nonatomic, weak, nullable) id<PDFDocumentDelegate> delegate;",
        ],
    );

    let page_header = read_header("PDFPage");
    assert_contains_all(
        &page_header,
        &[
            "- (nullable PDFAnnotation *)annotationAtPoint:(PDFPoint)point;",
            "- (nullable PDFSelection *)selectionForRect:(PDFRect)rect;",
            "- (nullable PDFSelection *)selectionForWordAtPoint:(PDFPoint)point;",
            "- (nullable PDFSelection *)selectionForLineAtPoint:(PDFPoint)point;",
            "- (nullable PDFSelection *)selectionFromPoint:(PDFPoint)startPoint toPoint:(PDFPoint)endPoint;",
            "@class PDFDocument, PDFAnnotation, PDFSelection, PDFAccessibilityNode;",
        ],
    );

    let selection_header = read_header("PDFSelection");
    assert_contains_all(
        &selection_header,
        &[
            "- (NSUInteger)numberOfTextRangesOnPage:(PDFPage *)page",
            "- (NSRange)rangeAtIndex:(NSUInteger)index onPage:(PDFPage *)page",
            "- (NSArray<PDFSelection*>*)selectionsByLine",
            "- (void)addSelection:(PDFSelection *)selection;",
        ],
    );

    let outline_header = read_header("PDFOutline");
    assert_contains_all(
        &outline_header,
        &[
            "- (instancetype)init NS_DESIGNATED_INITIALIZER;",
            "- (void)insertChild:(PDFOutline *)child atIndex:(NSUInteger)index",
            "@property (nonatomic, strong, nullable) PDFDestination *destination;",
            "@property (nonatomic, strong, nullable) PDFAction *action",
        ],
    );

    let annotation_header = read_header("PDFAnnotation");
    assert_contains_all(
        &annotation_header,
        &[
            "- (instancetype)initWithBounds:(PDFRect)bounds forType:(PDFAnnotationSubtype)annotationType withProperties:(nullable NSDictionary*)properties",
            "@property (nonatomic, strong, nullable) PDFBorder *border",
            "@property (nonatomic, strong, nullable) PDFAction *action",
            "@property (nonatomic, getter=isHighlighted) BOOL highlighted",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_document_new",
            "pdf_document_insert_page",
            "pdf_document_selection_from_pages_characters",
            "pdf_document_set_delegate",
            "pdf_page_selection_for_word_at_point",
            "pdf_page_selection_for_line_at_point",
            "pdf_annotation_action",
            "pdf_outline_set_action",
            "pdf_outline_set_destination",
            "pdf_selection_add_selection",
        ],
    );
}

#[test]
fn view_and_thumbnail_surface_is_present() {
    let view_header = read_header("PDFView");
    assert_contains_all(
        &view_header,
        &[
            "@property (nonatomic, retain, nullable) PDFDocument *document;",
            "@property (nonatomic) PDFDisplayMode displayMode;",
            "@property (nonatomic) PDFDisplayDirection displayDirection",
            "@property (nonatomic, readonly) NSArray<PDFPage *> *visiblePages",
            "@property (nonatomic, getter=isInMarkupMode) BOOL inMarkupMode",
        ],
    );

    let thumbnail_header = read_header("PDFThumbnailView");
    assert_contains_all(
        &thumbnail_header,
        &[
            "@property (nonatomic, weak, nullable) PDFView *PDFView;",
            "@property (nonatomic) PDFSize thumbnailSize;",
            "@property (nonatomic) NSUInteger maximumNumberOfColumns;",
            "@property (nonatomic) BOOL allowsMultipleSelection;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_view_new",
            "pdf_view_set_document",
            "pdf_view_visible_page_at",
            "pdf_thumbnail_view_new",
            "pdf_thumbnail_view_set_pdf_view",
            "pdf_thumbnail_view_selected_page_at",
        ],
    );
}

#[test]
fn action_border_destination_and_appearance_surface_is_present() {
    let action_url_header = read_header("PDFActionURL");
    let action_goto_header = read_header("PDFActionGoTo");
    let border_header = read_header("PDFBorder");
    let destination_header = read_header("PDFDestination");
    let appearance_header = read_header("PDFAppearanceCharacteristics");

    assert_contains_all(&action_url_header, &["- (instancetype)initWithURL:(NSURL *)url", "@property (nonatomic, copy, nullable) NSURL *URL;"]);
    assert_contains_all(&action_goto_header, &["- (instancetype)initWithDestination:(PDFDestination *)destination", "@property (nonatomic, strong) PDFDestination *destination;"]);
    assert_contains_all(&border_header, &["@property (nonatomic) PDFBorderStyle style;", "@property (nonatomic, copy, nullable) NSArray *dashPattern;"]);
    assert_contains_all(&destination_header, &["- (instancetype)initWithPage:(PDFPage *)page atPoint:(PDFPoint)point", "- (NSComparisonResult)compare:(PDFDestination *)destination"]);
    assert_contains_all(&appearance_header, &["@property (nonatomic) PDFWidgetControlType controlType;", "@property (nonatomic, copy, nullable) NSString *caption;"]);

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_action_url_new",
            "pdf_action_goto_new",
            "pdf_border_set_dash_pattern",
            "pdf_destination_compare",
            "pdf_appearance_characteristics_set_control_type",
        ],
    );
}

#[test]
fn advanced_action_delegate_notification_and_option_surface_is_present() {
    let action_header = read_header("PDFAction");
    let action_named_header = read_header("PDFActionNamed");
    let action_remote_goto_header = read_header("PDFActionRemoteGoTo");
    let document_header = read_header("PDFDocument");
    let annotation_utilities_header = read_header("PDFAnnotationUtilities");
    let view_header = read_header("PDFView");
    let thumbnail_header = read_header("PDFThumbnailView");

    assert_contains_all(&action_header, &["@property (nonatomic, readonly) NSString *type;"]);
    assert_contains_all(
        &action_named_header,
        &[
            "typedef NS_ENUM(NSInteger, PDFActionNamedName)",
            "- (instancetype)initWithName:(PDFActionNamedName)name NS_DESIGNATED_INITIALIZER;",
            "@property (nonatomic) PDFActionNamedName name;",
        ],
    );
    assert_contains_all(
        &action_remote_goto_header,
        &[
            "- (instancetype)initWithPageIndex:(NSUInteger)pageIndex atPoint:(PDFPoint)point fileURL:(NSURL *)url NS_DESIGNATED_INITIALIZER;",
            "@property (nonatomic) NSUInteger pageIndex;",
            "@property (nonatomic) PDFPoint point;",
            "@property (nonatomic, copy) NSURL *URL;",
        ],
    );
    assert_contains_all(
        &document_header,
        &[
            "PDFKIT_EXTERN NSNotificationName const PDFDocumentDidUnlockNotification",
            "PDFKIT_EXTERN NSString* const PDFDocumentFoundSelectionKey",
            "PDFKIT_EXTERN PDFDocumentWriteOption const PDFDocumentOwnerPasswordOption",
            "@protocol PDFDocumentDelegate< NSObject >",
        ],
    );
    assert_contains_all(
        &annotation_utilities_header,
        &[
            "typedef NS_ENUM(NSInteger, PDFLineStyle)",
            "typedef NS_ENUM(NSInteger, PDFMarkupType)",
        ],
    );
    assert_contains_all(
        &view_header,
        &[
            "PDFKIT_EXTERN NSNotificationName const PDFViewDocumentChangedNotification",
            "PDFKIT_EXTERN NSNotificationName const PDFViewVisiblePagesChangedNotification",
        ],
    );
    assert_contains_all(
        &thumbnail_header,
        &["PDFKIT_EXTERN NSString* const PDFThumbnailViewDocumentEditedNotification"],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_action_type_string",
            "pdf_action_named_new",
            "pdf_action_remote_goto_new",
            "pdf_document_delegate_new",
            "pdf_document_write_to_url_with_options",
        ],
    );

    let notifications_src = read_src("notifications.rs");
    assert_contains_all(
        &notifications_src,
        &[
            "pub enum PdfDocumentNotification",
            "pub enum PdfDocumentNotificationUserInfoKey",
            "pub enum PdfViewNotification",
            "pub enum PdfThumbnailViewNotification",
        ],
    );

    let delegate_src = read_src("document_delegate.rs");
    assert_contains_all(
        &delegate_src,
        &[
            "pub trait PdfDocumentDelegate",
            "pub struct PdfDocumentDelegateHandle",
            "fn page_class_name(&mut self) -> Option<String>",
            "fn annotation_class_name(&mut self, _annotation_type: &str) -> Option<String>",
        ],
    );

    let types_src = read_src("types.rs");
    assert_contains_all(
        &types_src,
        &[
            "pub enum PdfActionNamedName",
            "pub enum PdfLineStyle",
            "pub enum PdfMarkupType",
            "pub struct PdfDocumentWriteOptions",
        ],
    );
}

#[test]
fn accessibility_node_status_surface_is_present() {
    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_accessibility_node_public_api_available",
            "pdf_accessibility_node_reason",
        ],
    );
}
