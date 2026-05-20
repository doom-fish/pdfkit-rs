use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

/// Wraps `CGRect` values used by PDFKit geometry APIs.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct PdfRect {
    /// Wraps the rectangle x-origin used by PDFKit geometry APIs.
    pub x: f64,
    /// Wraps the rectangle y-origin used by PDFKit geometry APIs.
    pub y: f64,
    /// Wraps the rectangle width used by PDFKit geometry APIs.
    pub width: f64,
    /// Wraps the rectangle height used by PDFKit geometry APIs.
    pub height: f64,
}

/// Wraps `CGPoint` values used by PDFKit geometry APIs.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfPoint {
    /// Wraps the point x-coordinate used by PDFKit geometry APIs.
    pub x: f64,
    /// Wraps the point y-coordinate used by PDFKit geometry APIs.
    pub y: f64,
}

/// Wraps `CGSize` values used by PDFKit geometry APIs.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfSize {
    /// Wraps the width used by PDFKit geometry APIs.
    pub width: f64,
    /// Wraps the height used by PDFKit geometry APIs.
    pub height: f64,
}

/// Wraps `NSEdgeInsets` values used by `PDFView`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfEdgeInsets {
    /// Wraps the top inset used by `PDFView` layout APIs.
    pub top: f64,
    /// Wraps the left inset used by `PDFView` layout APIs.
    pub left: f64,
    /// Wraps the bottom inset used by `PDFView` layout APIs.
    pub bottom: f64,
    /// Wraps the right inset used by `PDFView` layout APIs.
    pub right: f64,
}

/// Wraps RGBA color values used by PDFKit.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfColor {
    /// Wraps the red component used by PDFKit color APIs.
    pub red: f64,
    /// Wraps the green component used by PDFKit color APIs.
    pub green: f64,
    /// Wraps the blue component used by PDFKit color APIs.
    pub blue: f64,
    /// Wraps the alpha component used by PDFKit color APIs.
    pub alpha: f64,
}

/// Wraps text-range values returned by `PDFSelection`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct PdfTextRange {
    /// Wraps the starting character index returned by `PDFSelection`.
    pub location: usize,
    /// Wraps the character count returned by `PDFSelection`.
    pub length: usize,
}

/// Wraps `PDFDisplayBox` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DisplayBox {
    /// Wraps the corresponding `PDFDisplayBox` value.
    MediaBox = 0,
    /// Wraps the corresponding `PDFDisplayBox` value.
    CropBox = 1,
    /// Wraps the corresponding `PDFDisplayBox` value.
    BleedBox = 2,
    /// Wraps the corresponding `PDFDisplayBox` value.
    TrimBox = 3,
    /// Wraps the corresponding `PDFDisplayBox` value.
    ArtBox = 4,
}

impl DisplayBox {
    /// Returns the raw `PDFDisplayBox` value used by PDFKit.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

/// Wraps `PDFBorderStyle` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfBorderStyle {
    /// Wraps the corresponding `PDFBorderStyle` value.
    Solid = 0,
    /// Wraps the corresponding `PDFBorderStyle` value.
    Dashed = 1,
    /// Wraps the corresponding `PDFBorderStyle` value.
    Beveled = 2,
    /// Wraps the corresponding `PDFBorderStyle` value.
    Inset = 3,
    /// Wraps the corresponding `PDFBorderStyle` value.
    Underline = 4,
}

impl PdfBorderStyle {
    /// Converts a raw `PDFBorderStyle` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Solid),
            1 => Some(Self::Dashed),
            2 => Some(Self::Beveled),
            3 => Some(Self::Inset),
            4 => Some(Self::Underline),
            _ => None,
        }
    }
}

