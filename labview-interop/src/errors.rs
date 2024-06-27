use std::{error::Error, fmt::Display};
use thiserror::Error;

#[derive(Debug)]
#[repr(i32)]
pub enum MgErrCode {
    Ok = 0,
    MgArgErr = 1,
    MFullErr = 2,
    FEof = 4,
    FIsOpen = 5,
    FIoErr = 6,
    FNotFound = 7,
    FNoPerm = 8,
    FDiskFull = 9,
    FDupPath = 10,
    FtMFOpen = 11,
    FNotEnabled = 12,
    RfNotFound = 13,
    RAddFailed = 14,
    RNotFound = 15,
    INotFound = 16,
    IMemoryErr = 17,
    DPenNotExist = 18,
    CfgBadType = 19,
    CfgTokenNotFound = 20,
    CfgParseError = 21,
    CfgAllocError = 22,
    EcLVSBFormatError = 23,
    EcLVSBSubrError = 24,
    EcLVSBNoCodeError = 25,
    WNullWindow = 26,
    WDestroyMixup = 27,
    MenuNullMenu = 28,
    PAbortJob = 29,
    PBadPrintRecord = 30,
    PDriverError = 31,
    PWindowsError = 32,
    PMemoryError = 33,
    PDialogError = 34,
    PMiscError = 35,
    DvInvalidRefnum = 36,
    DvDeviceNotFound = 37,
    DvParamErr = 38,
    DvUnitErr = 39,
    DvOpenErr = 40,
    DvAbortErr = 41,
    BogusError = 42,
    CancelError = 43,
    OMObjLowErr = 44,
    OMObjHiErr = 45,
    OMObjNotInHeapErr = 46,
    OMOHeapNotKnownErr = 47,
    OMBadDPIdErr = 48,
    OMNoDPinTabErr = 49,
    OMMsgOutOfRangeErr = 50,
    OMMethodNullErr = 51,
    OMUnknownMsgErr = 52,
    MgNotSupported = 53,
    NcBadAddressErr = 54,
    NcInProgress = 55,
    NcTimeOutErr = 56,
    NcBusyErr = 57,
    NcNotSupportedErr = 58,
    NcNetErr = 59,
    NcAddrInUseErr = 60,
    NcSysOutOfMem = 61,
    NcSysConnAbortedErr = 62,
    NcConnRefusedErr = 63,
    NcNotConnectedErr = 64,
    NcAlreadyConnectedErr = 65,
    NcConnClosedErr = 66,
    AmInitErr = 67,
    OccBadOccurrenceErr = 68,
    OccWaitOnUnBoundHdlrErr = 69,
    OccFunnyQOverFlowErr = 70,
    FDataLogTypeConflict = 71,
    EcLVSBCannotBeCalledFromThread = 72,
    AmUnrecognizedType = 73,
    MCorruptErr = 74,
    EcLVSBErrorMakingTempDLL = 75,
    EcLVSBOldCIN = 76,
    FmtTypeMismatch = 81,
    FmtUnknownConversion = 82,
    FmtTooFew = 83,
    FmtTooMany = 84,
    FmtScanError = 85,
    LvOLEConvertErr = 87,
    RtMenuErr = 88,
    PwdTampered = 89,
    LvVariantAttrNotFound = 90,
    LvVariantTypeMismatch = 91,
    AxEventDataNotAvailable = 92,
    AxEventStoreNotPresent = 93,
    AxOccurrenceNotFound = 94,
    AxEventQueueNotCreated = 95,
    AxEventInfoNotAvailable = 96,
    OleNullRefnumPassed = 97,
    IviInvalidDowncast = 102,
    IviInvalidClassSesn = 103,
    NcSockNotMulticast = 108,
    NcSockNotSinglecast = 109,
    NcBadMulticastAddr = 110,
    NcMcastSockReadOnly = 111,
    NcMcastSockWriteOnly = 112,
    NcDatagramMsgSzErr = 113,
    DataCorruptErr = 116,
    RequireFullPathErr = 117,
    FolderNotExistErr = 118,
    NcBtInvalidModeErr = 119,
    NcBtSetModeErr = 120,
    MgBtInvalidGUIDStrErr = 121,
    RVersInFuture = 122,
}

