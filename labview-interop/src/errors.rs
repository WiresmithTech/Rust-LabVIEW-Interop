use std::{error::Error, fmt::Display};
use thiserror::Error;

/// MgErr is a simple wrapper around the error code that
/// is returned by the memory manager functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MgErr(i32);

impl From<i32> for MgErr {
    fn from(value: i32) -> MgErr {
        MgErr(value)
    }
}

impl MgErr {
    pub const NO_ERROR: MgErr = MgErr(0);
    pub const INTEROP_ERROR: MgErr = MgErr(-1);
    pub const MEMORY_FULL: MgErr = MgErr(2);
    pub fn to_result<T>(self, success_value: T) -> Result<T> {
        if self.0 != 0 {
            Err(self.into())
        } else {
            Ok(success_value)
        }
    }

    fn get_description(&self) -> &'static str {
        match self.0 {
            0 => "No Error",
            2 => "Memory Full",
            _ => "No Description for Code",
        }
    }
}

impl Display for MgErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.get_description())
    }
}

impl Error for MgErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Error, Debug)]
pub enum LVInteropError {
    #[error("Internal LabVIEW Error: {0}")]
    LabviewError(#[from] MgErr),
    #[error("Invalid handle when valid handle is required")]
    InvalidHandle,
    #[error("LabVIEW API unavailable. Probably because it isn't being run in LabVIEW")]
    NoLabviewApi,
    #[error("LabVIEW arrays can only have dimensions of i32 range.")]
    ArrayDimensionsOutOfRange,
    #[error(
        "Array dimensions don't match. You may require the link feature to enable auto-resizing."
    )]
    ArrayDimensionMismatch,
    #[error("Creating of handle in LabVIEW memory manager failed. Perhaps you are out of memory?")]
    HandleCreationFailed,
}

pub type Result<T> = std::result::Result<T, LVInteropError>;

impl From<LVInteropError> for MgErr {
    fn from(value: LVInteropError) -> Self {
        match value {
            LVInteropError::LabviewError(err) => err,
            LVInteropError::InvalidHandle => MgErr::INTEROP_ERROR,
            LVInteropError::NoLabviewApi => MgErr(-2),
            LVInteropError::ArrayDimensionsOutOfRange => MgErr(-3),
            LVInteropError::ArrayDimensionMismatch => MgErr(-3),
            LVInteropError::HandleCreationFailed => MgErr(-4),
        }
    }
}

impl<T> From<Result<T>> for MgErr {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(_) => MgErr::NO_ERROR,
            Err(err) => err.into(),
        }
    }
}