/// Wraps `PDFActionNamedName` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfActionNamedName {
    /// Wraps the corresponding `PDFActionNamedName` value.
    None = 0,
    /// Wraps the corresponding `PDFActionNamedName` value.
    NextPage = 1,
    /// Wraps the corresponding `PDFActionNamedName` value.
    PreviousPage = 2,
    /// Wraps the corresponding `PDFActionNamedName` value.
    FirstPage = 3,
    /// Wraps the corresponding `PDFActionNamedName` value.
    LastPage = 4,
    /// Wraps the corresponding `PDFActionNamedName` value.
    GoBack = 5,
    /// Wraps the corresponding `PDFActionNamedName` value.
    GoForward = 6,
    /// Wraps the corresponding `PDFActionNamedName` value.
    GoToPage = 7,
    /// Wraps the corresponding `PDFActionNamedName` value.
    Find = 8,
    /// Wraps the corresponding `PDFActionNamedName` value.
    Print = 9,
    /// Wraps the corresponding `PDFActionNamedName` value.
    ZoomIn = 10,
    /// Wraps the corresponding `PDFActionNamedName` value.
    ZoomOut = 11,
}

impl PdfActionNamedName {
    /// Returns the raw `PDFActionNamedName` value used by PDFKit.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Converts a raw `PDFActionNamedName` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::NextPage),
            2 => Some(Self::PreviousPage),
            3 => Some(Self::FirstPage),
            4 => Some(Self::LastPage),
            5 => Some(Self::GoBack),
            6 => Some(Self::GoForward),
            7 => Some(Self::GoToPage),
            8 => Some(Self::Find),
            9 => Some(Self::Print),
            10 => Some(Self::ZoomIn),
            11 => Some(Self::ZoomOut),
            _ => None,
        }
    }
}

/// Wraps `PDFLineStyle` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfLineStyle {
    /// Wraps the corresponding `PDFLineStyle` value.
    None = 0,
    /// Wraps the corresponding `PDFLineStyle` value.
    Square = 1,
    /// Wraps the corresponding `PDFLineStyle` value.
    Circle = 2,
    /// Wraps the corresponding `PDFLineStyle` value.
    Diamond = 3,
    /// Wraps the corresponding `PDFLineStyle` value.
    OpenArrow = 4,
    /// Wraps the corresponding `PDFLineStyle` value.
    ClosedArrow = 5,
}

impl PdfLineStyle {
    /// Converts a raw `PDFLineStyle` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::Square),
            2 => Some(Self::Circle),
            3 => Some(Self::Diamond),
            4 => Some(Self::OpenArrow),
            5 => Some(Self::ClosedArrow),
            _ => None,
        }
    }
}

/// Wraps `PDFTextAnnotationIconType` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfTextAnnotationIconType {
    /// Wraps the corresponding `PDFTextAnnotationIconType` value.
    Comment = 0,
    /// Wraps the corresponding `PDFTextAnnotationIconType` value.
    Key = 1,
    /// Wraps the corresponding `PDFTextAnnotationIconType` value.
    Note = 2,
    /// Wraps the corresponding `PDFTextAnnotationIconType` value.
    Help = 3,
    /// Wraps the corresponding `PDFTextAnnotationIconType` value.
    NewParagraph = 4,
    /// Wraps the corresponding `PDFTextAnnotationIconType` value.
    Paragraph = 5,
    /// Wraps the corresponding `PDFTextAnnotationIconType` value.
    Insert = 6,
}

impl PdfTextAnnotationIconType {
    /// Returns the raw `PDFTextAnnotationIconType` value used by PDFKit.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Converts a raw `PDFTextAnnotationIconType` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Comment),
            1 => Some(Self::Key),
            2 => Some(Self::Note),
            3 => Some(Self::Help),
            4 => Some(Self::NewParagraph),
            5 => Some(Self::Paragraph),
            6 => Some(Self::Insert),
            _ => None,
        }
    }
}

/// Wraps `PDFMarkupType` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfMarkupType {
    /// Wraps the corresponding `PDFMarkupType` value.
    Highlight = 0,
    /// Wraps the corresponding `PDFMarkupType` value.
    StrikeOut = 1,
    /// Wraps the corresponding `PDFMarkupType` value.
    Underline = 2,
    /// Wraps the corresponding `PDFMarkupType` value.
    Redact = 3,
}

impl PdfMarkupType {
    /// Converts a raw `PDFMarkupType` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Highlight),
            1 => Some(Self::StrikeOut),
            2 => Some(Self::Underline),
            3 => Some(Self::Redact),
            _ => None,
        }
    }
}

