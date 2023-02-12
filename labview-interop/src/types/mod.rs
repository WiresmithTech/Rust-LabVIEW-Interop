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

labview_layout!(
    struct Test {
        one: u8,
        two: u16,
    }
);

/// Represents a LabVIEW Variant. The internal structure is undefined
/// by NI and therefore unavailable.
///
/// This is available as a placeholder in clusters etc.
#[repr(transparent)]
pub struct LVVariant(UHandle<c_void>);

labview_layout!(
    pub struct Waveform<T> {
        t0: timestamp::LVTime,
        dt: f64,
        data: LVArray<1, T>,
        attributes: LVVariant,
    }
);
