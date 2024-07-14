
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use super::LvCopy;
use crate::errors::LVInteropError;

/// A handle from LabVIEW for the data.
///
/// A handle is a double pointer so the underlying
/// data can be resized and moved.
///
/// `Deref` is implemented for this type so you can access the data using `*`
/// but this can panic if the handle is invalid.
///
/// ```
/// # use labview_interop::memory::UHandle;
/// fn print_value(handle: UHandle<i32>) {
///   println!("{}", *handle);
/// }
/// ```
///
/// If you want to handle the error you can use the `UHandle::as_ref` or `UHandle::as_ref_mut` method.
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct UHandle<'a, T: ?Sized + 'a>(pub *mut *mut T, pub PhantomData<&'a T>);

impl<'a, T: ?Sized> UHandle<'a, T> {
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
        self.0
            .as_ref()
            .and_then(|ptr| ptr.as_ref())
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
    pub unsafe fn as_ref_mut(&self) -> crate::errors::Result<&mut T> {
        self.0
            .as_ref()
            .and_then(|ptr| ptr.as_mut())
            .ok_or(LVInteropError::InvalidHandle)
    }

    /// Check the validity of the handle to ensure it wont panic later.
    ///
    /// A valid handle is:
    ///
    /// . Not Null.
    /// . Points to a pointer.
    /// . That pointer is in the LabVIEW memory zone.
    ///
    /// The last 2 checks are done by LabVIEW and require the `link` feature.
    ///
    /// If the `link` feature is not enabled then we just check it is not null.
    ///
    /// # Panics/Safety
    ///
    /// This will cause a segfault if the handle doesn't point to a valid address.
    pub fn valid(&self) -> bool {
        // check if is not NULL
        let inner_ref = unsafe { self.as_ref() };

        // # Safety
        //
        // Make sure we don't call the following function on an invalid pointer
        if inner_ref.is_err() {
            return false;
        }
        // Only call the API in the link feature.
        #[cfg(feature = "link")]
        {
            // check if the memory manager actually knows about the handle if it is not null
            let ret = unsafe {
                crate::labview::memory_api()
                    .unwrap()
                    .check_handle(self.0 as usize)
            };
            ret == crate::errors::MgErr::NO_ERROR
        }
        #[cfg(not(feature = "link"))]
        {
            return true;
        }
    }
}

impl<'a, T: ?Sized> Deref for UHandle<'a, T> {
    type Target = T;

    /// Extract the target type.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref(&self) -> &Self::Target {
        unsafe { self.as_ref().unwrap() }
    }
}

impl<'a, T: ?Sized> DerefMut for UHandle<'a, T> {
    /// Deref to a mutable reference.
    ///
    /// This will panic if the handle or internal pointer is null.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.as_ref_mut().unwrap() }
    }
}

impl<'a, T: Debug> Debug for UHandle<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::fmt_handle("UHandle", self, f)
    }
}

#[cfg(feature = "link")]
impl<'a, T: ?Sized> UHandle<'a, T> {
    /// Resize the handle to the desired size.
    ///
    /// # Safety
    ///
    /// * The handle must be valid.
    pub unsafe fn resize(&mut self, desired_size: usize) -> crate::errors::Result<()> {
        let err = crate::labview::memory_api()?.set_handle_size(self.0 as usize, desired_size);
        err.to_result(())
    }
}

#[cfg(feature = "link")]
impl<'a, T: ?Sized + LvCopy + 'static> UHandle<'a, T> {
    /// Copy the contents of one handle into another.
    ///
    /// If other points to a null value then this will allocate a handle for the contents.
    ///
    /// The data in the handle must be `'static` or copy as this will only perform a shallow copy.
    ///
    /// # Safety
    ///
    /// * If the other pointer is invalid this may cause UB.
    /// * If the other pointer points to null, you must wrap the value as an owned handle otherwise it will leak memory.
    ///
    /// # Examples
    ///
    /// ## Allowed Types
    /// ```no_run
    /// use labview_interop::labview_layout;
    /// use labview_interop::memory::{UHandle, LvOwned};
    /// use labview_interop::types::LStrHandle;
    ///
    /// labview_layout! {
    ///   #[derive(Copy, Clone)]
    ///   struct ClusterWithNumbers {
    ///     float: f64,
    ///     int: i32
    ///   }
    /// }
    ///
    /// fn copy_handles(input: UHandle<ClusterWithNumbers>) {
    ///   let cluster = ClusterWithNumbers { float: 3.14, int: 42 };
    ///   let mut new_owned = LvOwned::new(&cluster).unwrap();
    ///   unsafe {
    ///     let mut target_handle = new_owned.handle_to_inner();
    ///     input.clone_into_pointer(&mut target_handle).unwrap();
    ///   }
    /// }
    /// ```
    ///
    /// ## Lifetime Guarantees - Fails with Sub-Handles
    /// ```compile_fail,E0521
    /// use labview_interop::labview_layout;
    /// use labview_interop::memory::{UHandle, LvOwned};
    /// use labview_interop::types::LStrHandle;
    ///
    /// labview_layout! {
    ///   struct ClusterWithString<'a> {
    ///     string_handle: LStrHandle<'a>,
    ///     int: i32
    ///   }
    /// }
    ///
    /// fn copy_handles(input: UHandle<ClusterWithString>) {
    ///   let mut new_owned = LvOwned::<ClusterWithString>::new().unwrap();
    ///   unsafe {
    ///     let mut target_handle = new_owned.handle_to_inner();
    ///     input.clone_into_pointer(&mut target_handle).unwrap();
    ///   }
    /// }
    /// ```
    /// ## Lifetime Guarantees - Fails with Owned Handle
    /// ```compile_fail
    /// use labview_interop::labview_layout;
    /// use labview_interop::memory::{UHandle, LvOwned};
    /// use labview_interop::types::LStrOwned;
    ///
    /// labview_layout! {
    ///   struct ClusterWithString {
    ///     string_handle: LStrOwned,
    ///     int: i32
    ///   }
    /// }
    ///
    /// fn copy_handles(input: UHandle<ClusterWithString>, mut output: UHandle<ClusterWithString>) {
    ///   unsafe {
    ///     input.clone_into_pointer(&mut output).unwrap();
    ///   }
    /// }
    /// ```
    pub unsafe fn clone_into_pointer(&self, other: *mut UHandle<'_, T>) -> crate::errors::Result<()> {
        let error = crate::labview::memory_api()?.copy_handle(other as *mut usize, self.0 as usize);
        error.to_result(())
    }
}