/// Wraps `PDFDisplayMode` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfDisplayMode {
    /// Wraps the corresponding `PDFDisplayMode` value.
    SinglePage = 0,
    /// Wraps the corresponding `PDFDisplayMode` value.
    SinglePageContinuous = 1,
    /// Wraps the corresponding `PDFDisplayMode` value.
    TwoUp = 2,
    /// Wraps the corresponding `PDFDisplayMode` value.
    TwoUpContinuous = 3,
}

impl PdfDisplayMode {
    /// Converts a raw `PDFDisplayMode` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::SinglePage),
            1 => Some(Self::SinglePageContinuous),
            2 => Some(Self::TwoUp),
            3 => Some(Self::TwoUpContinuous),
            _ => None,
        }
    }
}

/// Wraps `PDFDisplayDirection` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfDisplayDirection {
    /// Wraps the corresponding `PDFDisplayDirection` value.
    Vertical = 0,
    /// Wraps the corresponding `PDFDisplayDirection` value.
    Horizontal = 1,
}

impl PdfDisplayDirection {
    /// Converts a raw `PDFDisplayDirection` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Vertical),
            1 => Some(Self::Horizontal),
            _ => None,
        }
    }
}

/// Wraps `PDFInterpolationQuality` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfInterpolationQuality {
    /// Wraps the corresponding `PDFInterpolationQuality` value.
    None = 0,
    /// Wraps the corresponding `PDFInterpolationQuality` value.
    Low = 1,
    /// Wraps the corresponding `PDFInterpolationQuality` value.
    High = 2,
}

impl PdfInterpolationQuality {
    /// Converts a raw `PDFInterpolationQuality` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::Low),
            2 => Some(Self::High),
            _ => None,
        }
    }
}

/// Wraps `PDFView area-of-interest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PdfAreaOfInterest(u64);

impl PdfAreaOfInterest {
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const NONE: Self = Self(0);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const PAGE: Self = Self(1 << 0);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const TEXT: Self = Self(1 << 1);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const ANNOTATION: Self = Self(1 << 2);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const LINK: Self = Self(1 << 3);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const CONTROL: Self = Self(1 << 4);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const TEXT_FIELD: Self = Self(1 << 5);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const ICON: Self = Self(1 << 6);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const POPUP: Self = Self(1 << 7);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const IMAGE: Self = Self(1 << 8);
    /// Wraps the corresponding `PDFView` area-of-interest flag.
    pub const ANY: Self = Self(i64::MAX as u64);

    /// Creates a flag set from raw PDFKit bits.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the raw PDFKit bit pattern.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Reports whether no PDFKit flags are set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Checks PDFKit flag membership against another value.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Checks PDFKit flag membership against another value.
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

impl BitOr for PdfAreaOfInterest {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for PdfAreaOfInterest {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for PdfAreaOfInterest {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for PdfAreaOfInterest {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Wraps `PDFWidgetControlType` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfWidgetControlType {
    /// Wraps the corresponding `PDFWidgetControlType` value.
    Unknown = -1,
    /// Wraps the corresponding `PDFWidgetControlType` value.
    PushButton = 0,
    /// Wraps the corresponding `PDFWidgetControlType` value.
    RadioButton = 1,
    /// Wraps the corresponding `PDFWidgetControlType` value.
    CheckBox = 2,
}

impl PdfWidgetControlType {
    /// Converts a raw `PDFWidgetControlType` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            -1 => Some(Self::Unknown),
            0 => Some(Self::PushButton),
            1 => Some(Self::RadioButton),
            2 => Some(Self::CheckBox),
            _ => None,
        }
    }
}

/// Wraps `PDFWidgetCellState` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfWidgetCellState {
    /// Wraps the corresponding `PDFWidgetCellState` value.
    Mixed = -1,
    /// Wraps the corresponding `PDFWidgetCellState` value.
    Off = 0,
    /// Wraps the corresponding `PDFWidgetCellState` value.
    On = 1,
}

impl PdfWidgetCellState {
    /// Returns the raw `PDFWidgetCellState` value used by PDFKit.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Converts a raw `PDFWidgetCellState` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            -1 => Some(Self::Mixed),
            0 => Some(Self::Off),
            1 => Some(Self::On),
            _ => None,
        }
    }
}

