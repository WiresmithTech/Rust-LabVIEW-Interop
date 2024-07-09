use labview_interop::errors::MgErr;
/// A simple type for testing the error integration.
///
use labview_interop::types::{ErrorClusterPtr, ToLvError};
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
pub extern "C" fn set_error_cluster(error_cluster: ErrorClusterPtr) -> MgErr {
    let error = ErrorText("This is a test");
    error.write_error(error_cluster).into()
}
