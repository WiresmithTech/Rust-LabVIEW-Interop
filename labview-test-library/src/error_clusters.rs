use labview_interop::errors::LVInteropError;
/// A simple type for testing the error integration.
///
use labview_interop::types::{ErrorClusterPtr, ToLvError};
use labview_interop::types::{LStrHandle, LVStatusCode};
struct ErrorText(&'static str);

#[cfg(target_pointer_width = "64")]
impl ToLvError for ErrorText {
    fn source(&self) -> std::borrow::Cow<'_, str> {
        "Rust".into()
    }

    fn description(&self) -> std::borrow::Cow<'_, str> {
        self.0.into()
    }
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn set_error_cluster(mut error_cluster: ErrorClusterPtr) -> LVStatusCode {
    let error = ErrorText("This is a test");
    error.write_error(&mut error_cluster).into()
}

/// Check scenarios for the error cluster function wrapping.
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn wrap_function(
    mut error_cluster: ErrorClusterPtr,
    mut text: LStrHandle,
    inner_error: i16,
) -> i32 {
    error_cluster.wrap_function(42, || -> Result<i32, LVInteropError> {
        text.set_str("Hello World")?;
        if inner_error != 0 {
            return Err(LVInteropError::from(LVStatusCode::from(1)));
        }
        Ok(0)
    })
}

#[no_mangle]
pub extern "C" fn get_error_description(code: i32, mut string: LStrHandle) {
    let status = LVStatusCode::from(code);
    let description = status.description();
    string.set_str(description.as_ref()).unwrap();
}
