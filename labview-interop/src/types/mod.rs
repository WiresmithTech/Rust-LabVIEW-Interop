//! The types module provides some of the common structures
//! and functions for handling types from LabVIEW.

pub mod array;
mod boolean;
#[cfg(target_pointer_width = "64")]
mod lv_errors;
pub mod string;
pub mod timestamp;

use std::ffi::c_void;

use crate::memory::UHandle;

//surface some of the common types.
pub use array::{LVArray, LVArrayHandle};
pub use boolean::LVBool;
#[cfg(target_pointer_width = "64")]
pub use lv_errors::{ErrorClusterPtr, ToLvError};
pub use string::LStrHandle;
#[cfg(feature = "link")]
pub use string::LStrOwned;
pub use timestamp::LVTime;

/// Wrap a struct declaration to have the packing attributes
/// set for exchanging the data with the LabVIEW cluster type.
///
/// # 64 Bit
///
/// For 64 bit this is just the standard C packing and is fully
/// functional as a Rust struct.
///
/// # 32 Bit
///
/// On 32 bit this uses a packed representation.
///
/// Because Rust references must be aligned this means you cannot get
/// a reference to an individual item and instead must use
/// [std::ptr::read_unaligned] or [std::ptr::write_unaligned] after getting
/// the address with [std::ptr::addr_of]. The 32 bit access example below shows this.
///
/// # Basic Example
/// ```
/// use labview_interop::labview_layout;
///
/// labview_layout!(
/// pub struct TestStruct {
///     one: u8,
///     two: u16,
///     three: u32,
/// }
/// );
///
/// ```
///
/// # 32 Bit Reference Access
/// ```
/// use labview_interop::labview_layout;
/// use std::ptr::{addr_of, read_unaligned};
///
/// labview_layout!(
/// pub struct TestStruct {
///     one: u8,
///     two: u16,
///     three: u32,
/// }
/// );
///
/// let value = TestStruct {
///     one: 1,
///     two: 2,
///     three: 3
/// };
///
/// // Not allowed on 32 bit.
/// //let three_ref = &value.three;
/// unsafe {
///     let three_ptr: *const u32 = addr_of!(value.three);
///     let three: u32 = read_unaligned(three_ptr);
/// }
///
/// ```
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
pub struct LVVariant<'variant>(UHandle<'variant, c_void>);

labview_layout!(
    /// Represents the LabVIEW waveform type where:
    ///
    /// * t0: The start time of the data.
    /// * dt: The time delte between samples.
    /// * data: A 1d array of type T
    ///
    /// ## Padding
    ///
    /// The padding scheme here is wierd and unexpected and has been reverse engineered
    /// based on real calls. No idea why the padding exists whether it is documented anywhere.
    pub struct Waveform<'waveform, T> {
        /// The timestamp for the first data value.
        pub t0: timestamp::LVTime,
        /// The time in seconds beween samples.
        pub dt: f64,
        /// A 1D array of the contained data.
        pub data: LVArrayHandle<'waveform, 1, T>,
        #[cfg(target_pointer_width = "64")]
        _pad: u64,
        #[cfg(target_pointer_width = "32")]
        _pad: u32,
        #[cfg(target_pointer_width = "32")]
        _mini_pad: u8,
        attributes: LVVariant<'waveform>,
        #[cfg(target_pointer_width = "64")]
        _pad2: u64,
        #[cfg(target_pointer_width = "32")]
        _pad2: u32,
    }
);
