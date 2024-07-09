//! The memory module handles the LabVIEW memory manager
//! functions and types.
//!
//! todo: get to reference without panics.
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::errors::{LVInteropError, Result};

/// A pointer from LabVIEW for the data.
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
    pub unsafe fn as_ref(&self) -> Result<&T> {
        self.0.as_ref().ok_or(LVInteropError::InvalidHandle)
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
    pub unsafe fn as_ref_mut(&self) -> Result<&mut T> {
        self.0.as_mut().ok_or(LVInteropError::InvalidHandle)
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

/// A handle from LabVIEW for the data.
///
/// A handle is a double pointer so the underlying
/// data can be resized and moved.
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug)]
pub struct UHandle<'a, T: ?Sized>(pub *mut *mut T, PhantomData<&'a ()>);

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
    pub unsafe fn as_ref(&self) -> Result<&T> {
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
    pub unsafe fn as_ref_mut(&self) -> Result<&mut T> {
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

//TODO test
#[cfg(feature = "link")]
impl<'a, T> Borrow<UHandle<'a, T>> for LvOwned<T::Owned>
where
    T: ToOwned + ?Sized + 'a,
    T::Owned: 'static,
{
    fn borrow(&self) -> &UHandle<'a, T> {
        // Unsafe transmute to convert the lifetime
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "link")]
impl<'a, T: ?Sized> UHandle<'a, T> {
    /// Resize the handle to the desired size.
    ///
    /// # Safety
    ///
    /// * The handle must be valid.
    pub unsafe fn resize(&mut self, desired_size: usize) -> Result<()> {
        let err = crate::labview::memory_api()?.set_handle_size(self.0 as usize, desired_size);
        err.to_result(())
    }

    /// Copy the contents of one handle into another.
    ///
    /// If other points to a null value then this will allocate a handle for the contents.
    ///
    /// # Safety
    ///
    /// * If the other pointer is invalid this may cause UB.
    /// * If the other pointer points to null, you must wrap the value as an owned handle otherwise it will leak memory.
    // pub unsafe fn clone_into_pointer2(&self, other: *mut Self) -> Result<()> {
    //     let error = crate::labview::memory_api()?.copy_handle(other as *mut usize, self.0 as usize);
    //     error.to_result(())
    // }
    pub unsafe fn clone_into_pointer(&self, other: *mut UHandle<'_, T>) -> Result<()> {
        let error = crate::labview::memory_api()?.copy_handle(other as *mut usize, self.0 as usize);
        error.to_result(())
    }
}

/// # Safety
///
/// * UHandle memory is managed by the Labview Memory Manager, which is thread safe
unsafe impl<'a, T: ?Sized> Send for UHandle<'a, T> {}
unsafe impl<'a, T: ?Sized> Sync for UHandle<'a, T> {}

#[cfg(feature = "link")]
mod lv_owned {
    use std::borrow::ToOwned;
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};

    use super::UHandle;
    use crate::errors::{LVInteropError, Result};
    use crate::labview::memory_api;

    /// A value allocated in the LabVIEW memory managed by us.
    ///
    /// This will manage the lifetime and free the handle on drop.
    ///
    /// This is a semantic difference from handle and is transparent with the handle data.
    ///
    /// This means it can be used in structs in place of a handle.
    ///
    /// # Example In Struct (LStrOwned is equivalent of `LvOwned<LStr>`).
    /// ```no_run
    ///# use labview_interop::labview_layout;
    ///# use labview_interop::types::LStrOwned;
    ///labview_layout!(
    ///pub struct UserEventCluster {
    ///    eventno: i32,
    ///    id: LStrOwned,
    ///}
    ///);
    ///```
    #[repr(transparent)]
    pub struct LvOwned<T: ?Sized>(pub(crate) UHandle<'static, T>);

    impl<T: Sized> LvOwned<T> {
        /// Create a new handle to a sized value of `T`.
        pub fn new() -> Result<Self> {
            let handle = unsafe { memory_api()?.new_handle(std::mem::size_of::<T>()) };
            if handle.is_null() {
                Err(LVInteropError::HandleCreationFailed)
            } else {
                Ok(Self(UHandle(handle as *mut *mut T, PhantomData)))
            }
        }
    }

    impl<T: ?Sized> LvOwned<T> {
        /// Create a new handle to the type `T`. It will create an empty handle
        /// which you must initialise with the `init_routine`.
        /// This is useful for unsized types.
        ///
        /// # Safety
        ///
        /// * This will create a handle to un-initialized memory. The provided initialisation
        ///    routine must prepare the memory.
        pub(crate) unsafe fn new_unsized(
            init_routine: impl FnOnce(&mut UHandle<'_, T>) -> Result<()>,
        ) -> Result<Self> {
            let handle = memory_api()?.new_handle(0);
            if handle.is_null() {
                Err(LVInteropError::HandleCreationFailed)
            } else {
                let mut new_value = Self(UHandle(handle as *mut *mut T, PhantomData));
                init_routine(&mut new_value)?;
                Ok(new_value)
            }
        }

        /// TODO test
        /// Return the UHandle to the owned memory
        ///
        /// # Safety
        ///
        /// * This needs to take a mutable reference to self and lifetime annotation on UHandle,
        ///    in order to avoid creating multiple UHandles.
        pub fn handle(&mut self) -> UHandle<'static, T> {
            UHandle(self.0 .0, PhantomData)
        }

        pub fn from_handle(handle: UHandle<'static, T>) -> Self {
            Self(handle)
        }
    }

    /// TODO
    /// potentially expensive operation
    #[cfg(feature = "link")]
    impl<T> Clone for LvOwned<T>
    where
        T: ?Sized + ToOwned,
        T::Owned: 'static,
    {
        fn clone(&self) -> Self {
            let mut cloned_handle = UHandle(std::ptr::null_mut(), PhantomData);
            unsafe {
                self.0.clone_into_pointer(&mut cloned_handle).unwrap();
            }
            LvOwned::from_handle(cloned_handle)
        }
    }

    impl<T: ?Sized> Deref for LvOwned<T> {
        type Target = UHandle<'static, T>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: ?Sized> DerefMut for LvOwned<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T: ?Sized> Drop for LvOwned<T> {
        fn drop(&mut self) {
            let result = memory_api()
                .map(|api| unsafe { api.dispose_handle(self.0 .0 as usize).to_result(()) });
            if let Err(e) | Ok(Err(e)) = result {
                println!("Error freeing handle from LV: {e}");
            }
        }
    }

    //TODO test
    #[cfg(feature = "link")]
    impl<'a, T> ToOwned for UHandle<'a, T>
    where
        T: ToOwned + ?Sized + 'a,
        T::Owned: 'static,
    {
        type Owned = LvOwned<T::Owned>;

        fn to_owned(&self) -> Self::Owned {
            // calling clone_into_pointer with a nullpointer returns a new Handle
            let mut owned_handle = UHandle(std::ptr::null_mut(), PhantomData);
            unsafe {
                self.clone_into_pointer(&mut owned_handle).unwrap();
            };

            LvOwned(owned_handle)
        }
    }
}

/// # Safety
///
/// * LvOwned memory is access through UHandle which is managed by the Labview Memory Manager, which is thread safe
unsafe impl<'a, T: ?Sized> Send for LvOwned<T> {}
unsafe impl<'a, T: ?Sized> Sync for LvOwned<T> {}

#[cfg(feature = "link")]
pub use lv_owned::LvOwned;

/// Magic cookie type used for various reference types in the memory manager.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
#[doc(hidden)]
pub struct MagicCookie(u32);

// test
// 1. LvOwned.clone()
// * Clone simple LvOwned
// * Clone struct also containing LvOwned / UHandle
// 2. UHandle.to_owned()
// 3. Send / Sync
