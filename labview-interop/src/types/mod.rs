//! The types module provides some of the common structures
//! and functions for handling types from LabVIEW.

pub mod array;
pub mod timestamp;

use std::ffi::c_void;

use crate::memory::UHandle;

//surface some of the common types.
pub use array::{LVArray, LVArrayHandle};
pub use timestamp::LVTime;

/// Wrap a struct declaration to have the packing attributes
/// set for exchanging the data with the LabVIEW cluster type.
///
/// todo: test and validation.
#[macro_export]
macro_rules! labview_layout {
    ($struct:item) => {
        #[repr(C)]
        #[cfg_attr(target_pointer_width = "32", repr(packed))]
        $struct
    };
}

/// Represents a LabVIEW Variant. The internal structure is undefined
/// by NI and therefore unavailable.
///
/// This is available as a placeholder in clusters etc.
#[repr(transparent)]
pub struct LVVariant(UHandle<c_void>);

//todo: why the padding? not needed in normal cluster.
labview_layout!(
    pub struct Waveform<T> {
        pub t0: timestamp::LVTime,
        pub dt: f64,
        pub data: LVArrayHandle<1, T>,
        #[cfg(target_pointer_width = "64")]
        _pad: u64,
        #[cfg(target_pointer_width = "32")]
        _pad: u64,
        pub attributes: LVVariant,
        #[cfg(target_pointer_width = "64")]
        _pad2: u64,
        #[cfg(target_pointer_width = "32")]
        _pad2: u64,
    }
);
