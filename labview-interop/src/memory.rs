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
pub struct UHandle<T>(pub *mut *mut T);

impl<T> UHandle<T> {
    /// Get a reference to the internal type.
    /// # Safety
    /// This is a wrapper around [`std::ptr::as_ref`] and so must follow its safety rules. Namely:
    ///
    ///* When calling this method, you have to ensure that either the pointer is null or all of the following is true:
    ///* The pointer must be properly aligned.
    ///* It must be "dereferenceable" in the sense defined in [the module documentation].
    ///* The pointer must point to an initialized instance of T.
    ///* You must enforce Rust's aliasing rules, since the returned lifetime 'a is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data. In particular, while this reference exists, the memory the pointer points to must not get mutated (except inside UnsafeCell).
    pub unsafe fn as_ref(&self) -> Option<&T> {
        self.0.as_ref().map(|ptr| ptr.as_ref()).flatten()
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
