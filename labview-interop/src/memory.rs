//! The memory module handles the LabVIEW memory manager
//! functions and types.
//!
//! todo: get to reference without panics.

/// A pointer from LabVIEW for the data.
#[repr(transparent)]
pub struct UPtr<T>(*mut T);

/// A handle from LabVIEW for the data.
///
/// A handle is a double pointer so the underlying
/// data can be resized and moved.
#[repr(transparent)]
pub struct UHandle<T>(*mut *mut T);

impl<T> UHandle<T> {
    /// Get a reference to the internal type.
    ///
    /// # Safety
    /// Will panic or produce undefined behaviour if either pointer
    /// in the handle is invalid.
    pub unsafe fn as_ref(&self) -> &T {
        self.0.as_ref().unwrap().as_ref().unwrap()
    }

    /// Get a mutable reference to the internal type.
    ///
    /// # Safety
    /// Will panic or produce undefined behaviour if either pointer
    /// in the handle is invalid.
    pub unsafe fn as_mut(&self) -> &mut T {
        self.0.as_ref().unwrap().as_mut().unwrap()
    }
}
