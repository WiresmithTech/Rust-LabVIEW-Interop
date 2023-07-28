//! The memory module handles the LabVIEW memory manager
//! functions and types.
//!
//! todo: get to reference without panics.
use std::ops::{Deref, DerefMut};

use crate::errors::{LVInteropError, Result};

/// A pointer from LabVIEW for the data.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct UPtr<T: ?Sized>(*mut T);

/// A handle from LabVIEW for the data.
///
/// A handle is a double pointer so the underlying
/// data can be resized and moved.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct UHandle<T: ?Sized>(pub *mut *mut T);

impl<T: ?Sized> UHandle<T> {
    /// Get a reference to the internal type. Errors if the pointer is null.
    ///
    /// # Safety
    /// This is a wrapper around `pointer::as_ref` and so must follow its safety rules. Namely:
    ///
    ///* When calling this method, you have to ensure that either the pointer is null or all of the following is true:
    ///* The pointer must be properly aligned.
    ///* It must be "dereferenceable" in the sense defined in [the module documentation].
    ///* The pointer must point to an initialized instance of T.
    ///* You must enforce Rust's aliasing rules, since the returned lifetime 'a is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data. In particular, while this reference exists, the memory the pointer points to must not get mutated (except inside UnsafeCell).
    pub unsafe fn as_ref(&self) -> Result<&T> {
        self.0
            .as_ref()
            .map(|ptr| ptr.as_ref())
            .flatten()
            .ok_or(LVInteropError::InvalidHandle)
    }

    /// Get a mutable reference to the internal type. Errors if handle contains a null.
    ///
    /// # Safety
    ///
    /// This method wraps the `pointer::as_mut` method and so follows its safety rules which require all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    /// * It must be “dereferenceable” in the sense defined in the module documentation.
    /// * The pointer must point to an initialized instance of T.
    /// * You must enforce Rust’s aliasing rules, since the returned lifetime 'a is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.
    pub unsafe fn as_ref_mut(&self) -> Result<&mut T> {
        self.0
            .as_ref()
            .map(|ptr| ptr.as_mut())
            .flatten()
            .ok_or(LVInteropError::InvalidHandle)
    }

    /// Check the validity of the handle to ensure it wont panic later.
    pub fn valid(&self) -> bool {
        let inner_ref = unsafe { self.as_ref() };
        inner_ref.is_ok()
    }
}

impl<T: ?Sized> Deref for UHandle<T> {
    type Target = T;

    /// Extract the target type.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref(&self) -> &Self::Target {
        unsafe { self.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for UHandle<T> {
    /// Deref to a mutable reference.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.as_ref_mut().unwrap() }
    }
}

#[cfg(feature = "link")]
impl<T: ?Sized> UHandle<T> {
    /// Resize the handle to the desired size.
    pub unsafe fn resize(&mut self, desired_size: usize) -> Result<()> {
        let err = crate::labview::MEMORY_API.set_handle_size(self.0 as usize, desired_size);
        err.to_result(())
    }
}

/// Magic cookie type used for various reference types in the memory manager.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
#[doc(hidden)]
pub struct MagicCookie(u32);