/// Wraps `PDFPrintScalingMode` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfPrintScalingMode {
    /// Wraps the corresponding `PDFPrintScalingMode` value.
    None = 0,
    /// Wraps the corresponding `PDFPrintScalingMode` value.
    ToFit = 1,
    /// Wraps the corresponding `PDFPrintScalingMode` value.
    DownToFit = 2,
}

impl PdfPrintScalingMode {
    /// Returns the raw `PDFPrintScalingMode` value used by PDFKit.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Converts a raw `PDFPrintScalingMode` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::ToFit),
            2 => Some(Self::DownToFit),
            _ => None,
        }
    }
}

/// Wraps `PDFDocumentPermissions` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfDocumentPermissions {
    /// Wraps the corresponding `PDFDocumentPermissions` value.
    None = 0,
    /// Wraps the corresponding `PDFDocumentPermissions` value.
    User = 1,
    /// Wraps the corresponding `PDFDocumentPermissions` value.
    Owner = 2,
}

impl PdfDocumentPermissions {
    /// Converts a raw `PDFDocumentPermissions` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::User),
            2 => Some(Self::Owner),
            _ => None,
        }
    }
}

/// Wraps `PDFSelectionGranularity` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum PdfSelectionGranularity {
    /// Wraps the corresponding `PDFSelectionGranularity` value.
    Character = 0,
    /// Wraps the corresponding `PDFSelectionGranularity` value.
    Word = 1,
    /// Wraps the corresponding `PDFSelectionGranularity` value.
    Line = 2,
}

impl PdfSelectionGranularity {
    /// Returns the raw `PDFSelectionGranularity` value used by PDFKit.
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self as u64
    }

    /// Converts a raw `PDFSelectionGranularity` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Option<Self> {
        match raw {
            0 => Some(Self::Character),
            1 => Some(Self::Word),
            2 => Some(Self::Line),
            _ => None,
        }
    }
}

/// Snapshot of `PDFDocument` state.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfDocumentInfo {
    /// Mirrors the corresponding `PDFDocument` field.
    pub document_url: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub major_version: i32,
    /// Mirrors the corresponding `PDFDocument` field.
    pub minor_version: i32,
    /// Mirrors the corresponding `PDFDocument` field.
    pub is_encrypted: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub is_locked: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub permissions_status: i32,
    /// Mirrors the corresponding `PDFDocument` field.
    pub access_permissions: u64,
    /// Mirrors the corresponding `PDFDocument` field.
    pub allows_printing: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub allows_copying: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub allows_document_changes: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub allows_document_assembly: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub allows_content_accessibility: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub allows_commenting: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub allows_form_field_entry: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub page_class: String,
}

impl PdfDocumentInfo {
    /// Converts the stored raw PDFKit value into a typed enum.
    #[must_use]
    pub fn permissions_status_enum(&self) -> Option<PdfDocumentPermissions> {
        PdfDocumentPermissions::from_raw(self.permissions_status)
    }
}

/// Wraps `PDFDocumentAttributes`.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfDocumentAttributes {
    /// Mirrors the corresponding `PDFDocument` field.
    pub title: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub author: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub subject: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub creator: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub producer: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub creation_date: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub modification_date: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub keywords: Option<Vec<String>>,
}

/// Builder-style options for the corresponding `PDFPageImageInitialization` API.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct PdfPageImageInitializationOptions {
    /// Mirrors the corresponding `PDFPage` field.
    pub media_box: Option<PdfRect>,
    /// Mirrors the corresponding `PDFPage` field.
    pub rotation: Option<i32>,
    /// Mirrors the corresponding `PDFPage` field.
    pub upscale_if_smaller: bool,
    /// Mirrors the corresponding `PDFPage` field.
    pub compression_quality: Option<f64>,
}

