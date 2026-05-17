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

#[allow(clippy::too_many_lines)]
#[test]
fn exhaustive_gap_closure_surface_is_present() {
    let action_reset_form_header = read_header("PDFActionResetForm");
    let annotation_header = read_header("PDFAnnotation");
    let annotation_utilities_header = read_header("PDFAnnotationUtilities");
    let destination_header = read_header("PDFDestination");
    let document_header = read_header("PDFDocument");
    let page_header = read_header("PDFPage");
    let page_overlay_header = read_header("PDFPageOverlayViewProvider");
    let selection_header = read_header("PDFSelection");
    let thumbnail_header = read_header("PDFThumbnailView");
    let view_header = read_header("PDFView");

    assert_contains_all(
        &action_reset_form_header,
        &[
            "@interface PDFActionResetForm : PDFAction <NSCopying>",
            "@property (nonatomic, copy, nullable) NSArray<NSString*> *fields;",
            "@property (nonatomic) BOOL fieldsIncludedAreCleared;",
        ],
    );
    assert_contains_all(
        &annotation_header,
        &[
            "typedef NSString* const PDFAnnotationKey NS_STRING_ENUM;",
            "PDFAnnotationKey PDFAnnotationKeyAdditionalActions",
            "PDFAnnotationKey PDFAnnotationKeyWidgetValue",
        ],
    );
    assert_contains_all(
        &annotation_utilities_header,
        &[
            "typedef NS_ENUM(NSInteger, PDFTextAnnotationIconType)",
            "typedef NS_ENUM(NSInteger, PDFWidgetCellState)",
            "PDFAnnotationSubtype PDFAnnotationSubtypeWidget",
            "PDFAnnotationHighlightingMode PDFAnnotationHighlightingModePush",
        ],
    );
    assert_contains_all(
        &destination_header,
        &["kPDFDestinationUnspecifiedValue"],
    );
    assert_contains_all(
        &document_header,
        &[
            "typedef NS_ENUM(NSInteger, PDFPrintScalingMode)",
            "typedef NS_ENUM(NSUInteger, PDFSelectionGranularity);",
            "withGranularity:(PDFSelectionGranularity)granularity",
        ],
    );
    assert_contains_all(
        &page_header,
        &[
            "typedef NS_OPTIONS(NSInteger, PDFAreaOfInterest)",
            "typedef NSString * PDFPageImageInitializationOption",
            "initWithImage:(PDFKitPlatformImage *)image options:",
        ],
    );
    assert_contains_all(
        &page_overlay_header,
        &[
            "@protocol PDFPageOverlayViewProvider <NSObject>",
            "overlayViewForPage:(PDFPage*)page",
        ],
    );
    assert_contains_all(
        &selection_header,
        &["typedef NS_ENUM(NSUInteger, PDFSelectionGranularity)"],
    );
    assert_contains_all(
        &thumbnail_header,
        &["typedef NS_ENUM(NSInteger, PDFThumbnailLayoutMode)"],
    );
    assert_contains_all(
        &view_header,
        &[
            "@property (nonatomic, weak, nullable) id< PDFViewDelegate > delegate;",
            "@property (nonatomic, weak, nullable) id<PDFPageOverlayViewProvider> pageOverlayViewProvider",
            "- (PDFAreaOfInterest)areaOfInterestForPoint:(PDFPoint)cursorLocation;",
            "@protocol PDFViewDelegate< NSObject >",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "pdf_action_as_reset_form",
            "pdf_action_reset_form_new",
            "pdf_document_selection_from_pages_points_with_granularity",
            "pdf_page_new_with_image_data",
            "pdf_page_overlay_view_new",
            "pdf_page_overlay_view_provider_new",
            "pdf_view_set_delegate",
            "pdf_view_set_page_overlay_view_provider",
            "pdf_view_area_of_interest_for_point",
            "pdf_view_delegate_new",
        ],
    );

    let action_reset_form_src = read_src("action_reset_form.rs");
    assert_contains_all(
        &action_reset_form_src,
        &[
            "pub struct PdfActionResetForm",
            "pub fn fields(&self) -> Result<Vec<String>>",
            "pub fn set_fields<I, S>(&self, fields: I) -> Result<()>",
        ],
    );

    let annotation_constants_src = read_src("annotation_constants.rs");
    assert_contains_all(
        &annotation_constants_src,
        &[
            "pub enum PdfAnnotationKey",
            "pub enum PdfAnnotationSubtype",
            "pub enum PdfAnnotationTextIconName",
            "pub enum PdfAnnotationWidgetSubtype",
        ],
    );

    let page_src = read_src("page.rs");
    assert_contains_all(
        &page_src,
        &[
            "PdfPageImageInitializationOptions",
            "pub fn from_image_data(",
        ],
    );

    let page_overlay_provider_src = read_src("page_overlay_view_provider.rs");
    assert_contains_all(
        &page_overlay_provider_src,
        &[
            "pub trait PdfPageOverlayViewProvider",
            "pub struct PdfPageOverlayViewProviderHandle",
        ],
    );

    let types_src = read_src("types.rs");
    assert_contains_all(
        &types_src,
        &[
            "pub enum PdfTextAnnotationIconType",
            "pub enum PdfWidgetCellState",
            "pub struct PdfAreaOfInterest(u64);",
            "pub enum PdfPrintScalingMode",
            "pub enum PdfSelectionGranularity",
            "pub enum PdfThumbnailLayoutMode",
            "pub struct PdfPageImageInitializationOptions",
        ],
    );

    let view_src = read_src("view.rs");
    assert_contains_all(
        &view_src,
        &[
            "pub fn set_delegate(&self, delegate: Option<&PdfViewDelegateHandle>) -> Result<()>",
            "pub fn set_page_overlay_view_provider(",
            "pub fn area_of_interest_for_point(&self, point: PdfPoint) -> PdfAreaOfInterest",
        ],
    );

    let view_delegate_src = read_src("view_delegate.rs");
    assert_contains_all(
        &view_delegate_src,
        &[
            "pub trait PdfViewDelegate",
            "pub struct PdfViewDelegateHandle",
            "fn handle_link_click(&mut self, _view: PdfView, _url: &str) -> bool",
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