// halucination based of the error code names by chatgpt4o
// not sure if there are official descriptions anywhere available
#[derive(Debug, Error)]
pub enum MgError {
    #[error("Argument Error")]
    MgArgErr,

    #[error("Memory Full Error")]
    MFullErr,

    #[error("File End of File")]
    FEof,

    #[error("File is Open")]
    FIsOpen,

    #[error("File I/O Error")]
    FIoErr,

    #[error("File Not Found")]
    FNotFound,

    #[error("File No Permission")]
    FNoPerm,

    #[error("File Disk Full")]
    FDiskFull,

    #[error("File Duplicate Path")]
    FDupPath,

    #[error("File Table Memory Full")]
    FtMFOpen,

    #[error("File Not Enabled")]
    FNotEnabled,

    #[error("Reference Not Found")]
    RfNotFound,

    #[error("Resource Addition Failed")]
    RAddFailed,

    #[error("Resource Not Found")]
    RNotFound,

    #[error("Item Not Found")]
    INotFound,

    #[error("I Memory Error")]
    IMemoryErr,

    #[error("Dependency Not Exist")]
    DPenNotExist,

    #[error("Configuration Bad Type")]
    CfgBadType,

    #[error("Configuration Token Not Found")]
    CfgTokenNotFound,

    #[error("Configuration Parse Error")]
    CfgParseError,

    #[error("Configuration Allocation Error")]
    CfgAllocError,

    #[error("LVSB Format Error")]
    EcLVSBFormatError,

    #[error("LVSB Subroutine Error")]
    EcLVSBSubrError,

    #[error("LVSB No Code Error")]
    EcLVSBNoCodeError,

    #[error("Null Window")]
    WNullWindow,

    #[error("Destroy Mixup")]
    WDestroyMixup,

    #[error("Null Menu")]
    MenuNullMenu,

    #[error("Abort Job")]
    PAbortJob,

    #[error("Bad Print Record")]
    PBadPrintRecord,

    #[error("Driver Error")]
    PDriverError,

    #[error("Windows Error")]
    PWindowsError,

    #[error("Memory Error")]
    PMemoryError,

    #[error("Dialog Error")]
    PDialogError,

    #[error("Miscellaneous Error")]
    PMiscError,

    #[error("Invalid Refnum")]
    DvInvalidRefnum,

    #[error("Device Not Found")]
    DvDeviceNotFound,

    #[error("Parameter Error")]
    DvParamErr,

    #[error("Unit Error")]
    DvUnitErr,

    #[error("Open Error")]
    DvOpenErr,

    #[error("Abort Error")]
    DvAbortErr,

    #[error("Bogus Error")]
    BogusError,

    #[error("Cancel Error")]
    CancelError,

    #[error("Object Manager Low Error")]
    OMObjLowErr,

    #[error("Object Manager High Error")]
    OMObjHiErr,

    #[error("Object Manager Not In Heap Error")]
    OMObjNotInHeapErr,

    #[error("Object Manager Heap Not Known Error")]
    OMOHeapNotKnownErr,

    #[error("Object Manager Bad DPId Error")]
    OMBadDPIdErr,

    #[error("Object Manager No DP In Table Error")]
    OMNoDPinTabErr,

    #[error("Object Manager Message Out Of Range Error")]
    OMMsgOutOfRangeErr,

    #[error("Object Manager Method Null Error")]
    OMMethodNullErr,

    #[error("Object Manager Unknown Message Error")]
    OMUnknownMsgErr,

    #[error("Not Supported")]
    MgNotSupported,

    #[error("Network Controller Bad Address Error")]
    NcBadAddressErr,

    #[error("In Progress")]
    NcInProgress,

