use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PdfRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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
    pub allows_printing: bool,
    pub allows_copying: bool,
    pub allows_document_changes: bool,
    pub allows_document_assembly: bool,
    pub allows_content_accessibility: bool,
    pub allows_commenting: bool,
    pub allows_form_field_entry: bool,
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

#[derive(Debug, Clone, Deserialize)]
pub struct PdfAnnotationInfo {
    pub annotation_type: Option<String>,
    pub bounds: PdfRect,
    pub contents: Option<String>,
    pub should_display: bool,
    pub should_print: bool,
    pub has_appearance_stream: bool,
    pub user_name: Option<String>,
}
