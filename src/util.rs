use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::{PdfKitError, Result};
use crate::ffi;

pub(crate) fn c_string(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        PdfKitError::new(
            ffi::status::INVALID_ARGUMENT,
            "string contained interior NUL",
        )
    })
}

pub(crate) fn option_c_string(value: Option<&str>) -> Result<Option<CString>> {
    value.map(c_string).transpose()
}

pub(crate) fn path_to_c_string(path: &Path) -> Result<CString> {
    c_string(path.to_string_lossy().as_ref())
}

/// Takes ownership of a C string allocated by the Swift bridge and converts it to a Rust String,
/// then frees the original C string.
///
/// # Safety
/// `ptr` must be either null or a valid, non-null pointer to a null-terminated C string allocated
/// by the Swift bridge using `malloc` or compatible allocator. The pointer must not be used after
/// this function returns as the memory is freed.
pub(crate) fn take_string(ptr: *mut c_char) -> Option<String> {
    // SAFETY: caller guarantees `ptr` is null or a valid bridge-allocated C string.
    unsafe { doom_fish_utils::ffi_string::take_owned_cstring_c(ptr, |p| libc::free(p.cast::<c_void>())) }
}

pub(crate) fn status_result(status: i32, error_ptr: *mut c_char) -> Result<()> {
    if status == ffi::status::OK {
        return Ok(());
    }

    let message = take_string(error_ptr).unwrap_or_else(|| format!("PDFKit bridge error {status}"));
    Err(PdfKitError::new(status, message))
}

pub(crate) fn required_handle(
    ptr: *mut c_void,
    context: &'static str,
) -> Result<crate::handle::ObjectHandle> {
    // SAFETY: ptr is either null (checked below) or a retained PDFKit object pointer from the Swift bridge
    unsafe { crate::handle::ObjectHandle::from_retained_ptr(ptr) }.ok_or_else(|| {
        PdfKitError::new(ffi::status::NULL_RESULT, format!("{context} returned null"))
    })
}

pub(crate) fn parse_json<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &'static str,
) -> Result<T> {
    let json = take_string(ptr).ok_or_else(|| {
        PdfKitError::new(ffi::status::NULL_RESULT, format!("{context} returned null"))
    })?;
    serde_json::from_str(&json).map_err(|error| {
        PdfKitError::new(
            ffi::status::FRAMEWORK,
            format!("failed to parse {context} JSON: {error}"),
        )
    })
}
