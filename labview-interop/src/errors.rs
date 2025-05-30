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
//!    We want to be able to return the errors generated internally through the function return and be
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
//! This section is a summary of defined and observed LabVIEW behaviour
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
//!
//! For obvious reasons the labview interop crate will use the **range 542,000 to 542,999** for errors that are generated
//! internally and not handed down by c functions
//!
//! # Error Implementation
//! There is a hierarchy of Status and Errors
//! - A status encode Success and Error
//!
//! The errors we expect to receive from calls to labview functions are
//! MgErrorCode
//! MgError
//!
//! Our generic Error handling is an enum
//! LabviewInteropError
//! This enum has custom errors for our internal use, we can hold MgErrors, and as last resort we can also hold
//! LVError

use crate::types::LVStatusCode;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

/// the conversion from LVInteropError back to LVStatusCode is important
/// to return the status in extern "C" functions back to LV
impl<T> From<Result<T>> for LVStatusCode {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(_) => LVStatusCode::SUCCESS,
            Err(err) => (&err).into(),
        }
    }
}
impl From<&LVInteropError> for LVStatusCode {
    fn from(value: &LVInteropError) -> Self {
        match value {
            LVInteropError::LabviewMgError(e) => e.into(),
            LVInteropError::InternalError(e) => e.into(),
            LVInteropError::LabviewError(e) => *e,
        }
    }
}

impl From<LVInteropError> for LVStatusCode {
    fn from(value: LVInteropError) -> Self {
        (&value).into()
    }
}

impl From<LVStatusCode> for LVInteropError {
    fn from(status: LVStatusCode) -> Self {
        LVInteropError::LabviewError(status)
    }
}

