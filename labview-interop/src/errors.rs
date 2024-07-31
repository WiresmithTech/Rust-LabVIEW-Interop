//! # Error handling in LabVIEW Interop
//!
//! There are four error paths that the labview-interop crate needs to handle:
//!
//! 1. **MgError and MgErrorCode**, Internal: LabView Memory Manager --> Rust
//!    This crate calls the labview memory manager internally to deal with memory
//!    owned by LabVIEW. The functions of the memory manager return MgErr. The documentation
//!    on https://www.ni.com/docs/en-US/bundle/labview/page/labview-manager-function-errors.html
//!    gives a full list of possible error values.
//!
//! 2. **MgErrorCode**: to LabVIEW through function return
//!    We want to be able to return the errors genated internally through the function return and be
//!    understood on the LabVIEW side. This is straight forward for Errors of MgError type. But we will
//!    have an internal compound Error type that can have a different type. When using status returns, these
//!    can only be converted to a very generic error code. Therefore 3
//!
//! 3. **InteropError**: to LabVIEW through ErrorCluster parameter
//!    Our internal `LvInteropError` compound error can easily be converted to an ErrorCluster. For MgErrors the conversion is
//!    straight forward. The correct source descriptions are gotten from the memory manager through `NIGetOneErrorCode`.
//!    For non LV errors, a generic error is leveraged, and the source description is overwritten.
//!
//! 4. from LabVIEW through ErrorCluster parameter
//!    Will labview-interop ever need to make sense of an error? It may be good enough to differentiate between an error and
//!    warnings. TBD
//!
//! # Notes on Error Handling in LabVIEW
//! This section is a sumary of defined and observed LabVIEW behaviour
//!
//! ## Labview error clusters and data types
//! THe labview error clusters possess an error code of i32. The error lists on labviewwiki show
//! official Labview errors as low as 0x8000000A and as high as 0x3FFC0105.
//!
//! ## the Labview Memory Manager / MgErr and types
//! The memory manager code examples from the documentation call the return value of the c function calls ´MgErr´ of type i32
//!
//! ## Custom error ranges
//! Custom defined errors can range from
//! * -8999 through -8000
//! * 5000 through 9999
//! * 500,000 through 599,999
//! For obvious reasons the labview interop crate will use the **range 542,000 to 542,999** for errors that are generated
//! internally and not handed down by c functions
//!
//! # Error Implementation
//! There is a hierarcy of Status and Errors
//! - A status encode Success and Error
//!
//! These two are very generic and not bound to our crate:
//! LVStatusCode - a simple i32 code that can be returned from any c function, no checks
//! LVError - a generic i32 error that can get the official description through the memory manager
//!           this is the basis for creating a LVErrorCluster
//!
//! The errors we expect to receive from calls to labview functions are
//! MgErrorCode
//! MgError
//!
//! Our generic Error handling is an enum
//! LabviewInteropError
//! This enum has custom errors for our internal use, we can hold MgErrors, and as last resort we can also hold
//! LVError

use std::{borrow::Cow, error::Error, fmt::Display, mem::MaybeUninit};
use thiserror::Error;

use crate::labview;
use crate::types::LStrHandle;

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// ´LVStatusCode´ is a newtype on i32 to represent all potential error codes and 0 as a success value. Therefore it
/// is named status and not error on purpose. There is no checks or guarantees if the code is a valid range or has an official labview
/// definition.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LVStatusCode(i32);

impl LVStatusCode {
    pub const SUCCESS: LVStatusCode = LVStatusCode(0);

    //this will convert the LVStatusCode to either Ok(T) or Err(LVInteropError(LabviewMgError)) or Err(LVInteropError)
    //mostly for our internal use
    fn to_specific_result<T>(self, success_value: T) -> Result<T> {
        if self == Self::SUCCESS {
            Ok(success_value)
        } else {
            match MgErrorCode::try_from(self) {
                Ok(mg_err) => Err(MgError::from(mg_err).into()),
                Err(inter_err) => Err(inter_err),
            }
        }
    }

    // this will convert the LVStatusCode to the generic LVError with no checks of validity
    fn to_generic_result<T>(self, success_value: T) -> core::result::Result<T, LVError> {
        if self == Self::SUCCESS {
            Ok(success_value)
        } else {
            Err(LVError { code: self })
        }
    }
}

