//! A module for working with LabVIEW pointers.

use crate::errors::InternalError;
use std::ops::{Deref, DerefMut};

/// A pointer from LabVIEW for the data.
///
/// In general, these should be avoided in favor of `UHandle` which allows
/// for more functionality such as resizing types.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct UPtr<T: ?Sized>(*mut T);

impl<T: ?Sized> UPtr<T> {
    /// Create a new UPtr from a raw pointer
    pub fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }
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
    pub unsafe fn as_ref(&self) -> crate::errors::Result<&T> {
        self.0.as_ref().ok_or(InternalError::InvalidHandle.into())
    }

    /// Get a mutable reference to the internal type. Errors if pointer contains a null.
    ///
    /// # Safety
    ///
    /// This method wraps the `pointer::as_mut` method and so follows its safety rules which require all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    /// * It must be “dereferenceable” in the sense defined in the module documentation.
    /// * The pointer must point to an initialized instance of T.
    /// * You must enforce Rust’s aliasing rules, since the returned lifetime 'a is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.
    pub unsafe fn as_ref_mut(&self) -> crate::errors::Result<&mut T> {
        self.0.as_mut().ok_or(InternalError::InvalidHandle.into())
    }
}

impl<T: ?Sized> Deref for UPtr<T> {
    type Target = T;

    /// Extract the target type.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref(&self) -> &Self::Target {
        unsafe { self.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for UPtr<T> {
    /// Deref to a mutable reference.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.as_ref_mut().unwrap() }
    }
}

/// # Safety
///
/// * UPtr memory is managed by the Labview Memory Manager, which is thread safe
unsafe impl<'a, T: ?Sized> Send for UPtr<T> {}
unsafe impl<'a, T: ?Sized> Sync for UPtr<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptr() {
        let mut data = 42;
        let mut ptr = UPtr(std::ptr::addr_of_mut!(data));
        assert_eq!(*ptr, 42);
        *ptr = 43;
        assert_eq!(*ptr, 43);
    }

    #[test]
    fn test_uptr_as_ref() {
        let mut data = 42;
        let ptr = UPtr(std::ptr::addr_of_mut!(data));
        assert_eq!(unsafe { ptr.as_ref() }.unwrap(), &42);
    }

    #[test]
    fn test_uptr_as_ref_mut() {
        let mut data = 42;
        let ptr = UPtr(std::ptr::addr_of_mut!(data));
        assert_eq!(unsafe { ptr.as_ref_mut() }.unwrap(), &mut 42);
    }

    #[test]
    fn test_uptr_null() {
        let ptr: UPtr<i32> = UPtr(std::ptr::null_mut());
        assert!(unsafe { ptr.as_ref() }.is_err());
        assert!(unsafe { ptr.as_ref_mut() }.is_err());
    }
}