impl PdfPageImageInitializationOptions {
    /// Sets the corresponding `PDFPageImageInitialization` option and returns the builder.
    pub fn with_media_box(mut self, value: PdfRect) -> Self {
        self.media_box = Some(value);
        self
    }

    /// Sets the corresponding `PDFPageImageInitialization` option and returns the builder.
    pub fn with_rotation(mut self, value: i32) -> Self {
        self.rotation = Some(value);
        self
    }

    /// Sets the corresponding `PDFPageImageInitialization` option and returns the builder.
    pub fn with_upscale_if_smaller(mut self, value: bool) -> Self {
        self.upscale_if_smaller = value;
        self
    }

    /// Sets the corresponding `PDFPageImageInitialization` option and returns the builder.
    pub fn with_compression_quality(mut self, value: f64) -> Self {
        self.compression_quality = Some(value);
        self
    }
}

/// Builder-style options for the corresponding `PDFDocumentWrite` API.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct PdfDocumentWriteOptions {
    /// Mirrors the corresponding `PDFDocument` field.
    pub owner_password: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub user_password: Option<String>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub access_permissions: Option<u64>,
    /// Mirrors the corresponding `PDFDocument` field.
    pub burn_in_annotations: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub save_text_from_ocr: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub save_images_as_jpeg: bool,
    /// Mirrors the corresponding `PDFDocument` field.
    pub optimize_images_for_screen: bool,
}

impl PdfDocumentWriteOptions {
    /// Sets the corresponding `PDFDocumentWrite` option and returns the builder.
    pub fn with_owner_password(mut self, value: impl Into<String>) -> Self {
        self.owner_password = Some(value.into());
        self
    }

    /// Sets the corresponding `PDFDocumentWrite` option and returns the builder.
    pub fn with_user_password(mut self, value: impl Into<String>) -> Self {
        self.user_password = Some(value.into());
        self
    }

    /// Sets the corresponding `PDFDocumentWrite` option and returns the builder.
    pub fn with_access_permissions(mut self, value: u64) -> Self {
        self.access_permissions = Some(value);
        self
    }

    /// Sets the corresponding `PDFDocumentWrite` option and returns the builder.
    pub fn with_burn_in_annotations(mut self, value: bool) -> Self {
        self.burn_in_annotations = value;
        self
    }

    /// Sets the corresponding `PDFDocumentWrite` option and returns the builder.
    pub fn with_save_text_from_ocr(mut self, value: bool) -> Self {
        self.save_text_from_ocr = value;
        self
    }

    /// Sets the corresponding `PDFDocumentWrite` option and returns the builder.
    pub fn with_save_images_as_jpeg(mut self, value: bool) -> Self {
        self.save_images_as_jpeg = value;
        self
    }

    /// Sets the corresponding `PDFDocumentWrite` option and returns the builder.
    pub fn with_optimize_images_for_screen(mut self, value: bool) -> Self {
        self.optimize_images_for_screen = value;
        self
    }
}

/// Snapshot of `PDFBorder` state.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfBorderInfo {
    /// Mirrors the corresponding `PDFBorder` field.
    pub style: i32,
    /// Mirrors the corresponding `PDFBorder` field.
    pub line_width: f64,
    /// Mirrors the corresponding `PDFBorder` field.
    pub dash_pattern: Option<Vec<f64>>,
}

impl PdfBorderInfo {
    /// Converts the stored raw PDFKit value into a typed enum.
    #[must_use]
    pub fn style_enum(&self) -> Option<PdfBorderStyle> {
        PdfBorderStyle::from_raw(self.style)
    }
}

/// Snapshot of `PDFAnnotation` state.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfAnnotationInfo {
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub annotation_type: Option<String>,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub bounds: PdfRect,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub contents: Option<String>,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub should_display: bool,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub should_print: bool,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub has_appearance_stream: bool,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub user_name: Option<String>,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub modification_date: Option<String>,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub color: Option<PdfColor>,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub highlighted: bool,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub action_type: Option<String>,
    /// Mirrors the corresponding `PDFAnnotation` field.
    pub border: Option<PdfBorderInfo>,
}