/* not all LVInteropErrors have an equivalent
impl<T> From<Result<T>> for LVStatusCode {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(_) => LVStatusCode::SUCCESS,
            Err(err) => err.into(),
        }
    }
}*/

impl<T> From<core::result::Result<T, LVError>> for LVStatusCode {
    fn from(value: core::result::Result<T, LVError>) -> Self {
        match value {
            Ok(_) => LVStatusCode::SUCCESS,
            Err(err) => err.into(),
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

impl From<LVError> for LVStatusCode {
    fn from(err: LVError) -> LVStatusCode {
        err.code
    }
}

impl Display for LVStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// LVError is a generic Labview Error
/// that can retrieve a error description if the link feature is enabled
#[derive(Debug, Clone, Copy, Eq, PartialEq, Error)]
pub struct LVError {
    code: LVStatusCode,
}

impl LVError {
    #[cfg(feature = "link")]
    pub fn description(&self) -> Cow<'static, str> {
        static DEFAULT_STRING: &str = "LabVIEW-Interop: Description not retrievable";
        let mut error_text_ptr = MaybeUninit::<LStrHandle>::uninit();

        let memory_api = match labview::memory_api() {
            Ok(api) => api,
            Err(_) => return Cow::Borrowed(DEFAULT_STRING),
        };

        unsafe {
            if memory_api
                .error_code_description(self.code.0, error_text_ptr.as_mut_ptr() as *mut usize)
            {
                let error_text_ptr = error_text_ptr.assume_init();
                let desc = error_text_ptr.to_rust_string().to_string();
                return Cow::Owned(desc);
            }
        }
        Cow::Borrowed(DEFAULT_STRING)
    }
}

/* no From, as there is no translation on LVStatusCode == SUCCESS
impl From<LVStatusCode> for LVError {
    fn from(code: LVStatusCode) -> LVError {
        LVError { code }
    }
}*/

impl Display for LVError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.description())
    }
}

//pub type MgErr = LVStatusCode;

/// The `MgError` / `MgErrorCode` implement From in both directions. Additionally IntoPrimitive and TryFromPrimitive is derived
/// to enable the conversion from and to int primitives.
///
/// LabVIEW official general error list:
/// https://www.ni.com/docs/en-US/bundle/labview-api-ref/page/errors/general-labview-error-codes.html
/// more complete inofficial lists:
/// https://labviewwiki.org/wiki/LabVIEW_Error_Code_Family   /  https://labviewwiki.org/wiki/List_of_errors
///
/// Accoring to https://www.ni.com/docs/en-US/bundle/labview/page/labview-manager-function-errors.html
/// the memory manager only uses a subset of this huge error list. The subset is implemented in `MgError` using
/// the official abbreviations.

// Macro to define the MgError and MgErrorCode and the From conversions
macro_rules! define_errors {
    ($(($name:ident, $code:expr, $msg:expr)),*) => {
        /// `MgErrorCode` is an enum of all error codes listed
        /// in https://www.ni.com/docs/en-US/bundle/labview/page/labview-manager-function-errors.html
        #[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
        #[repr(i32)]
        pub enum MgErrorCode {
            $(
                $name = $code,
            )*
        }

        /// `MgError` implements Error on top of the `MgErrorCode` and includes a description
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
        pub enum MgError {
            $(
                #[error($msg)]
                $name,
            )*
        }

        impl From<MgErrorCode> for MgError {
            fn from(code: MgErrorCode) -> Self {
                match code {
                    $(
                        MgErrorCode::$name => MgError::$name,
                    )*
                }
            }
        }

        impl From<MgError> for MgErrorCode {
            fn from(error: MgError) -> Self {
                match error {
                    $(
                        MgError::$name => MgErrorCode::$name,
                    )*
                }
            }
        }
    };
}

impl TryFrom<LVStatusCode> for MgErrorCode {
    type Error = LVInteropError;
    fn try_from(status: LVStatusCode) -> ::core::result::Result<Self, Self::Error> {
        // SUCCESS is not a valid error!
        if status == LVStatusCode::SUCCESS {
            return Err(LVInteropError::InvalidMgErrorCode);
        }
        match MgErrorCode::try_from_primitive(status.0) {
            Ok(code) => Ok(code),
            Err(_) => Err(LVInteropError::InvalidMgErrorCode),
        }
    }
}

