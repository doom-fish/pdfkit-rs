use std::error::Error;
use std::fmt;

/// Result alias used by the PDFKit wrappers.
pub type Result<T> = std::result::Result<T, PdfKitError>;

/// Wraps `PDFKitError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfKitError {
    code: i32,
    message: String,
}

impl PdfKitError {
    pub(crate) fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Returns the PDFKit framework status code.
    #[must_use]
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Returns the PDFKit framework error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PdfKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (status {})", self.message, self.code)
    }
}

impl Error for PdfKitError {}
