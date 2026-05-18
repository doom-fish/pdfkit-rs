/// Wraps `PDFDocument notification` constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfDocumentNotification {
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidUnlock,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidBeginFind,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidEndFind,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidBeginPageFind,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidEndPageFind,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidFindMatch,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidBeginWrite,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidEndWrite,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidBeginPageWrite,
    /// Wraps the corresponding `PDFDocument` notification constant.
    DidEndPageWrite,
}

impl PdfDocumentNotification {
    /// Returns the corresponding PDFKit constant name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::DidUnlock => "PDFDocumentDidUnlockNotification",
            Self::DidBeginFind => "PDFDocumentDidBeginFindNotification",
            Self::DidEndFind => "PDFDocumentDidEndFindNotification",
            Self::DidBeginPageFind => "PDFDocumentDidBeginPageFindNotification",
            Self::DidEndPageFind => "PDFDocumentDidEndPageFindNotification",
            Self::DidFindMatch => "PDFDocumentDidFindMatchNotification",
            Self::DidBeginWrite => "PDFDocumentDidBeginWriteNotification",
            Self::DidEndWrite => "PDFDocumentDidEndWriteNotification",
            Self::DidBeginPageWrite => "PDFDocumentDidBeginPageWriteNotification",
            Self::DidEndPageWrite => "PDFDocumentDidEndPageWriteNotification",
        }
    }

    pub(crate) const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::DidUnlock),
            1 => Some(Self::DidBeginFind),
            2 => Some(Self::DidEndFind),
            3 => Some(Self::DidBeginPageFind),
            4 => Some(Self::DidEndPageFind),
            5 => Some(Self::DidFindMatch),
            6 => Some(Self::DidBeginWrite),
            7 => Some(Self::DidEndWrite),
            8 => Some(Self::DidBeginPageWrite),
            9 => Some(Self::DidEndPageWrite),
            _ => None,
        }
    }
}

/// Wraps `PDFDocument notification user-info key` constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfDocumentNotificationUserInfoKey {
    /// Wraps the corresponding `PDFDocument` user-info key.
    FoundSelection,
    /// Wraps the corresponding `PDFDocument` user-info key.
    PageIndex,
}

impl PdfDocumentNotificationUserInfoKey {
    /// Returns the corresponding PDFKit constant name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::FoundSelection => "PDFDocumentFoundSelection",
            Self::PageIndex => "PDFDocumentPageIndex",
        }
    }
}

/// Wraps `PDFView notification` constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfViewNotification {
    /// Wraps the corresponding `PDFView` notification constant.
    AnnotationHit,
    /// Wraps the corresponding `PDFView` notification constant.
    AnnotationWillHit,
    /// Wraps the corresponding `PDFView` notification constant.
    ChangedHistory,
    /// Wraps the corresponding `PDFView` notification constant.
    CopyPermission,
    /// Wraps the corresponding `PDFView` notification constant.
    DisplayBoxChanged,
    /// Wraps the corresponding `PDFView` notification constant.
    DisplayModeChanged,
    /// Wraps the corresponding `PDFView` notification constant.
    DocumentChanged,
    /// Wraps the corresponding `PDFView` notification constant.
    PageChanged,
    /// Wraps the corresponding `PDFView` notification constant.
    PrintPermission,
    /// Wraps the corresponding `PDFView` notification constant.
    ScaleChanged,
    /// Wraps the corresponding `PDFView` notification constant.
    SelectionChanged,
    /// Wraps the corresponding `PDFView` notification constant.
    VisiblePagesChanged,
}

impl PdfViewNotification {
    /// Returns the corresponding PDFKit constant name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::AnnotationHit => "PDFViewAnnotationHitNotification",
            Self::AnnotationWillHit => "PDFViewAnnotationWillHitNotification",
            Self::ChangedHistory => "PDFViewChangedHistoryNotification",
            Self::CopyPermission => "PDFViewCopyPermissionNotification",
            Self::DisplayBoxChanged => "PDFViewDisplayBoxChangedNotification",
            Self::DisplayModeChanged => "PDFViewDisplayModeChangedNotification",
            Self::DocumentChanged => "PDFViewDocumentChangedNotification",
            Self::PageChanged => "PDFViewPageChangedNotification",
            Self::PrintPermission => "PDFViewPrintPermissionNotification",
            Self::ScaleChanged => "PDFViewScaleChangedNotification",
            Self::SelectionChanged => "PDFViewSelectionChangedNotification",
            Self::VisiblePagesChanged => "PDFViewVisiblePagesChangedNotification",
        }
    }
}

/// Wraps `PDFThumbnailView notification` constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfThumbnailViewNotification {
    /// Wraps the corresponding `PDFThumbnailView` notification constant.
    DocumentEdited,
}

impl PdfThumbnailViewNotification {
    /// Returns the corresponding PDFKit constant name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::DocumentEdited => "PDFThumbnailViewDocumentEditedNotification",
        }
    }
}
