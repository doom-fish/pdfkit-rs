use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfEdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct PdfTextRange {
    pub location: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DisplayBox {
    MediaBox = 0,
    CropBox = 1,
    BleedBox = 2,
    TrimBox = 3,
    ArtBox = 4,
}

impl DisplayBox {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfBorderStyle {
    Solid = 0,
    Dashed = 1,
    Beveled = 2,
    Inset = 3,
    Underline = 4,
}

impl PdfBorderStyle {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfActionNamedName {
    None = 0,
    NextPage = 1,
    PreviousPage = 2,
    FirstPage = 3,
    LastPage = 4,
    GoBack = 5,
    GoForward = 6,
    GoToPage = 7,
    Find = 8,
    Print = 9,
    ZoomIn = 10,
    ZoomOut = 11,
}

impl PdfActionNamedName {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfLineStyle {
    None = 0,
    Square = 1,
    Circle = 2,
    Diamond = 3,
    OpenArrow = 4,
    ClosedArrow = 5,
}

impl PdfLineStyle {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfMarkupType {
    Highlight = 0,
    StrikeOut = 1,
    Underline = 2,
    Redact = 3,
}

impl PdfMarkupType {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfDisplayMode {
    SinglePage = 0,
    SinglePageContinuous = 1,
    TwoUp = 2,
    TwoUpContinuous = 3,
}

impl PdfDisplayMode {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfDisplayDirection {
    Vertical = 0,
    Horizontal = 1,
}

impl PdfDisplayDirection {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Vertical),
            1 => Some(Self::Horizontal),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfInterpolationQuality {
    None = 0,
    Low = 1,
    High = 2,
}

impl PdfInterpolationQuality {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfWidgetControlType {
    Unknown = -1,
    PushButton = 0,
    RadioButton = 1,
    CheckBox = 2,
}

impl PdfWidgetControlType {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PdfDocumentPermissions {
    None = 0,
    User = 1,
    Owner = 2,
}

impl PdfDocumentPermissions {
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

#[derive(Debug, Clone, Deserialize)]
pub struct PdfDocumentInfo {
    pub document_url: Option<String>,
    pub major_version: i32,
    pub minor_version: i32,
    pub is_encrypted: bool,
    pub is_locked: bool,
    pub permissions_status: i32,
    pub access_permissions: u64,
    pub allows_printing: bool,
    pub allows_copying: bool,
    pub allows_document_changes: bool,
    pub allows_document_assembly: bool,
    pub allows_content_accessibility: bool,
    pub allows_commenting: bool,
    pub allows_form_field_entry: bool,
    pub page_class: String,
}

impl PdfDocumentInfo {
    #[must_use]
    pub fn permissions_status_enum(&self) -> Option<PdfDocumentPermissions> {
        PdfDocumentPermissions::from_raw(self.permissions_status)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PdfDocumentAttributes {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct PdfDocumentWriteOptions {
    pub owner_password: Option<String>,
    pub user_password: Option<String>,
    pub access_permissions: Option<u64>,
    pub burn_in_annotations: bool,
    pub save_text_from_ocr: bool,
    pub save_images_as_jpeg: bool,
    pub optimize_images_for_screen: bool,
}

impl PdfDocumentWriteOptions {
    pub fn with_owner_password(mut self, value: impl Into<String>) -> Self {
        self.owner_password = Some(value.into());
        self
    }

    pub fn with_user_password(mut self, value: impl Into<String>) -> Self {
        self.user_password = Some(value.into());
        self
    }

    pub fn with_access_permissions(mut self, value: u64) -> Self {
        self.access_permissions = Some(value);
        self
    }

    pub fn with_burn_in_annotations(mut self, value: bool) -> Self {
        self.burn_in_annotations = value;
        self
    }

    pub fn with_save_text_from_ocr(mut self, value: bool) -> Self {
        self.save_text_from_ocr = value;
        self
    }

    pub fn with_save_images_as_jpeg(mut self, value: bool) -> Self {
        self.save_images_as_jpeg = value;
        self
    }

    pub fn with_optimize_images_for_screen(mut self, value: bool) -> Self {
        self.optimize_images_for_screen = value;
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PdfBorderInfo {
    pub style: i32,
    pub line_width: f64,
    pub dash_pattern: Option<Vec<f64>>,
}

impl PdfBorderInfo {
    #[must_use]
    pub fn style_enum(&self) -> Option<PdfBorderStyle> {
        PdfBorderStyle::from_raw(self.style)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PdfAnnotationInfo {
    pub annotation_type: Option<String>,
    pub bounds: PdfRect,
    pub contents: Option<String>,
    pub should_display: bool,
    pub should_print: bool,
    pub has_appearance_stream: bool,
    pub user_name: Option<String>,
    pub modification_date: Option<String>,
    pub color: Option<PdfColor>,
    pub highlighted: bool,
    pub action_type: Option<String>,
    pub border: Option<PdfBorderInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PdfDestinationInfo {
    pub page_label: Option<String>,
    pub page_index: Option<usize>,
    pub point: PdfPoint,
    pub zoom: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PdfAppearanceCharacteristicsInfo {
    pub control_type: i32,
    pub background_color: Option<PdfColor>,
    pub border_color: Option<PdfColor>,
    pub rotation: i32,
    pub caption: Option<String>,
    pub rollover_caption: Option<String>,
    pub down_caption: Option<String>,
}

impl PdfAppearanceCharacteristicsInfo {
    #[must_use]
    pub fn control_type_enum(&self) -> Option<PdfWidgetControlType> {
        PdfWidgetControlType::from_raw(self.control_type)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PdfViewInfo {
    pub display_mode: i32,
    pub display_direction: i32,
    pub displays_page_breaks: bool,
    pub page_break_margins: PdfEdgeInsets,
    pub display_box: i32,
    pub displays_as_book: bool,
    pub displays_rtl: bool,
    pub background_color: Option<PdfColor>,
    pub interpolation_quality: i32,
    pub page_shadows_enabled: bool,
    pub scale_factor: f64,
    pub min_scale_factor: f64,
    pub max_scale_factor: f64,
    pub auto_scales: bool,
    pub scale_factor_for_size_to_fit: f64,
    pub in_markup_mode: bool,
    pub has_document_view: bool,
    pub visible_page_count: usize,
    pub current_page_label: Option<String>,
}

impl PdfViewInfo {
    #[must_use]
    pub fn display_mode_enum(&self) -> Option<PdfDisplayMode> {
        PdfDisplayMode::from_raw(self.display_mode)
    }

    #[must_use]
    pub fn display_direction_enum(&self) -> Option<PdfDisplayDirection> {
        PdfDisplayDirection::from_raw(self.display_direction)
    }

    #[must_use]
    pub fn interpolation_quality_enum(&self) -> Option<PdfInterpolationQuality> {
        PdfInterpolationQuality::from_raw(self.interpolation_quality)
    }

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

#[derive(Debug, Clone, Deserialize)]
pub struct PdfThumbnailViewInfo {
    pub has_pdf_view: bool,
    pub background_color: Option<PdfColor>,
    pub selected_pages_count: usize,
    pub thumbnail_size: PdfSize,
    pub maximum_number_of_columns: usize,
    pub allows_dragging: bool,
    pub allows_multiple_selection: bool,
}
