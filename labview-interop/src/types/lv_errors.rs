//! Functions for working with the LabVIEW error clusters.
//!
//! This is only available in 64 bit currently due to restrictions
//! on unaligned pointer access.
#[cfg(feature = "link")]
use crate::errors::Result;
use crate::errors::{LVInteropError, MgError};
use crate::labview_layout;
use crate::memory::UPtr;
use crate::types::LStrHandle;
use crate::types::LVBool;
use crate::types::LVStatusCode;
use std::borrow::Cow;

labview_layout!(
    /// The cluster format used by LabVIEW for transmitting errors.
    pub struct ErrorCluster<'a> {
        status: LVBool,
        code: LVStatusCode,
        source: LStrHandle<'a>,
    }
);

impl ErrorCluster<'_> {
    /// Does the error cluster contain an error.
    pub fn is_err(&self) -> bool {
        self.status.into()
    }
}

/// The pointer as passed by LabVIEW when using "Handles By Value" for type.
///
/// Debugging shows only one level of indirection hence UPtr here.
///
/// It is recommended to manually call `ErrorClusterPtr::as_ref` or `ErrorClusterPtr::as_mut`
/// so that null pointers can be detected.
///
/// Many string manipulation functions are only available with the `link` feature enabled so
/// it can manipulate LabVIEW Strings.
pub type ErrorClusterPtr<'a> = UPtr<ErrorCluster<'a>>;

/// Format the source and description into a string that LabVIEW will interpret.
// Only used in link but sat outside the module to make testing easier.
#[cfg(any(test, feature = "link"))]
fn format_error_source(source: &str, description: &str) -> String {
    match (source, description) {
        ("", description) => format!("<ERR>\n{description}"),
        (source, "") => source.to_string(),
        (source, description) => format!("{source}\n<ERR>\n{description}"),
    }
}

#[cfg(feature = "link")]
impl ErrorClusterPtr<'_> {
    /// Wrap the provided function in error handling to match LabVIEW semantics.
    ///
    /// i.e. no execution on error in, convert return errors into error cluster.
    ///
    /// ## Parameters
    ///
    /// - `return_on_error` - The value to return if an error.
    /// - `function` - The function to wrap. This is intended to be a closure for
    ///   easy use.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use labview_interop::types::ErrorClusterPtr;
    /// use labview_interop::errors::LVInteropError;
    /// use labview_interop::types::LStrHandle;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn example_function(mut error_cluster: ErrorClusterPtr, mut string_input: LStrHandle) -> i32 {
    ///   error_cluster.wrap_function(42, || -> Result<i32, LVInteropError> {
    ///    // Do some work
    ///    string_input.set_str("Hello World")?;
    ///   Ok(42)
    ///  })
    /// }
    /// ```
    pub fn wrap_function<R, E: ToLvError, F: FnOnce() -> std::result::Result<R, E>>(
        &mut self,
        return_on_error: R,
        function: F,
    ) -> R {
        if self.is_err() {
            return return_on_error;
        }
        match function() {
            Ok(value) => value,
            Err(error) => {
                let _ = error.write_error(self);
                return_on_error
            }
        }
    }

    /// Wrap the provided function in error handling to match LabVIEW semantics.
    ///
    /// i.e. no execution on error in, convert return errors into error cluster.
    ///
    /// This version returns the LabVIEW status code of the error.
    /// To return a different value, see [`ErrorClusterPtr::wrap_function`].
    ///
    /// ## Parameters
    ///
    /// - `function` - The function to wrap. This is intended to be a closure for
    ///   easy use.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use labview_interop::types::{ErrorClusterPtr, LVStatusCode};
    /// use labview_interop::errors::LVInteropError;
    /// use labview_interop::types::LStrHandle;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn example_function(mut error_cluster: ErrorClusterPtr, mut string_input: LStrHandle) -> LVStatusCode {
    ///   error_cluster.wrap_return_status(|| -> Result<(), LVInteropError> {
    ///    // Do some work
    ///    string_input.set_str("Hello World")?;
    ///     Ok(())
    ///
    ///  })
    /// }
    /// ```
    pub fn wrap_return_status<E: ToLvError, F: FnOnce() -> std::result::Result<(), E>>(
        &mut self,
        function: F,
    ) -> LVStatusCode {
        if self.is_err() {
            return self.code;
        }
        self.wrap_function((), function);
        self.code
    }
}

#[cfg(feature = "link")]
mod error_cluster_link_features {
    use super::*;
    use crate::errors::Result;
    use crate::types::boolean::{LV_FALSE, LV_TRUE};

    impl ErrorCluster<'_> {
        /// Set a description and source in the format that LabVIEW will interpret for display.
        fn set_source(&mut self, source: &str, description: &str) -> Result<()> {
            // Probably a clever way to avoid this allocation but for now we will take it.
            let full_source = format_error_source(source, description);
            self.source.set_str(&full_source)
        }

        /// Set the error cluster to a warning state.
        pub fn set_warning(
            &mut self,
            code: LVStatusCode,
            source: &str,
            description: &str,
        ) -> Result<()> {
            self.code = code;
            self.status = LV_FALSE;
            self.set_source(source, description)
        }

        /// Set the error cluster to an error state.
        pub fn set_error(
            &mut self,
            code: LVStatusCode,
            source: &str,
            description: &str,
        ) -> Result<()> {
            self.code = code;
            self.status = LV_TRUE;
            self.set_source(source, description)
        }
    }
}

/// A trait that can be implemented on types to allow them to be written into a
/// error cluster with `ToLvError::write_error`.
pub trait ToLvError {
    /// The code for the error. Default is 42.
    fn code(&self) -> LVStatusCode {
        (&MgError::BogusError).into() // code 42, Generic Error
    }

    /// True if is error. Default is true.
    fn is_error(&self) -> bool {
        true
    }

    /// The source of the error if available. Default: none.
    fn source(&self) -> Cow<'_, str> {
        "".into()
    }

    /// The description of the error;
    fn description(&self) -> Cow<'_, str>;

    /// Write into the LabVIEW Error Pointer.
    ///
    /// The pointer is the type that is recieved through the Call Library Node so
    /// there is no need to deal with references before this point.
    ///
    /// This requires the `link` feature to enable string manipulation.
    #[cfg(feature = "link")]
    fn write_error(&self, error_cluster: &mut ErrorClusterPtr) -> Result<()> {
        let cluster = unsafe { error_cluster.as_ref_mut()? };
        let code = self.code();
        let source = self.source();
        let source = source.as_ref();
        let description = self.description();
        let description = description.as_ref();
        if self.is_error() {
            cluster.set_error(code, source, description)
        } else {
            cluster.set_warning(code, source, description)
        }
    }
}

impl ToLvError for LVInteropError {
    fn code(&self) -> LVStatusCode {
        self.into()
    }
    fn source(&self) -> Cow<'_, str> {
        std::error::Error::source(self)
            .map(|s| s.to_string())
            .unwrap_or_default()
            .into()
    }

    fn description(&self) -> Cow<'_, str> {
        self.to_string().into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_source_writer_empty_description() {
        let source = format_error_source("Rust", "");
        assert_eq!(source, "Rust");
    }

    #[test]
    fn test_source_writer_with_description() {
        let source = format_error_source("Rust", "An Error Occured");
        let expected = "Rust\n<ERR>\nAn Error Occured";
        assert_eq!(source, expected)
    }

    #[test]
    fn test_source_writer_empty_source() {
        let source = format_error_source("", "An Error Occured");
        let expected = "<ERR>\nAn Error Occured";
        assert_eq!(source, expected)
    }
}