/// Snapshot of `PDFDestination` state.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfDestinationInfo {
    /// Mirrors the corresponding `PDFDestination` field.
    pub page_label: Option<String>,
    /// Mirrors the corresponding `PDFDestination` field.
    pub page_index: Option<usize>,
    /// Mirrors the corresponding `PDFDestination` field.
    pub point: PdfPoint,
    /// Mirrors the corresponding `PDFDestination` field.
    pub zoom: f64,
}

/// Snapshot of `PDFAppearanceCharacteristics` state.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfAppearanceCharacteristicsInfo {
    /// Mirrors the corresponding `PDFAppearanceCharacteristics` field.
    pub control_type: i32,
    /// Mirrors the corresponding `PDFAppearanceCharacteristics` field.
    pub background_color: Option<PdfColor>,
    /// Mirrors the corresponding `PDFAppearanceCharacteristics` field.
    pub border_color: Option<PdfColor>,
    /// Mirrors the corresponding `PDFAppearanceCharacteristics` field.
    pub rotation: i32,
    /// Mirrors the corresponding `PDFAppearanceCharacteristics` field.
    pub caption: Option<String>,
    /// Mirrors the corresponding `PDFAppearanceCharacteristics` field.
    pub rollover_caption: Option<String>,
    /// Mirrors the corresponding `PDFAppearanceCharacteristics` field.
    pub down_caption: Option<String>,
}

impl PdfAppearanceCharacteristicsInfo {
    /// Converts the stored raw PDFKit value into a typed enum.
    #[must_use]
    pub fn control_type_enum(&self) -> Option<PdfWidgetControlType> {
        PdfWidgetControlType::from_raw(self.control_type)
    }
}

/// Snapshot of `PDFView` state.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfViewInfo {
    /// Mirrors the corresponding `PDFView` field.
    pub display_mode: i32,
    /// Mirrors the corresponding `PDFView` field.
    pub display_direction: i32,
    /// Mirrors the corresponding `PDFView` field.
    pub displays_page_breaks: bool,
    /// Mirrors the corresponding `PDFView` field.
    pub page_break_margins: PdfEdgeInsets,
    /// Mirrors the corresponding `PDFView` field.
    pub display_box: i32,
    /// Mirrors the corresponding `PDFView` field.
    pub displays_as_book: bool,
    /// Mirrors the corresponding `PDFView` field.
    pub displays_rtl: bool,
    /// Mirrors the corresponding `PDFView` field.
    pub background_color: Option<PdfColor>,
    /// Mirrors the corresponding `PDFView` field.
    pub interpolation_quality: i32,
    /// Mirrors the corresponding `PDFView` field.
    pub page_shadows_enabled: bool,
    /// Mirrors the corresponding `PDFView` field.
    pub scale_factor: f64,
    /// Mirrors the corresponding `PDFView` field.
    pub min_scale_factor: f64,
    /// Mirrors the corresponding `PDFView` field.
    pub max_scale_factor: f64,
    /// Mirrors the corresponding `PDFView` field.
    pub auto_scales: bool,
    /// Mirrors the corresponding `PDFView` field.
    pub scale_factor_for_size_to_fit: f64,
    /// Mirrors the corresponding `PDFView` field.
    pub in_markup_mode: bool,
    /// Mirrors the corresponding `PDFView` field.
    pub has_document_view: bool,
    /// Mirrors the corresponding `PDFView` field.
    pub visible_page_count: usize,
    /// Mirrors the corresponding `PDFView` field.
    pub current_page_label: Option<String>,
}

impl PdfViewInfo {
    /// Converts the stored raw PDFKit value into a typed enum.
    #[must_use]
    pub fn display_mode_enum(&self) -> Option<PdfDisplayMode> {
        PdfDisplayMode::from_raw(self.display_mode)
    }

    /// Converts the stored raw PDFKit value into a typed enum.
    #[must_use]
    pub fn display_direction_enum(&self) -> Option<PdfDisplayDirection> {
        PdfDisplayDirection::from_raw(self.display_direction)
    }