// impl MgStatus {
//     fn to_interop_result(self) -> std::result::Result<(), LVInteropError> {
//         if self == MgStatus(0) {
//             Ok(())
//         } else {
//             let code = MgErrorCode::try_from_primitive(self.0).expect("We implement all possible memory manager error codes, this conversion should therefore succeed.");
//             Err(code.into())
//         }
//     }
// }

define_errors!(
    (MgArgErr, 1, "An input parameter is invalid."),
    (MFullErr, 2, "Memory is full."),
    (FEof, 4, "End of file encountered."),
    (FIsOpen, 5, "File already open"),
    (FIoErr, 6, "Generic file I/O error."),
    (FNotFound, 7, "File not found"),
    (FNoPerm, 8, "File permission error."),
    (FDiskFull, 9, "Disk full"),
    (FDupPath, 10, "Duplicate path"),
    (FtMFOpen, 11, "Too many files open."),
    (FNotEnabled, 12, "Some system capacity necessary for operation is not enabled."),
    (RfNotFound, 13, "Failed to load dynamic library because of missing external symbols or dependencies, or because of an invalid file format."),
    (RAddFailed, 14, "Cannot add resource."),
    (RNotFound, 15, "Resource not found."),
    (INotFound, 16, "Image not found."),
    (IMemoryErr, 17, "Not enough memory to manipulate image."),
    (DPenNotExist, 18, "DPen does not exist."),
    (CfgBadType, 19, "Configuration type invalid."),
    (CfgTokenNotFound, 20, "Configuration token not found."),
    (CfgParseError, 21, "Error occurred parsing configuration string."),
    (CfgAllocError, 22, "Configuration memory error."),
    (EcLVSBFormatError, 23, "Bad external code format."),
    (EcLVSBSubrError, 24, "External subroutine not supported."),
    (EcLVSBNoCodeError, 25, "External code not present."),
    (WNullWindow, 26, "Null window."),
    (WDestroyMixup, 27, "Destroy window error."),
    (MenuNullMenu, 28, "Null menu."),
    (PAbortJob, 29, "Print aborted"),
    (PBadPrintRecord, 30, "Bad print record."),
    (PDriverError, 31, "Print driver error."),
    (PWindowsError, 32, "Operating system error during print."),
    (PMemoryError, 33, "Memory error during print."),
    (PDialogError, 34, "Print dialog error."),
    (PMiscError, 35, "Generic print error."),
    (DvInvalidRefnum, 36, "Invalid device refnum."),
    (DvDeviceNotFound, 37, "Device not found."),
    (DvParamErr, 38, "Device parameter error."),
    (DvUnitErr, 39, "Device unit error."),
    (DvOpenErr, 40, "Cannot open device."),
    (DvAbortErr, 41, "Device call aborted."),
    (BogusError, 42, "Generic error."),
    (CancelError, 43, "Operation cancelled by user."),
    (OMObjLowErr, 44, "Object ID too low."),
    (OMObjHiErr, 45, "Object ID too high."),
    (OMObjNotInHeapErr, 46, "Object not in heap."),
    (OMOHeapNotKnownErr, 47,"Unknown heap."),
    (OMBadDPIdErr, 48, "Unknown object (invalid DefProc)."),
    (OMNoDPinTabErr, 49, "Unknown object (DefProc not in table)."),
    (OMMsgOutOfRangeErr, 50, "Message out of range."),
    (OMMethodNullErr, 51, "Null method."),
    (OMUnknownMsgErr, 52, "Unknown message."),
    (MgNotSupported, 53, "Manager call not supported."),
    (NcBadAddressErr, 54, "The network address is ill-formed."),
    (NcInProgress, 55, "The network operation is in progress."),
    (NcTimeOutErr, 56, "The network operation exceeded the user-specified or system time limit."),
    (NcBusyErr,	57, "The network connection is busy."),
    (NcNotSupportedErr,	58, "The network function is not supported by the system."),
    (NcNetErr, 59, "The network is down, unreachable, or has been reset."),
    (NcAddrInUseErr, 60, "The specified port or network address is currently in use. Select an available port or network address."),
    (NcSysOutOfMem, 61, "The system could not allocate the necessary memory."),
    (NcSysConnAbortedErr, 62, "The system caused the network connection to be aborted."),
    (NcConnRefusedErr, 63, "The network connection was refused by the server."),
    (NcNotConnectedErr, 64, "The network connection is not yet established."),
    (NcAlreadyConnectedErr, 65, "The network connection is already established."),
    (NcConnClosedErr, 66, "The network connection was closed by the peer."),
    (AmInitErr, 67, "Interapplication Manager initialization error."),
    (OccBadOccurrenceErr, 68, "Bad occurrence."),
    (OccWaitOnUnBoundHdlrErr, 69, "Handler does not know what occurrence to wait for."),
    (OccFunnyQOverFlowErr, 70, "Occurrence queue overflow."),
    (FDataLogTypeConflict, 71, "File datalog type conflict."),
    (EcLVSBCannotBeCalledFromThread, 72, "Semaphore not signaled."),
    (AmUnrecognizedType, 73, "Interapplication Manager unrecognized type error."),
    (MCorruptErr, 74, "Memory or data structure corrupt."),
    (EcLVSBErrorMakingTempDLL, 75, "Failed to make temporary DLL."),
    (EcLVSBOldCIN, 76, "Old CIN version."),
    (FmtTypeMismatch, 81, "Format specifier type mismatch."),
    (FmtUnknownConversion, 82, "Unknown format specifier."),
    (FmtTooFew, 83, "Too few format specifiers."),
    (FmtTooMany, 84, "Too many format specifiers."),
    (FmtScanError, 85, "Scan failed. The input string does not contain data in the expected format."),
    (LvOLEConvertErr, 87, "Error converting to variant."),
    (RtMenuErr, 88, "Run-time menu error."),
    (PwdTampered, 89, "Another user tampered with the VI password."),
    (LvVariantAttrNotFound, 90, "Variant attribute not found."),
    (LvVariantTypeMismatch, 91, "The data type of the variant is not compatible with the data type wired to the type input."),
    (AxEventDataNotAvailable, 92, "The ActiveX event data was not available on the queue."),
    (AxEventStoreNotPresent, 93, "ActiveX event information was not available."),
    (AxOccurrenceNotFound, 94, "The occurrence associated with the ActiveX event was not found."),
    (AxEventQueueNotCreated, 95, "The ActiveX event queue could not be created."),
    (AxEventInfoNotAvailable, 96, "ActiveX event information was not available in the type library."),
    (OleNullRefnumPassed, 97, "A null or previously deleted refnum was passed in as an input."),
    (IviInvalidDowncast, 102, "IVI invalid downcast."),
    (IviInvalidClassSesn, 103, "No IVI class session opened."),
    (NcSockNotMulticast, 108, "Singlecast connections cannot send to multicast addresses."),
    (NcSockNotSinglecast, 109, "Multicast connections cannot send to singlecast addresses."),
    (NcBadMulticastAddr, 110, "Specified IP address is not in multicast address range."),
    (NcMcastSockReadOnly, 111, "Cannot write to read-only multicast connection."),
    (NcMcastSockWriteOnly, 112, "Cannot read from write-only multicast connection."),
    (NcDatagramMsgSzErr, 113, "A message sent on a datagram socket was larger than the internal message buffer or some other network limit, or the buffer used to receive a datagram was smaller than the datagram itself."),
    (DataCorruptErr, 116, "Unflatten or byte stream read operation failed due to corrupt, unexpected, or truncated data."),
    (RequireFullPathErr, 117, "Directory path supplied where a file path is required. A file path with the filename is required, but the supplied path is a path to a directory."),
    (FolderNotExistErr, 118, "The supplied folder path does not exist."),
    (NcBtInvalidModeErr, 119, "Illegal combination of Bluetooth discoverable and non-connectable modes."),
    (NcBtSetModeErr, 120, "Error setting Bluetooth mode."),
    (MgBtInvalidGUIDStrErr, 121, "Invalid GUID string."),
    (RVersInFuture, 122, "The resource you are attempting to open was created in a more recent version of LabVIEW and is incompatible with this version.")
);

