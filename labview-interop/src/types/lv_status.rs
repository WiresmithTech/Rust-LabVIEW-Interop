//! Module for handling of LabVIEW status codes.
//!
//! Although not a unique type in LabVIEW, the status code holds special semantic meaning
//! which is why it is given its own type.

use crate::errors::{LVInteropError, MgError};
use crate::labview;
use crate::types::LStrHandle;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
use std::mem::MaybeUninit;

/// ´LVStatusCode´ is a transparent newtype on i32 to represent all potential error codes and SUCCESS (0) as a success value.
///
/// This kind of status code corresponds to the Rust Result types.
/// Therefore, it is named status and not error on purpose. There is no checks or guarantees if the code is a valid range or has an official labview
/// definition.
///
/// # Examples
///
/// ```
/// use labview_interop::types::LVStatusCode;
/// let status = LVStatusCode::from(42);
///
/// assert_eq!(status, 42.into());
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LVStatusCode(i32);

impl Display for LVStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(not(feature = "link"))]
        write!(f, "LVStatusCode: {}", self.0)?;
        #[cfg(feature = "link")]
        write!(f, "LVStatusCode: {} - {}", self.0, self.description())?;
        Ok(())
    }
}

impl Error for LVStatusCode {}

impl LVStatusCode {
    pub const SUCCESS: LVStatusCode = LVStatusCode(0);

    ///this will convert the LVStatusCode to either Ok(T) or Err(LVInteropError(LabviewMgError)) or Err(LVInteropError)
    ///mostly for our internal use
    pub(crate) fn to_specific_result<T>(self, success_value: T) -> crate::errors::Result<T> {
        if self == Self::SUCCESS {
            Ok(success_value)
        } else {
            match MgError::try_from(self) {
                Ok(mg_err) => Err(mg_err.into()),
                Err(inter_err) => Err(inter_err),
            }
        }
    }

    /// this will convert the LVStatusCode to the generic LVError with no checks of validity
    pub fn to_generic_result<T>(self, success_value: T) -> Result<T, LVInteropError> {
        if self == Self::SUCCESS {
            Ok(success_value)
        } else {
            Err(LVInteropError::LabviewError(self))
        }
    }
}

// From<i32> vice versa implemented, but not Deref (do not want to inherit other math operations)
impl From<i32> for LVStatusCode {
    fn from(value: i32) -> LVStatusCode {
        LVStatusCode(value)
    }
}

impl From<LVStatusCode> for i32 {
    fn from(code: LVStatusCode) -> i32 {
        code.0
    }
}

#[cfg(feature = "link")]
impl LVStatusCode {
    pub fn description(&self) -> Cow<'static, str> {
        static DEFAULT_STRING: &str = "LabVIEW-Interop: Description not retrievable";
        let mut error_text_ptr = MaybeUninit::<LStrHandle>::uninit();

        let memory_api = match labview::memory_api() {
            Ok(api) => api,
            Err(_) => return Cow::Borrowed(DEFAULT_STRING),
        };

        unsafe {
            if memory_api.error_code_description(self.0, error_text_ptr.as_mut_ptr() as *mut usize)
            {
                let error_text_ptr = error_text_ptr.assume_init();
                let desc = error_text_ptr.to_rust_string().to_string();
                return Cow::Owned(desc);
            }
        }
        Cow::Borrowed(DEFAULT_STRING)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_lvstatuscode_from_i32() {
        let status = LVStatusCode::from(0);
        assert_eq!(status, LVStatusCode::SUCCESS);

        let status = LVStatusCode::from(1);
        assert_eq!(status, LVStatusCode(1));

        let status: LVStatusCode = 42.into();
        assert_eq!(status, LVStatusCode(42));
    }

    // test transparency of status type
    #[test]
    fn test_error_lvstatuscode_from_externc() {
        // Mock the external C function
        unsafe extern "C" fn mock_externc() -> i32 {
            542_002 // Simulate a C function returning an `i32`
        }

        fn post_lv_user_event_safe() -> LVStatusCode {
            let result: i32 = unsafe { mock_externc() };

            // Transmute the i32 result to LVStatusCode
            unsafe { std::mem::transmute(result) }
        }

        let lv_status = post_lv_user_event_safe();

        assert_eq!(lv_status, LVStatusCode(542_002));
    }
}
