//! Handle the various string times that the LabVIEW
//! interface provides.
//!
//!

// Placeholder for now.
// Priority would be to support the LVString type as
// that is what we might find in a cluster.

use crate::labview_layout;
use crate::memory::{UHandle, UPtr};

labview_layout!(
    /// Internal LabVIEW string
    ///
    /// todo: does this follow cluster packing rules? yes but lots breaks.
    pub struct LStr {
        size: i32,
        str: [u8],
    }
);

/// Definition of a handle to an LabVIEW String. Helper for FFI definitin.
pub type LStrHandle = UHandle<LStr>;
/// Definition of a pointer to an LabVIEW String. Helper for FFI definitin.
pub type LStrPtr = UPtr<LStr>;
