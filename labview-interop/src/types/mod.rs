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

labview_layout!(
    /// Represents the LabVIEW waveform type where:
    ///
    /// * t0: The start time of the data.
    /// * dt: The time delte between samples.
    /// * data: A 1d array of type <T>
    /// * attributes: Variant attributes but these are inaccessible to rust.
    ///
    /// ## Padding
    ///
    /// The padding scheme here is wierd and unexpected and has been reverse engineered
    /// based on real calls. No idea why the padding exists whether it is documented anywhere.
    pub struct Waveform<T> {
        pub t0: timestamp::LVTime,
        pub dt: f64,
        pub data: LVArrayHandle<1, T>,
        #[cfg(target_pointer_width = "64")]
        _pad: u64,
        #[cfg(target_pointer_width = "32")]
        _pad: u32,
        #[cfg(target_pointer_width = "32")]
        _mini_pad: u8,
        pub attributes: LVVariant,
        #[cfg(target_pointer_width = "64")]
        _pad2: u64,
        #[cfg(target_pointer_width = "32")]
        _pad2: u32,
    }
);