/// # Examples
///
/// ```
/// use labview_interop::error::{LVStatusCode, MgErrorCode, MgError, LVInteropError};
/// use std::convert::TryFrom;
///
/// let status = LVStatusCode::from(1);
/// let result: Result<MgErrorCode, LVInteropError> = MgErrorCode::try_from(status);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), MgErrorCode::OutOfMemory);
///
/// let status = LVStatusCode::from(0);
/// let result: Result<MgErrorCode, LVInteropError> = MgErrorCode::try_from(status);
/// assert!(result.is_err());
///
/// let error_code = MgErrorCode::InvalidHandle;
/// let error: MgError = error_code.into();
/// assert_eq!(error, MgError::InvalidHandle);
///
/// let error = MgError::MFullErr;
/// let error_code: MgErrorCode = error.into();
/// assert_eq!(error_code, MgErrorCode::OutOfMemory);
/// ```

/*
// at the cost of using nightly rust,
// this implementation would allow to use the try operator
// directly on the c calls.
//
// ```rust
// extern "C" fn mycfun(blar: &str) -> MgStatus {
//    return 1;
// }
//
// fn test<T>(a: T) -> Result<T, LVInteropError> {
//     mycfun("dudu")?
// }
//

use std::ops;
impl ops::Try for MgStatus {
    type Output: ();
    type Residual: MgError;

    fn from_output(_: Self::Output) -> Self {
        MgStatus(0)
    }
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        if self.0 == 0 {
            ControlFlow::Continue(());
        } else {
            ControlFlow::Break(self.0.into());
        }
    }

}
impl ops::FromResidual<MgError> for MgStatus {
    fn from_residual(residual: MgError) -> Self {
        MgStatus(residual.into())
    }
}

impl From<MgStatus> for Result<(), MgError> {
    fn from(status: MgStatus) -> Self {
        if status.0 == 0 {
            Ok(())
        } else {
            Err(status.0.into())
        }
    }
}
*/

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
    #[error("Invalid numeric status code for conversion into enumerated error code")]
    InvalidMgErrorCode,
    #[error("Internal LabVIEW Manager Error: {0}")]
    LabviewMgError(#[from] MgError),
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
            LVInteropError::InvalidMgErrorCode => MgErr(-1),
            LVInteropError::LabviewMgError(err) => MgErr(-1),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lvstatuscode_from_i32() {
        let status = LVStatusCode::from(0);
        assert_eq!(status, LVStatusCode::SUCCESS);

        let status = LVStatusCode::from(1);
        assert_eq!(status, LVStatusCode(1));
    }

    #[test]
    fn test_mgerrorcode_to_mgerror() {
        define_errors!(
            (OutOfMemory, 1, "Memory is full"),
            (InvalidHandle, 2, "Invalid handle")
        );

        let error_code = MgErrorCode::OutOfMemory;
        let error: MgError = error_code.into();
        assert_eq!(error, MgError::OutOfMemory);

        let error_code = MgErrorCode::InvalidHandle;
        let error: MgError = error_code.into();
        assert_eq!(error, MgError::InvalidHandle);
    }

    #[test]
    fn test_mgerror_to_mgerrorcode() {
        define_errors!(
            (OutOfMemory, 1, "Memory is full"),
            (InvalidHandle, 2, "Invalid handle")
        );

        let error = MgError::OutOfMemory;
        let error_code: MgErrorCode = error.into();
        assert_eq!(error_code, MgErrorCode::OutOfMemory);

        let error = MgError::InvalidHandle;
        let error_code: MgErrorCode = error.into();
        assert_eq!(error_code, MgErrorCode::InvalidHandle);
    }

    #[test]
    fn test_lvstatuscode_to_mgerrorcode() {
        define_errors!(
            (OutOfMemory, 1, "Memory is full"),
            (InvalidHandle, 2, "Invalid handle")
        );

        let status = LVStatusCode::from(1);
        let result: core::result::Result<MgErrorCode, LVInteropError> =
            MgErrorCode::try_from(status);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MgErrorCode::OutOfMemory);

        let status = LVStatusCode::from(0);
        let result: core::result::Result<MgErrorCode, LVInteropError> =
            MgErrorCode::try_from(status);
        assert!(result.is_err());
    }

    #[test]
    fn test_lvstatuscode_to_result() {
        let status = LVStatusCode::from(1);
        let result: core::result::Result<(), LVInteropError> = Result::try_from(status).unwrap();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MgError::MFullErr);

        let status = LVStatusCode::from(0);
        let result: core::result::Result<(), MgError> = Result::try_from(status).unwrap();
        assert!(result.is_ok());
    }
}
