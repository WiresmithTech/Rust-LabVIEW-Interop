//! A module for working with LabVIEW pointers.

use std::marker::PhantomData;
use crate::errors::{InternalError};
use std::ops::{Deref, DerefMut};
use crate::labview::{UPtrValue};

/// A pointer from LabVIEW for the data.
///
/// In general, these should be avoided in favor of `UHandle` which allows
/// for more functionality such as resizing types.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct UPtr<'a, T: ?Sized>(*mut T, PhantomData<&'a T>);

impl<'a, T: Sized> UPtr<'a, T> {
    /// Create a new UPtr from a raw pointer
    /// 
    /// # SAFETY
    /// This should be a pointer that has been allocated by the LV memory manager.
    pub unsafe fn new(ptr: UPtrValue) -> Self {
        Self::from_raw(ptr as *mut T)
    }

    /// Create a new UPtr value from a raw pointer.
    ///
    /// # SAFETY
    /// This must be a pointer allocated by the LabVIEW memory manager.
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self(ptr, PhantomData)
    }
    
    /// Convert to the raw UPtrValue for the APIs
    pub unsafe fn as_uptr_value(&self) -> UPtrValue {
        self.0 as UPtrValue
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
    pub unsafe fn as_ref_mut(&mut self) -> crate::errors::Result<&mut T> {
        self.0.as_mut().ok_or(InternalError::InvalidHandle.into())
    }

}

impl<'a, T: Sized> Deref for UPtr<'a, T> {
    type Target = T;

    /// Extract the target type.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref(&self) -> &Self::Target {
        unsafe { self.as_ref().unwrap() }
    }
}

impl<'a, T: Sized> DerefMut for UPtr<'a, T> {
    /// Deref to a mutable reference.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.as_ref_mut().unwrap() }
    }
}

/// # Safety
/// UPtr memory is managed by the Labview Memory Manager, which is thread safe making Send OK.
unsafe impl<'a, T: ?Sized> Send for UPtr<'a, T> {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptr() {
        let mut data = 42;
        let mut ptr = unsafe { UPtr::from_raw(std::ptr::addr_of_mut!(data)) };
        assert_eq!(*ptr, 42);
        *ptr = 43;
        assert_eq!(*ptr, 43);
    }

    #[test]
    fn test_uptr_as_ref() {
        let mut data = 42;
        let ptr = unsafe { UPtr::from_raw(std::ptr::addr_of_mut!(data))};
        assert_eq!(unsafe { ptr.as_ref() }.unwrap(), &42);
    }

    #[test]
    fn test_uptr_as_ref_mut() {
        let mut data = 42;
        let mut ptr = unsafe { UPtr::from_raw(std::ptr::addr_of_mut!(data))};
        assert_eq!(unsafe { ptr.as_ref_mut() }.unwrap(), &mut 42);
    }

    #[test]
    fn test_uptr_null() {
        let mut ptr: UPtr<i32> = unsafe { UPtr::from_raw(std::ptr::null_mut())};
        assert!(unsafe { ptr.as_ref() }.is_err());
        assert!(unsafe { ptr.as_ref_mut() }.is_err());
    }
}