    /// Converts the stored raw PDFKit value into a typed enum.
    #[must_use]
    pub fn interpolation_quality_enum(&self) -> Option<PdfInterpolationQuality> {
        PdfInterpolationQuality::from_raw(self.interpolation_quality)
    }

    /// Converts the stored raw PDFKit value into a typed enum.
    #[must_use]
    pub fn display_box_enum(&self) -> Option<DisplayBox> {
        match self.display_box {
            0 => Some(DisplayBox::MediaBox),
            1 => Some(DisplayBox::CropBox),
            2 => Some(DisplayBox::BleedBox),
            3 => Some(DisplayBox::TrimBox),
            4 => Some(DisplayBox::ArtBox),
            _ => None,
        }
    }
}

/// Wraps `PDFThumbnailLayoutMode` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfThumbnailLayoutMode {
    /// Wraps the corresponding `PDFThumbnailLayoutMode` value.
    Vertical = 0,
    /// Wraps the corresponding `PDFThumbnailLayoutMode` value.
    Horizontal = 1,
}

impl PdfThumbnailLayoutMode {
    /// Returns the raw `PDFThumbnailLayoutMode` value used by PDFKit.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Converts a raw `PDFThumbnailLayoutMode` value into this Rust enum.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Vertical),
            1 => Some(Self::Horizontal),
            _ => None,
        }
    }
}

/// Snapshot of `PDFThumbnailView` state.
#[derive(Debug, Clone, Deserialize)]
pub struct PdfThumbnailViewInfo {
    /// Mirrors the corresponding `PDFThumbnailView` field.
    pub has_pdf_view: bool,
    /// Mirrors the corresponding `PDFThumbnailView` field.
    pub background_color: Option<PdfColor>,
    /// Mirrors the corresponding `PDFThumbnailView` field.
    pub selected_pages_count: usize,
    /// Mirrors the corresponding `PDFThumbnailView` field.
    pub thumbnail_size: PdfSize,
    /// Mirrors the corresponding `PDFThumbnailView` field.
    pub maximum_number_of_columns: usize,
    /// Mirrors the corresponding `PDFThumbnailView` field.
    pub allows_dragging: bool,
    /// Mirrors the corresponding `PDFThumbnailView` field.
    pub allows_multiple_selection: bool,
}

