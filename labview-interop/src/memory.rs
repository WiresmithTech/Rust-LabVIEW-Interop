//! The memory module handles the LabVIEW memory manager
//! functions and types.

/// A pointer from LabVIEW for the data.
pub struct UPtr<T>(*mut T);

/// A handle from LabVIEW for the data.
///
/// A handle is a double pointer so the underlying
/// data can be resized and moved.
pub struct UHandle<T>(*mut *mut T);