/// MgError is the subset of LabVIEW errors that may occur when dealing with the memory manager
/// So in the context of Rust-LabVIEW-interop these are the kind of labview errors we may trigger within the library
///
///
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
///
/// in https://www.ni.com/docs/en-US/bundle/labview/page/labview-manager-function-errors.html
/// `MgError` implements Error on top of the `MgErrorCode` and includes a description
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum MgError {
    #[error("An input parameter is invalid.")]
    MgArgErr = 1,
    #[error("Memory is full.")]
    MFullErr = 2,
    #[error("End of file encountered.")]
    FEof = 4,
    #[error("File already open")]
    FIsOpen = 5,
    #[error("Generic file I/O error.")]
    FIoErr = 6,
    #[error("File not found")]
    FNotFound = 7,
    #[error("File permission error.")]
    FNoPerm = 8,
    #[error("Disk full")]
    FDiskFull = 9,
    #[error("Duplicate path")]
    FDupPath = 10,
    #[error("Too many files open.")]
    FtMFOpen = 11,
    #[error("Some system capacity necessary for operation is not enabled.")]
    FNotEnabled = 12,
    #[error(
                "Failed to load dynamic library because of missing external symbols or dependencies, or because of an invalid file format."
            )]
    RfNotFound = 13,
    #[error("Cannot add resource.")]
    RAddFailed = 14,
    #[error("Resource not found.")]
    RNotFound = 15,
    #[error("Image not found.")]
    INotFound = 16,
    #[error("Not enough memory to manipulate image.")]
    IMemoryErr = 17,
    #[error("DPen does not exist.")]
    DPenNotExist = 18,
    #[error("Configuration type invalid.")]
    CfgBadType = 19,
    #[error("Configuration token not found.")]
    CfgTokenNotFound = 20,
    #[error("Error occurred parsing configuration string.")]
    CfgParseError = 21,
    #[error("Configuration memory error.")]
    CfgAllocError = 22,
    #[error("Bad external code format.")]
    EcLVSBFormatError = 23,
    #[error("External subroutine not supported.")]
    EcLVSBSubrError = 24,
    #[error("External code not present.")]
    EcLVSBNoCodeError = 25,
    #[error("Null window.")]
    WNullWindow = 26,
    #[error("Destroy window error.")]
    WDestroyMixup = 27,
    #[error("Null menu.")]
    MenuNullMenu = 28,
    #[error("Print aborted")]
    PAbortJob = 29,
    #[error("Bad print record.")]
    PBadPrintRecord = 30,
    #[error("Print driver error.")]
    PDriverError = 31,
    #[error("Operating system error during print.")]
    PWindowsError = 32,
    #[error("Memory error during print.")]
    PMemoryError = 33,
    #[error("Print dialog error.")]
    PDialogError = 34,
    #[error("Generic print error.")]
    PMiscError = 35,
    #[error("Invalid device refnum.")]
    DvInvalidRefnum = 36,
    #[error("Device not found.")]
    DvDeviceNotFound = 37,
    #[error("Device parameter error.")]
    DvParamErr = 38,
    #[error("Device unit error.")]
    DvUnitErr = 39,
    #[error("Cannot open device.")]
    DvOpenErr = 40,
    #[error("Device call aborted.")]
    DvAbortErr = 41,
    #[error("Generic error.")]
    BogusError = 42,
    #[error("Operation cancelled by user.")]
    CancelError = 43,
    #[error("Object ID too low.")]
    OMObjLowErr = 44,
    #[error("Object ID too high.")]
    OMObjHiErr = 45,
    #[error("Object not in heap.")]
    OMObjNotInHeapErr = 46,
    #[error("Unknown heap.")]
    OMOHeapNotKnownErr = 47,
    #[error("Unknown object (invalid DefProc).")]
    OMBadDPIdErr = 48,
    #[error("Unknown object (DefProc not in table).")]
    OMNoDPinTabErr = 49,
    #[error("Message out of range.")]
    OMMsgOutOfRangeErr = 50,
    #[error("Null method.")]
    OMMethodNullErr = 51,
    #[error("Unknown message.")]
    OMUnknownMsgErr = 52,
    #[error("Manager call not supported.")]
    MgNotSupported = 53,
    #[error("The network address is ill-formed.")]
    NcBadAddressErr = 54,
    #[error("The network operation is in progress.")]
    NcInProgress = 55,
    #[error("The network operation exceeded the user-specified or system time limit.")]
    NcTimeOutErr = 56,
    #[error("The network connection is busy.")]
    NcBusyErr = 57,
    #[error("The network function is not supported by the system.")]
    NcNotSupportedErr = 58,
    #[error("The network is down, unreachable, or has been reset.")]
    NcNetErr = 59,
    #[error(
                "The specified port or network address is currently in use. Select an available port or network address."
            )]
    NcAddrInUseErr = 60,
    #[error("The system could not allocate the necessary memory.")]
    NcSysOutOfMem = 61,
    #[error("The system caused the network connection to be aborted.")]
    NcSysConnAbortedErr = 62,
    #[error("The network connection was refused by the server.")]
    NcConnRefusedErr = 63,
    #[error("The network connection is not yet established.")]
    NcNotConnectedErr = 64,
    #[error("The network connection is already established.")]
    NcAlreadyConnectedErr = 65,
    #[error("The network connection was closed by the peer.")]
    NcConnClosedErr = 66,
    #[error("Interapplication Manager initialization error.")]
    AmInitErr = 67,
    #[error("Bad occurrence.")]
    OccBadOccurrenceErr = 68,
    #[error("Handler does not know what occurrence to wait for.")]
    OccWaitOnUnBoundHdlrErr = 69,
    #[error("Occurrence queue overflow.")]
    OccFunnyQOverFlowErr = 70,
    #[error("File datalog type conflict.")]
    FDataLogTypeConflict = 71,
    #[error("Semaphore not signaled.")]
    EcLVSBCannotBeCalledFromThread = 72,
    #[error("Interapplication Manager unrecognized type error.")]
    AmUnrecognizedType = 73,
    #[error("Memory or data structure corrupt.")]
    MCorruptErr = 74,
    #[error("Failed to make temporary DLL.")]
    EcLVSBErrorMakingTempDLL = 75,
    #[error("Old CIN version.")]
    EcLVSBOldCIN = 76,
    #[error("Format specifier type mismatch.")]
    FmtTypeMismatch = 81,
    #[error("Unknown format specifier.")]
    FmtUnknownConversion = 82,
    #[error("Too few format specifiers.")]
    FmtTooFew = 83,
    #[error("Too many format specifiers.")]
    FmtTooMany = 84,
    #[error("Scan failed. The input string does not contain data in the expected format.")]
    FmtScanError = 85,
    #[error("Error converting to variant.")]
    LvOLEConvertErr = 87,
    #[error("Run-time menu error.")]
    RtMenuErr = 88,
    #[error("Another user tampered with the VI password.")]
    PwdTampered = 89,
    #[error("Variant attribute not found.")]
    LvVariantAttrNotFound = 90,
    #[error(
                "The data type of the variant is not compatible with the data type wired to the type input."
            )]
    LvVariantTypeMismatch = 91,
    #[error("The ActiveX event data was not available on the queue.")]
    AxEventDataNotAvailable = 92,
    #[error("ActiveX event information was not available.")]
    AxEventStoreNotPresent = 93,
    #[error("The occurrence associated with the ActiveX event was not found.")]
    AxOccurrenceNotFound = 94,
    #[error("The ActiveX event queue could not be created.")]
    AxEventQueueNotCreated = 95,
    #[error("ActiveX event information was not available in the type library.")]
    AxEventInfoNotAvailable = 96,
    #[error("A null or previously deleted refnum was passed in as an input.")]
    OleNullRefnumPassed = 97,
    #[error("IVI invalid downcast.")]
    IviInvalidDowncast = 102,
    #[error("No IVI class session opened.")]
    IviInvalidClassSesn = 103,
    #[error("Singlecast connections cannot send to multicast addresses.")]
    NcSockNotMulticast = 108,
    #[error("Multicast connections cannot send to singlecast addresses.")]
    NcSockNotSinglecast = 109,
    #[error("Specified IP address is not in multicast address range.")]
    NcBadMulticastAddr = 110,
    #[error("Cannot write to read-only multicast connection.")]
    NcMcastSockReadOnly = 111,
    #[error("Cannot read from write-only multicast connection.")]
    NcMcastSockWriteOnly = 112,
    #[error(
                "A message sent on a datagram socket was larger than the internal message buffer or some other network limit, or the buffer used to receive a datagram was smaller than the datagram itself."
            )]
    NcDatagramMsgSzErr = 113,
    #[error(
                "Unflatten or byte stream read operation failed due to corrupt, unexpected, or truncated data."
            )]
    DataCorruptErr = 116,
    #[error(
                "Directory path supplied where a file path is required. A file path with the filename is required, but the supplied path is a path to a directory."
            )]
    RequireFullPathErr = 117,
    #[error("The supplied folder path does not exist.")]
    FolderNotExistErr = 118,
    #[error("Illegal combination of Bluetooth discoverable and non-connectable modes.")]
    NcBtInvalidModeErr = 119,
    #[error("Error setting Bluetooth mode.")]
    NcBtSetModeErr = 120,
    #[error("Invalid GUID string.")]
    MgBtInvalidGUIDStrErr = 121,
    #[error(
                "The resource you are attempting to open was created in a more recent version of LabVIEW and is incompatible with this version."
            )]
    RVersInFuture = 122,
}