#[cfg(test)]
mod tests {
    use super::{
        DisplayBox, PdfActionNamedName, PdfAreaOfInterest, PdfBorderStyle, PdfColor,
        PdfDisplayDirection, PdfDisplayMode, PdfDocumentWriteOptions,
        PdfPageImageInitializationOptions, PdfInterpolationQuality, PdfLineStyle, PdfMarkupType,
        PdfPoint, PdfRect, PdfSelectionGranularity, PdfSize, PdfTextAnnotationIconType,
        PdfThumbnailLayoutMode, PdfWidgetCellState,
    };

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < f64::EPSILON, "expected {left} to match {right}");
    }

    #[test]
    fn geometry_wrappers_preserve_field_values() {
        let rect = PdfRect {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        };
        let point = PdfPoint { x: -1.0, y: 5.0 };
        let size = PdfSize {
            width: 8.0,
            height: 9.0,
        };
        let color = PdfColor {
            red: 0.1,
            green: 0.2,
            blue: 0.3,
            alpha: 0.4,
        };

        assert_close(rect.x, 1.0);
        assert_close(rect.y, 2.0);
        assert_close(rect.width, 3.0);
        assert_close(rect.height, 4.0);
        assert_close(point.x, -1.0);
        assert_close(point.y, 5.0);
        assert_close(size.width, 8.0);
        assert_close(size.height, 9.0);
        assert_close(color.red, 0.1);
        assert_close(color.green, 0.2);
        assert_close(color.blue, 0.3);
        assert_close(color.alpha, 0.4);
    }

    #[test]
    fn action_and_annotation_enums_round_trip_raw_values() {
        assert_eq!(DisplayBox::CropBox.as_raw(), 1);
        assert_eq!(PdfBorderStyle::from_raw(4), Some(PdfBorderStyle::Underline));
        assert_eq!(PdfBorderStyle::from_raw(99), None);
        assert_eq!(PdfActionNamedName::GoToPage.as_raw(), 7);
        assert_eq!(PdfActionNamedName::from_raw(7), Some(PdfActionNamedName::GoToPage));
        assert_eq!(PdfLineStyle::from_raw(5), Some(PdfLineStyle::ClosedArrow));
        assert_eq!(PdfTextAnnotationIconType::Insert.as_raw(), 6);
        assert_eq!(PdfTextAnnotationIconType::from_raw(6), Some(PdfTextAnnotationIconType::Insert));
        assert_eq!(PdfMarkupType::from_raw(3), Some(PdfMarkupType::Redact));
    }

    #[test]
    fn view_related_enums_round_trip_raw_values() {
        assert_eq!(PdfDisplayMode::from_raw(3), Some(PdfDisplayMode::TwoUpContinuous));
        assert_eq!(PdfDisplayDirection::from_raw(1), Some(PdfDisplayDirection::Horizontal));
        assert_eq!(PdfInterpolationQuality::from_raw(2), Some(PdfInterpolationQuality::High));
        assert_eq!(PdfWidgetCellState::Mixed.as_raw(), -1);
        assert_eq!(PdfWidgetCellState::from_raw(-1), Some(PdfWidgetCellState::Mixed));
        assert_eq!(PdfSelectionGranularity::Line.as_raw(), 2);
        assert_eq!(PdfSelectionGranularity::from_raw(2), Some(PdfSelectionGranularity::Line));
        assert_eq!(PdfThumbnailLayoutMode::Horizontal.as_raw(), 1);
        assert_eq!(PdfThumbnailLayoutMode::from_raw(1), Some(PdfThumbnailLayoutMode::Horizontal));
    }

    #[test]
    fn area_of_interest_bitflags_combine_and_mask() {
        let mut area = PdfAreaOfInterest::PAGE | PdfAreaOfInterest::TEXT;
        area |= PdfAreaOfInterest::LINK;

        assert!(!PdfAreaOfInterest::NONE.intersects(PdfAreaOfInterest::PAGE));
        assert!(area.contains(PdfAreaOfInterest::PAGE));
        assert!(area.contains(PdfAreaOfInterest::TEXT));
        assert!(area.intersects(PdfAreaOfInterest::LINK));
        assert!(!area.contains(PdfAreaOfInterest::IMAGE));
        assert_eq!((area & PdfAreaOfInterest::TEXT).bits(), PdfAreaOfInterest::TEXT.bits());
        assert!(!PdfAreaOfInterest::NONE.bits().eq(&PdfAreaOfInterest::ANY.bits()));
    }

    #[test]
    fn builder_options_capture_selected_values() {
        let image_options = PdfPageImageInitializationOptions::default()
            .with_media_box(PdfRect {
                x: 0.0,
                y: 1.0,
                width: 200.0,
                height: 100.0,
            })
            .with_rotation(90)
            .with_upscale_if_smaller(true)
            .with_compression_quality(0.8);
        let write_options = PdfDocumentWriteOptions::default()
            .with_owner_password("owner")
            .with_user_password("user")
            .with_access_permissions(0x15)
            .with_burn_in_annotations(true)
            .with_save_text_from_ocr(true)
            .with_save_images_as_jpeg(true)
            .with_optimize_images_for_screen(true);

        assert_eq!(image_options.rotation, Some(90));
        assert!(image_options.upscale_if_smaller);
        assert_close(
            image_options
                .compression_quality
                .expect("compression quality should be set"),
            0.8,
        );
        assert_eq!(write_options.owner_password.as_deref(), Some("owner"));
        assert_eq!(write_options.user_password.as_deref(), Some("user"));
        assert_eq!(write_options.access_permissions, Some(0x15));
        assert!(write_options.burn_in_annotations);
        assert!(write_options.save_text_from_ocr);
        assert!(write_options.save_images_as_jpeg);
        assert!(write_options.optimize_images_for_screen);
    }
}
