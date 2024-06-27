use std::{error::Error, fmt::Display};
use thiserror::Error;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
// Macro to define the enum and implement the conversions
macro_rules! define_errors {
    ($(($name:ident, $code:expr, $msg:expr)),*) => {
        #[derive(Debug, IntoPrimitive, TryFromPrimitive)]
        #[repr(i32)]
        pub enum MgErrorCode {
            $(
                $name = $code,
            )*
        }

        #[derive(Debug, Error)]
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

// Define the errors using the macro
define_errors!(
    (MgArgErr, 1, "Argument Error"),
    (MFullErr, 2, "Memory Full Error"),
    (FEof, 4, "End of File"),
    (FIsOpen, 5, "File is Open"),
    (FIoErr, 6, "I/O Error"),
    (FNotFound, 7, "File Not Found"),
    (FNoPerm, 8, "No Permission"),
    (FDiskFull, 9, "Disk Full"),
    (FDupPath, 10, "Duplicate Path"),
    (FtMFOpen, 11, "File Table Memory Full"),
    (FNotEnabled, 12, "File Not Enabled"),
    (RfNotFound, 13, "Reference Not Found"),
    (RAddFailed, 14, "Addition Failed"),
    (RNotFound, 15, "Resource Not Found"),
    (INotFound, 16, "Item Not Found"),
    (IMemoryErr, 17, "Memory Error"),
    (DPenNotExist, 18, "Dependency Not Exist"),
    (CfgBadType, 19, "Configuration Bad Type"),
    (CfgTokenNotFound, 20, "Configuration Token Not Found"),
    (CfgParseError, 21, "Configuration Parse Error"),
    (CfgAllocError, 22, "Configuration Allocation Error"),
    (EcLVSBFormatError, 23, "LVSB Format Error"),
    (EcLVSBSubrError, 24, "LVSB Subroutine Error"),
    (EcLVSBNoCodeError, 25, "LVSB No Code Error"),
    (WNullWindow, 26, "Null Window"),
    (WDestroyMixup, 27, "Destroy Mixup"),
    (MenuNullMenu, 28, "Null Menu"),
    (PAbortJob, 29, "Abort Job"),
    (PBadPrintRecord, 30, "Bad Print Record"),
    (PDriverError, 31, "Driver Error"),
    (PWindowsError, 32, "Windows Error"),
    (PMemoryError, 33, "Memory Error"),
    (PDialogError, 34, "Dialog Error"),
    (PMiscError, 35, "Miscellaneous Error"),
    (DvInvalidRefnum, 36, "Invalid Refnum"),
    (DvDeviceNotFound, 37, "Device Not Found"),
    (DvParamErr, 38, "Parameter Error"),
    (DvUnitErr, 39, "Unit Error"),
    (DvOpenErr, 40, "Open Error"),
    (DvAbortErr, 41, "Abort Error"),
    (BogusError, 42, "Bogus Error"),
    (CancelError, 43, "Cancel Error"),
    (OMObjLowErr, 44, "Object Manager Low Error"),
    (OMObjHiErr, 45, "Object Manager High Error"),
    (OMObjNotInHeapErr, 46, "Object Manager Not In Heap Error"),
    (
        OMOHeapNotKnownErr,
        47,
        "Object Manager Heap Not Known Error"
    ),
    (OMBadDPIdErr, 48, "Object Manager Bad DPId Error"),
    (OMNoDPinTabErr, 49, "Object Manager No DP In Table Error"),
    (
        OMMsgOutOfRangeErr,
        50,
        "Object Manager Message Out Of Range Error"
    ),
    (OMMethodNullErr, 51, "Object Manager Method Null Error"),
    (OMUnknownMsgErr, 52, "Object Manager Unknown Message Error"),
    (MgNotSupported, 53, "Not Supported"),
    (NcBadAddressErr, 54, "Network Controller Bad Address Error"),
    (NcInProgress, 55, "In Progress"),
    (NcTimeOutErr, 56, "Timeout Error"),
    (NcBusyErr, 57, "Busy Error"),
    (NcNotSupportedErr, 58, "Not Supported Error"),
    (NcNetErr, 59, "Network Error"),
    (NcAddrInUseErr, 60, "Address In Use Error"),
    (NcSysOutOfMem, 61, "System Out Of Memory"),
    (NcSysConnAbortedErr, 62, "System Connection Aborted Error"),
    (NcConnRefusedErr, 63, "Connection Refused Error"),
    (NcNotConnectedErr, 64, "Not Connected Error"),
    (NcAlreadyConnectedErr, 65, "Already Connected Error"),
    (NcConnClosedErr, 66, "Connection Closed Error"),
    (AmInitErr, 67, "Initialization Error"),
    (OccBadOccurrenceErr, 68, "Bad Occurrence Error"),
    (OccWaitOnUnBoundHdlrErr, 69, "Wait On Unbound Handler Error"),
    (OccFunnyQOverFlowErr, 70, "Funny Queue Overflow Error"),
    (FDataLogTypeConflict, 71, "Data Log Type Conflict"),
    (
        EcLVSBCannotBeCalledFromThread,
        72,
        "LVSB Cannot Be Called From Thread"
    ),
    (AmUnrecognizedType, 73, "Unrecognized Type"),
    (MCorruptErr, 74, "Corrupt Error"),
    (EcLVSBErrorMakingTempDLL, 75, "LVSB Error Making Temp DLL"),
    (EcLVSBOldCIN, 76, "LVSB Old CIN"),
    (FmtTypeMismatch, 81, "Format Type Mismatch"),
    (FmtUnknownConversion, 82, "Unknown Conversion"),
    (FmtTooFew, 83, "Too Few"),
    (FmtTooMany, 84, "Too Many"),
    (FmtScanError, 85, "Scan Error"),
    (LvOLEConvertErr, 87, "OLE Convert Error"),
    (RtMenuErr, 88, "Runtime Menu Error"),
    (PwdTampered, 89, "Password Tampered"),
    (LvVariantAttrNotFound, 90, "Variant Attribute Not Found"),
    (LvVariantTypeMismatch, 91, "Variant Type Mismatch"),
    (AxEventDataNotAvailable, 92, "Event Data Not Available"),
    (AxEventStoreNotPresent, 93, "Event Store Not Present"),
    (AxOccurrenceNotFound, 94, "Occurrence Not Found"),
    (AxEventQueueNotCreated, 95, "Event Queue Not Created"),
    (AxEventInfoNotAvailable, 96, "Event Info Not Available"),
    (OleNullRefnumPassed, 97, "Null Refnum Passed"),
    (IviInvalidDowncast, 102, "Invalid Downcast"),
    (IviInvalidClassSesn, 103, "Invalid Class Session"),
    (NcSockNotMulticast, 108, "Socket Not Multicast"),
    (NcSockNotSinglecast, 109, "Socket Not Singlecast"),
    (NcBadMulticastAddr, 110, "Bad Multicast Address"),
    (NcMcastSockReadOnly, 111, "Multicast Socket Read Only"),
    (NcMcastSockWriteOnly, 112, "Multicast Socket Write Only"),
    (NcDatagramMsgSzErr, 113, "Datagram Message Size Error"),
    (DataCorruptErr, 116, "Data Corrupt Error"),
    (RequireFullPathErr, 117, "Require Full Path Error"),
    (FolderNotExistErr, 118, "Folder Does Not Exist Error"),
    (NcBtInvalidModeErr, 119, "Bluetooth Invalid Mode Error"),
    (NcBtSetModeErr, 120, "Bluetooth Set Mode Error"),
    (
        MgBtInvalidGUIDStrErr,
        121,
        "Bluetooth Invalid GUID String Error"
    ),
    (RVersInFuture, 122, "Version In Future")
);

// Helper to convert ErrorCode to i32
impl From<MgErrorCode> for i32 {
    fn from(code: MgErrorCode) -> Self {
        code as i32
    }
}

// Helper to convert i32 to ErrorCode
impl TryFrom<i32> for MgErrorCode {
    type Error = MgError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        MgErrorCode::try_from(value).map_err(|_| MgError::MgArgErr)
    }
}

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