    #[error("Timeout Error")]
    NcTimeOutErr,

    #[error("Busy Error")]
    NcBusyErr,

    #[error("Not Supported Error")]
    NcNotSupportedErr,

    #[error("Network Error")]
    NcNetErr,

    #[error("Address In Use Error")]
    NcAddrInUseErr,

    #[error("System Out Of Memory")]
    NcSysOutOfMem,

    #[error("System Connection Aborted Error")]
    NcSysConnAbortedErr,

    #[error("Connection Refused Error")]
    NcConnRefusedErr,

    #[error("Not Connected Error")]
    NcNotConnectedErr,

    #[error("Already Connected Error")]
    NcAlreadyConnectedErr,

    #[error("Connection Closed Error")]
    NcConnClosedErr,

    #[error("Initialization Error")]
    AmInitErr,

    #[error("Bad Occurrence Error")]
    OccBadOccurrenceErr,

    #[error("Wait On Unbound Handler Error")]
    OccWaitOnUnBoundHdlrErr,

    #[error("Funny Queue Overflow Error")]
    OccFunnyQOverFlowErr,

    #[error("Data Log Type Conflict")]
    FDataLogTypeConflict,

    #[error("LVSB Cannot Be Called From Thread")]
    EcLVSBCannotBeCalledFromThread,

    #[error("Unrecognized Type")]
    AmUnrecognizedType,

    #[error("Corrupt Error")]
    MCorruptErr,

    #[error("LVSB Error Making Temp DLL")]
    EcLVSBErrorMakingTempDLL,

    #[error("LVSB Old CIN")]
    EcLVSBOldCIN,

    #[error("Format Type Mismatch")]
    FmtTypeMismatch,

    #[error("Unknown Conversion")]
    FmtUnknownConversion,

    #[error("Too Few")]
    FmtTooFew,

    #[error("Too Many")]
    FmtTooMany,

    #[error("Scan Error")]
    FmtScanError,

    #[error("OLE Convert Error")]
    LvOLEConvertErr,

    #[error("Runtime Menu Error")]
    RtMenuErr,

    #[error("Password Tampered")]
    PwdTampered,

    #[error("Variant Attribute Not Found")]
    LvVariantAttrNotFound,

    #[error("Variant Type Mismatch")]
    LvVariantTypeMismatch,

    #[error("Event Data Not Available")]
    AxEventDataNotAvailable,

    #[error("Event Store Not Present")]
    AxEventStoreNotPresent,

    #[error("Occurrence Not Found")]
    AxOccurrenceNotFound,

    #[error("Event Queue Not Created")]
    AxEventQueueNotCreated,

    #[error("Event Info Not Available")]
    AxEventInfoNotAvailable,

    #[error("Null Refnum Passed")]
    OleNullRefnumPassed,

    #[error("Invalid Downcast")]
    IviInvalidDowncast,

    #[error("Invalid Class Session")]
    IviInvalidClassSesn,

    #[error("Socket Not Multicast")]
    NcSockNotMulticast,

    #[error("Socket Not Singlecast")]
    NcSockNotSinglecast,

    #[error("Bad Multicast Address")]
    NcBadMulticastAddr,

    #[error("Multicast Socket Read Only")]
    NcMcastSockReadOnly,

    #[error("Multicast Socket Write Only")]
    NcMcastSockWriteOnly,

    #[error("Datagram Message Size Error")]
    NcDatagramMsgSzErr,

    #[error("Data Corrupt Error")]
    DataCorruptErr,

    #[error("Require Full Path Error")]
    RequireFullPathErr,

    #[error("Folder Does Not Exist Error")]
    FolderNotExistErr,

    #[error("Bluetooth Invalid Mode Error")]
    NcBtInvalidModeErr,

    #[error("Bluetooth Set Mode Error")]
    NcBtSetModeErr,

    #[error("Bluetooth Invalid GUID String Error")]
    MgBtInvalidGUIDStrErr,

    #[error("Version In Future")]
    RVersInFuture,
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