impl TryFrom<LVStatusCode> for MgError {
    type Error = LVInteropError;
    fn try_from(status: LVStatusCode) -> ::core::result::Result<Self, Self::Error> {
        // SUCCESS is not a valid error!
        if status == LVStatusCode::SUCCESS {
            return Err(InternalError::InvalidMgErrorCode.into());
        }
        match MgError::try_from_primitive(status.into()) {
            Ok(code) => Ok(code),
            Err(_) => Err(InternalError::InvalidMgErrorCode.into()),
        }
    }
}

impl From<&MgError> for LVStatusCode {
    fn from(errcode: &MgError) -> LVStatusCode {
        let erri32: i32 = *errcode as i32;
        erri32.into()
    }
}

impl From<MgError> for LVStatusCode {
    fn from(errcode: MgError) -> LVStatusCode {
        (&errcode).into()
    }
}

/// # Examples
///
/// ```
/// use labview_interop::errors::{MgError, LVInteropError};
/// use labview_interop::types::LVStatusCode;
/// use std::convert::TryFrom;
///
/// let status = LVStatusCode::from(2);
/// let result: Result<MgError, LVInteropError> = MgError::try_from(status);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), MgError::MFullErr);
///
/// let status = LVStatusCode::from(0);
/// let result: Result<MgError, LVInteropError> = MgError::try_from(status);
/// assert!(result.is_err());
///
/// let error = MgError::BogusError;
/// let error_code: i32 = error.into();
/// assert_eq!(error_code, 42);
/// ```
///
/// LVInteropError is our internal Error type
/// in order to be able to easily convert it to LV ErrorClusters all Errors should possess an
/// Error Code
///
/// Our choice of a custom ranges in Labview is (see comment above on valid ranges)
/// 542,000 to 542,999
#[derive(Error, Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum InternalError {
    #[error("LabVIEW Interop General Error. Probably because of a missing implementation.")]
    Misc = 542_000,
    #[error("LabVIEW API unavailable. Probably because it isn't being run in LabVIEW. Source Error: {0}")]
    NoLabviewApi(String) = 542_001,
    #[error("Invalid handle when valid handle is required")]
    InvalidHandle = 542_002,
    #[error("LabVIEW arrays can only have dimensions of i32 range.")]
    ArrayDimensionsOutOfRange = 542_003,
    #[error(
        "Array dimensions don't match. You may require the link feature to enable auto-resizing."
    )]
    ArrayDimensionMismatch = 542_004,
    #[error("Creating of handle in LabVIEW memory manager failed. Perhaps you are out of memory?")]
    HandleCreationFailed = 542_005,
    #[error("Invalid numeric status code for conversion into enumerated error code")]
    InvalidMgErrorCode = 542_006,
}

