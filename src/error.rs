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

#[cfg(test)]
mod tests {
    use super::PdfKitError;

    #[test]
    fn constructor_accessors_and_display_match() {
        let error = PdfKitError::new(-3, "failed to parse action");

        assert_eq!(error.code(), -3);
        assert_eq!(error.message(), "failed to parse action");
        assert_eq!(error.to_string(), "failed to parse action (status -3)");
    }

    #[test]
    fn cloned_errors_preserve_payloads_and_have_no_source() {
        let error = PdfKitError::new(-5, "bad annotation");
        let cloned = error.clone();
        let as_std_error: &dyn std::error::Error = &cloned;

        assert_eq!(cloned, error);
        assert!(as_std_error.source().is_none());
    }
}