/// # Safety
///
/// * UHandle memory is managed by the Labview Memory Manager, which is thread safe
unsafe impl<'a, T: ?Sized> Send for UHandle<'a, T> {}
unsafe impl<'a, T: ?Sized> Sync for UHandle<'a, T> {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_debug() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let handle = UHandle(std::ptr::addr_of_mut!(value_ptr), PhantomData);
        assert_eq!(format!("{:?}", handle), "UHandle(42)");
    }

    #[test]
    fn test_invalid_handle_debug() {
        let handle = UHandle(std::ptr::null_mut::<*mut i32>(), PhantomData);
        assert_eq!(format!("{:?}", handle), "UHandle(Invalid)");
    }

    #[test]
    fn test_handle_debug_inner_from_reference() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let handle = UHandle(std::ptr::addr_of_mut!(value_ptr), PhantomData);
        assert_eq!(format!("{:?}", *handle), "42");
    }

    #[test]
    fn test_handle_deref() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let handle = UHandle(std::ptr::addr_of_mut!(value_ptr), PhantomData);
        assert_eq!(*handle, 42);
    }

    #[test]
    fn test_handle_deref_mut() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let mut handle = UHandle(std::ptr::addr_of_mut!(value_ptr), PhantomData);
        *handle = 43;
        assert_eq!(*handle, 43);
    }

    #[test]
    fn handle_as_ref_valid() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let handle = UHandle(std::ptr::addr_of_mut!(value_ptr), PhantomData);
        assert_eq!(unsafe { handle.as_ref() }.unwrap(), &42);
    }

    #[test]
    fn handle_as_ref_outer_null() {
        let handle = UHandle(std::ptr::null_mut::<*mut i32>(), PhantomData);
        assert!(unsafe { handle.as_ref() }.is_err());
    }

    #[test]
    fn handle_as_ref_inner_null() {
        let mut inner_ptr = std::ptr::null_mut::<i32>();
        let handle = UHandle(std::ptr::addr_of_mut!(inner_ptr), PhantomData);
        assert!(unsafe { handle.as_ref() }.is_err());
    }

    #[test]
    fn handle_as_ref_mut_valid() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let handle = UHandle(std::ptr::addr_of_mut!(value_ptr), PhantomData);
        assert_eq!(unsafe { handle.as_ref_mut() }.unwrap(), &mut 42);
    }

    #[test]
    fn handle_as_ref_mut_outer_null() {
        let handle = UHandle(std::ptr::null_mut::<*mut i32>(), PhantomData);
        assert!(unsafe { handle.as_ref_mut() }.is_err());
    }

    #[test]
    fn handle_as_ref_mut_inner_null() {
        let mut inner_ptr = std::ptr::null_mut::<i32>();
        let handle = UHandle(std::ptr::addr_of_mut!(inner_ptr), PhantomData);
        assert!(unsafe { handle.as_ref_mut() }.is_err());
    }

    #[test]
    fn handle_valid_check_false_if_null() {
        let handle = UHandle(std::ptr::null_mut::<*mut i32>(), PhantomData);
        assert!(!handle.valid());
    }

    #[cfg(not(feature = "link"))]
    #[test]
    fn handle_valid_check_is_valid_no_link() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let handle = UHandle(std::ptr::addr_of_mut!(value_ptr), PhantomData);
        assert!(handle.valid());
    }

}