impl From<&InternalError> for LVStatusCode {
    fn from(err: &InternalError) -> LVStatusCode {
        let err_i32: i32 = match err {
            InternalError::Misc => 542_000,
            InternalError::NoLabviewApi(_) => 542_001,
            InternalError::InvalidHandle => 542_002,
            InternalError::ArrayDimensionsOutOfRange => 542_003,
            InternalError::ArrayDimensionMismatch => 542_004,
            InternalError::HandleCreationFailed => 542_005,
            InternalError::InvalidMgErrorCode => 542_006,
        };
        err_i32.into()
    }
}
#[derive(Error, Debug, Clone, PartialEq)]
pub enum LVInteropError {
    #[error("Internal LabVIEW Manager Error: {0}")]
    LabviewMgError(#[from] MgError),
    #[error("Internal Error: {0}")]
    InternalError(#[from] InternalError),
    #[error("LabVIEW Error: {0}")]
    LabviewError(LVStatusCode),
}

pub type Result<T> = std::result::Result<T, LVInteropError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_lvstatuscode_from_mgerror() {
        let err = MgError::BogusError;
        let status: LVStatusCode = err.into();

        assert_eq!(LVStatusCode::from(42), status)
    }

    #[test]
    fn test_error_lvinteroperror_from_lvstatuscode() {
        let status = LVStatusCode::from(42);
        let mg_err = MgError::try_from(status).unwrap();

        let expected_code: LVStatusCode = 42.into();
        assert_eq!(expected_code, mg_err.into());
    }

    #[test]
    fn test_error_lvstatuscode_from_lvinteroperror() {
        let err: LVInteropError = MgError::BogusError.into();
        let status: LVStatusCode = LVStatusCode::from(42);
        assert_eq!(status, err.into());

        let err: LVInteropError =
            InternalError::NoLabviewApi("Test Inner message".to_string()).into();
        let status: LVStatusCode = LVStatusCode::from(542_001);

        println!("{}", status);
        assert_eq!(status, err.into());

        //let err = LVStatusCode::from(42);
        //assert

        //let num: i32 = err.code().into();
        //assert_eq!(num, 42);
        //println!("{}", err);
    }
}
