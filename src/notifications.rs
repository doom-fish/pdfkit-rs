#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfDocumentNotification {
    DidUnlock,
    DidBeginFind,
    DidEndFind,
    DidBeginPageFind,
    DidEndPageFind,
    DidFindMatch,
    DidBeginWrite,
    DidEndWrite,
    DidBeginPageWrite,
    DidEndPageWrite,
}

impl PdfDocumentNotification {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfDocumentNotificationUserInfoKey {
    FoundSelection,
    PageIndex,
}

impl PdfDocumentNotificationUserInfoKey {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::FoundSelection => "PDFDocumentFoundSelection",
            Self::PageIndex => "PDFDocumentPageIndex",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfViewNotification {
    AnnotationHit,
    AnnotationWillHit,
    ChangedHistory,
    CopyPermission,
    DisplayBoxChanged,
    DisplayModeChanged,
    DocumentChanged,
    PageChanged,
    PrintPermission,
    ScaleChanged,
    SelectionChanged,
    VisiblePagesChanged,
}

impl PdfViewNotification {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfThumbnailViewNotification {
    DocumentEdited,
}

impl PdfThumbnailViewNotification {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::DocumentEdited => "PDFThumbnailViewDocumentEditedNotification",
        }
    }
}
